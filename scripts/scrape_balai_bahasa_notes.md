# Balai Bahasa Scraping Notes

## Site Inspection Required

The scraper script is a **template** that requires actual site inspection to work. Before running:

1. Visit https://balaibahasaprovinsibali.kemendikdasmen.go.id
2. Inspect the HTML structure:
   - Search for a test word (e.g., "apa")
   - Open browser DevTools → Elements
   - Identify CSS selectors for:
     - Entry container (e.g., `.entry`, `.result-item`)
     - Lemma/headword
     - Part of speech tag
     - Definition text
     - Domain/category (if present)

3. Update `scrape_balai_bahasa.py` selectors:
   ```python
   entry_div = soup.find("div", class_="ACTUAL_CLASS_NAME")
   pos = entry_div.find("span", class_="ACTUAL_POS_CLASS")
   definition = entry_div.find("div", class_="ACTUAL_DEF_CLASS")
   ```

4. Test with a single word first:
   ```bash
   python3 scripts/scrape_balai_bahasa.py data/test_scrape.csv
   ```

## Alternative: APK Extraction

If scraping is blocked or site structure is complex:

1. Download APK from APKMirror or device:
   ```bash
   # From device
   adb pull /data/app/id.go.kemendikbud.balaibahasaprovinsibali.kamus-*/base.apk

   # Or download from APKMirror
   wget https://www.apkmirror.com/...balaibahasaprovinsibali.apk
   ```

2. Extract with apktool:
   ```bash
   apktool d balaibahasaprovinsibali.apk
   find balaibahasaprovinsibali -name "*.db" -o -name "*.sqlite"
   ```

3. If SQLite found, dump to CSV:
   ```bash
   sqlite3 kamus.db ".mode csv" ".output data/balai_bahasa_from_apk.csv" "SELECT * FROM entries;"
   ```

## Legal Note

Balai Bahasa Bali is a government institution. The dictionary is a public educational resource. Scraping for research/preservation purposes falls under fair use, but:
- Rate limit requests (0.5-1s delay)
- Attribute source in imported data
- Consider formal data request for production use
