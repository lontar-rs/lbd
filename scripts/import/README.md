# Import Scripts

Placeholder directory for seed-data imports. Expected CSV schema:

```
lemma_latin,pos,def_indonesian,def_english,register,domain
padem,verb,"meninggal dunia (untuk orang terhormat)","to die (honorific)",alus_singgih,ritual
```

Run (example):
```
cargo run --bin import -- --file data/balai_bahasa_sample.csv --source balai_bahasa --database-url $DATABASE_URL
```
