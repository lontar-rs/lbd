#!/usr/bin/env python3
"""
Scrape Balai Bahasa Bali Kamus from kamusbahasaprovinsibali.kemdikbud.go.id
Output: CSV with lemma_latin, pos, def_indonesian, domain
"""
import csv
import time
import sys
from urllib.parse import urljoin

import requests
from bs4 import BeautifulSoup

BASE_URL = "https://balaibahasaprovinsibali.kemendikdasmen.go.id"
SEARCH_URL = f"{BASE_URL}/search"

def scrape_entry(lemma):
    """Fetch entry page for a given lemma."""
    try:
        resp = requests.get(SEARCH_URL, params={"q": lemma}, timeout=10)
        resp.raise_for_status()
        soup = BeautifulSoup(resp.text, "html.parser")

        # Parse entry structure (adjust selectors based on actual site structure)
        # This is a placeholder - actual selectors need inspection
        entry_div = soup.find("div", class_="entry")
        if not entry_div:
            return None

        pos = entry_div.find("span", class_="pos")
        definition = entry_div.find("div", class_="definition")

        return {
            "lemma_latin": lemma,
            "pos": pos.text.strip() if pos else None,
            "def_indonesian": definition.text.strip() if definition else None,
            "def_english": None,
            "register": None,
            "domain": "general"
        }
    except Exception as e:
        print(f"Error fetching {lemma}: {e}", file=sys.stderr)
        return None

def scrape_all_lemmas(output_file):
    """
    Scrape all lemmas from the dictionary.
    Strategy: start with common Balinese words, then crawl alphabetically.
    """
    # Seed with high-frequency Balinese words
    seed_lemmas = [
        "apa", "adi", "ajak", "anak", "bapa", "beli", "cai", "dadi", "dija",
        "gedé", "icang", "ida", "jani", "kéto", "lakar", "maan", "napi", "né",
        "niki", "pidan", "raga", "saking", "tiang", "uli", "wénten"
    ]

    with open(output_file, "w", newline="", encoding="utf-8") as f:
        writer = csv.DictWriter(f, fieldnames=["lemma_latin", "pos", "def_indonesian", "def_english", "register", "domain"])
        writer.writeheader()

        for lemma in seed_lemmas:
            entry = scrape_entry(lemma)
            if entry:
                writer.writerow(entry)
            time.sleep(0.5)  # Rate limiting

if __name__ == "__main__":
    output = sys.argv[1] if len(sys.argv) > 1 else "data/balai_bahasa_scraped.csv"
    print(f"Scraping to {output}...")
    scrape_all_lemmas(output)
    print("Done.")
