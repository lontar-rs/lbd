-- Sample seed data for development
INSERT INTO entry (id, lemma_latin, pos, status)
VALUES ('11111111-1111-1111-1111-111111111111', 'padem', 'verb', 'draft')
ON CONFLICT DO NOTHING;

INSERT INTO sense (id, entry_id, sense_order, def_indonesian, def_english, domain)
VALUES ('22222222-2222-2222-2222-222222222222', '11111111-1111-1111-1111-111111111111', 1,
        'meninggal dunia (untuk orang terhormat)', 'to die (honorific register)', 'ritual')
ON CONFLICT DO NOTHING;
