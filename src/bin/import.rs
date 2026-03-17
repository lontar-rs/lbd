use std::{fs::File, path::PathBuf};

use anyhow::{Context, Result};
use clap::{Parser, ValueEnum};
use csv::ReaderBuilder;
use serde::Deserialize;
use sqlx::{postgres::PgPoolOptions, PgPool};
use uuid::Uuid;

#[derive(Debug, Parser)]
#[command(author, version, about = "Import seed data into LBD", long_about = None)]
struct Args {
    /// Path to CSV file to import
    #[arg(long)]
    file: PathBuf,

    /// Source tag (balai_bahasa | sutjaja)
    #[arg(long, value_enum)]
    source: Source,

    /// Database URL (falls back to env DATABASE_URL)
    #[arg(long, env)]
    database_url: String,
}

#[derive(Debug, Clone, ValueEnum)]
enum Source {
    BalaiBahasa,
    Sutjaja,
}

#[derive(Debug, Deserialize)]
struct Record {
    lemma_latin: String,
    pos: Option<String>,
    def_indonesian: Option<String>,
    def_english: Option<String>,
    register: Option<String>,
    domain: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&args.database_url)
        .await
        .context("connect database")?;

    import_file(&pool, &args.file, args.source).await?;
    Ok(())
}

async fn import_file(pool: &PgPool, path: &PathBuf, source: Source) -> Result<()> {
    let file = File::open(path).with_context(|| format!("open file {}", path.display()))?;
    let mut rdr = ReaderBuilder::new().flexible(true).from_reader(file);

    for rec in rdr.deserialize::<Record>() {
        let rec = rec?;
        let entry_id = Uuid::new_v4();
        let sense_id = Uuid::new_v4();

        // Insert entry (skip on conflict)
        sqlx::query!(
            r#"
            INSERT INTO entry (id, lemma_latin, pos, status)
            VALUES ($1, $2, $3, 'draft')
            ON CONFLICT (lemma_latin) DO NOTHING
            "#,
            entry_id,
            rec.lemma_latin.trim(),
            rec.pos
        )
        .execute(pool)
        .await
        .with_context(|| format!("insert entry {}", rec.lemma_latin))?;

        // Fetch existing entry id if already present
        let stored_entry_id = sqlx::query_scalar!(
            "SELECT id FROM entry WHERE lemma_latin = $1",
            rec.lemma_latin.trim()
        )
        .fetch_one(pool)
        .await?;

        // Insert sense if we have any definition text
        if rec.def_indonesian.is_some() || rec.def_english.is_some() {
            sqlx::query!(
                r#"
                INSERT INTO sense (id, entry_id, sense_order, def_indonesian, def_english, domain)
                VALUES ($1, $2, 1, $3, $4, $5)
                ON CONFLICT DO NOTHING
                "#,
                sense_id,
                stored_entry_id,
                rec.def_indonesian,
                rec.def_english,
                rec.domain
            )
            .execute(pool)
            .await
            .with_context(|| format!("insert sense for {}", rec.lemma_latin))?;
        }

        // Register linking (placeholder: just store register level)
        if let Some(level) = rec.register {
            sqlx::query!(
                r#"
                INSERT INTO entry_register (entry_id, level)
                VALUES ($1, $2)
                ON CONFLICT DO NOTHING
                "#,
                stored_entry_id,
                level
            )
            .execute(pool)
            .await
            .with_context(|| format!("insert register for {}", rec.lemma_latin))?;
        }

        // Event log (source tagging)
        let source_str = source_label(&source);
        sqlx::query!(
            r#"
            INSERT INTO entry_event (id, entry_id, event_type, diff)
            VALUES ($1, $2, 'created', jsonb_build_object('source', $3::text))
            ON CONFLICT DO NOTHING
            "#,
            Uuid::new_v4(),
            stored_entry_id,
            source_str
        )
        .execute(pool)
        .await
        .with_context(|| format!("log event for {}", rec.lemma_latin))?;
    }

    Ok(())
}

fn source_label(src: &Source) -> &'static str {
    match src {
        Source::BalaiBahasa => "balai_bahasa",
        Source::Sutjaja => "sutjaja",
    }
}
