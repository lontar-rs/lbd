-- Domain taxonomy and source authority lookup tables

CREATE TABLE domain_taxonomy (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    slug TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    description TEXT
);

CREATE TABLE source_authority (
    code TEXT PRIMARY KEY,
    rank INTEGER NOT NULL UNIQUE,
    label TEXT NOT NULL,
    notes TEXT
);

INSERT INTO domain_taxonomy (slug, name, description) VALUES
    ('general', 'General', 'General vocabulary'),
    ('medical', 'Medical', 'Health, medicine, healing'),
    ('ritual', 'Ritual', 'Ritual and ceremonial language'),
    ('agricultural', 'Agricultural', 'Farming, irrigation, crops'),
    ('legal', 'Legal', 'Law, governance'),
    ('literary', 'Literary', 'Literary and poetic usage'),
    ('botanical', 'Botanical', 'Plants and flora')
ON CONFLICT (slug) DO NOTHING;

INSERT INTO source_authority (code, rank, label, notes) VALUES
    ('prasasti', 1, 'Prasasti', 'Datable, primary, archaeological'),
    ('van_der_tuuk', 2, 'Van der Tuuk', 'Philologically rigorous, 19th c. Dutch'),
    ('zoetmulder', 3, 'Zoetmulder', 'Kawi layer authority'),
    ('balai_bahasa', 4, 'Balai Bahasa', 'Modern official'),
    ('sutjaja', 5, 'Sutjaja', 'Modern scholarly'),
    ('community', 6, 'Community', 'Flagged, requires review')
ON CONFLICT (code) DO NOTHING;
