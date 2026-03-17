-- Initial schema for LBD

CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE corpus (
    id UUID PRIMARY KEY,
    title TEXT NOT NULL,
    type TEXT NOT NULL,
    date_min INTEGER,
    date_max INTEGER,
    date_cert TEXT,
    period TEXT NOT NULL,
    script TEXT NOT NULL,
    location TEXT,
    call_number TEXT,
    dig_status TEXT NOT NULL,
    license TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE entry (
    id UUID PRIMARY KEY,
    lemma_latin TEXT NOT NULL UNIQUE,
    lemma_aksara TEXT,
    ipa TEXT,
    pos TEXT,
    root UUID REFERENCES entry(id),
    status TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE entry_register (
    entry_id UUID REFERENCES entry(id),
    level TEXT NOT NULL,
    dialect TEXT,
    equivalent_id UUID REFERENCES entry(id),
    PRIMARY KEY (entry_id, level)
);

CREATE TABLE etymology (
    id UUID PRIMARY KEY,
    entry_id UUID REFERENCES entry(id),
    proto_austronesian TEXT,
    proto_mp TEXT,
    sanskrit TEXT,
    kawi TEXT,
    old_balinese TEXT,
    loan_source TEXT,
    loan_form TEXT,
    notes TEXT,
    confidence TEXT NOT NULL
);

CREATE TABLE sense (
    id UUID PRIMARY KEY,
    entry_id UUID REFERENCES entry(id),
    sense_order INTEGER NOT NULL,
    def_balinese TEXT,
    def_indonesian TEXT NOT NULL,
    def_english TEXT,
    domain TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE attestation (
    id UUID PRIMARY KEY,
    sense_id UUID REFERENCES sense(id),
    corpus_id UUID REFERENCES corpus(id),
    quote_aksara TEXT,
    quote_latin TEXT,
    quote_trans_id TEXT,
    quote_trans_en TEXT,
    confidence NUMERIC(3,2),
    source_rank INTEGER NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE cross_ref (
    entry_id UUID REFERENCES entry(id),
    ref_entry_id UUID REFERENCES entry(id),
    type TEXT NOT NULL
);

CREATE TABLE entry_event (
    id UUID PRIMARY KEY,
    entry_id UUID REFERENCES entry(id),
    editor_id UUID,
    event_type TEXT,
    diff JSONB,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
