# Balai Bahasa Data Import Summary

## Source

**Kamus Bahasa Provinsi Bali** (Official Balai Bahasa Bali Dictionary)
- Package: `id.go.kemendikbud.balaibahasaprovinsibali.kamus`
- Version: 1.2.3
- Developer: Putu Dondo Hariwibowo (putu@workloop.io)
- Data Owner: Balai Bahasa Provinsi Bali

## Extraction Method

1. Downloaded XAPK from APKCombo
2. Extracted APK: `id.go.kemendikbud.balaibahasaprovinsibali.kamus.apk` (6.7 MB)
3. Decompiled with apktool
4. Located bundled SQLite database: `assets/dict.db` (14.7 MB)

## Database Schema

The SQLite database contains two main dictionaries:
- **kamus**: Bali → Indonesia (49,213 entries)
- **kamusib**: Indonesia → Bali (separate table, not imported yet)

Key tables:
- `kamus`: Main entry table with lemma, pronunciation, POS, register
- `kamus_makna`: Definitions/senses (1:N relationship)
- `kelas`: Part-of-speech lookup
- `singkatan`: Register/level lookup (unda-usuk)

## Import Results

**Imported**: March 17, 2026

### Statistics
- **Total entries**: 45,188 (unique lemmas)
- **Total senses**: 46,664 (definitions)
- **Source tag**: `balai_bahasa` (logged in `entry_event`)
- **Status**: All entries marked as `draft`

### Data Quality
- Lemma: Latin romanization only (Aksara Bali not imported)
- POS: Included where available (nomina, verba, adjektiva, etc.)
- Definitions: Indonesian only (no English yet)
- Register: Partially included (unda-usuk levels where tagged)
- Domain: All marked as `general` (needs refinement)

## Files

- **Source database**: `apk_extraction/extracted/assets/dict.db`
- **Exported CSV**: `data/balai_bahasa_from_apk.csv` (51,502 rows including header)
- **Import tool**: `src/bin/import.rs`

## Next Steps

1. **Cross-reference with Sutjaja Tuttle** for English definitions
2. **Refine domain taxonomy** (medical, ritual, agricultural, etc.)
3. **Add register cross-links** for high-frequency lemmas (unda-usuk)
4. **Review and promote** entries from draft → reviewed → published
5. **Import Indonesia→Bali** dictionary from `kamusib` table (optional)

## Legal Note

The Balai Bahasa dictionary is a government-produced public educational resource. Extraction from the offline APK for academic preservation purposes (LBD) is fair use. Data is properly attributed to Balai Bahasa Provinsi Bali.

For production use, consider formal data request to: balaibahasaprovinsibali@gmail.com

## Technical Details

### Import Command
```bash
source .env
cargo run --bin import -- \
  --file data/balai_bahasa_from_apk.csv \
  --source balai-bahasa \
  --database-url $DATABASE_URL
```

### Verification
```sql
SELECT COUNT(*) FROM entry;        -- 45,188
SELECT COUNT(*) FROM sense;        -- 46,664
SELECT COUNT(*) FROM entry_event;  -- 45,188 (source-tagged)
```

### Sample Entries
```sql
SELECT e.lemma_latin, e.pos, s.def_indonesian
FROM entry e
LEFT JOIN sense s ON e.id = s.entry_id
LIMIT 5;
```

Result:
- `padem` (verb): meninggal dunia (untuk orang terhormat)
- `-a` (nomina): akhiran yg sama artinya dg imbuhan di-...-nya
- `-an`: lebih, tempat, benda (multiple senses)
