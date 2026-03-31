#!/bin/bash

# Script to index published entries in Meilisearch

echo "Fetching published entries from database..."

# Get published entries as JSON
PUBLISHED_ENTRIES=$(PGPASSWORD=lbd_app_password_20260317 psql -h localhost -U lbd_app -d lbd -t -c "
SELECT json_agg(json_build_object(
  'id', e.id,
  'lemma_latin', e.lemma_latin,
  'register', COALESCE(r.level, 'umum'),
  'first_sense', COALESCE(s.def_indonesian, ''),
  'first_sense_en', COALESCE(s.def_english, ''),
  'domain', COALESCE(s.domain, 'umum')
)) FROM entry e
LEFT JOIN sense s ON e.id = s.entry_id
LEFT JOIN entry_register r ON e.id = r.entry_id
WHERE e.status = 'published'
")

echo "Indexing entries in Meilisearch..."

# Send to Meilisearch
curl -X POST 'http://localhost:7700/indexes/entries/documents' \
  -H 'Content-Type: application/json' \
  -H 'Authorization: Bearer masterKey' \
  --data-binary "$PUBLISHED_ENTRIES"

echo ""
echo "Indexing complete!"
