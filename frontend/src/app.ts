import './app.css';

// Type definitions for API responses
export interface Entry {
	id: string;
	lemma_latin: string;
	lemma_aksara?: string;
	pos: string;
	register: string;
	senses: Sense[];
	etymology?: Etymology;
	cross_references?: CrossReference[];
}

export interface Sense {
	id: string;
	definition_indonesian: string;
	definition_english: string;
	domain: string;
	order: number;
	attestations?: Attestation[];
}

export interface Attestation {
	id: string;
	source: string;
	date_range?: string;
	quote_latin: string;
	quote_translation: string;
	confidence: number;
}

export interface Etymology {
	chain: EtymologyStep[];
}

export interface EtymologyStep {
	language: string;
	form: string;
	meaning?: string;
}

export interface CrossReference {
	type: 'synonym' | 'antonym' | 'see_also';
	lemma: string;
}

export interface SearchResult {
	id: string;
	lemma_latin: string;
	register: string;
	first_sense: string;
	domain: string;
}

export interface SearchResponse {
	hits: SearchResult[];
	estimated_total_hits: number;
	processing_time_ms: number;
	query: string;
}

// Language types for i18n
export type Language = 'en' | 'id';

// UI translation interface
export interface Translation {
	search: {
		placeholder: string;
		button: string;
		no_results: string;
	};
	entry: {
		part_of_speech: string;
		register: string;
		domain: string;
		definition: string;
		etymology: string;
		cross_references: string;
		cite: string;
	};
	navigation: {
		home: string;
		browse: string;
		about: string;
		search: string;
	};
}

// Language configurations
export const translations: Record<Language, Translation> = {
	en: {
		search: {
			placeholder: "Search Balinese dictionary...",
			button: "Search",
			no_results: "No results found. Try a different search term."
		},
		entry: {
			part_of_speech: "Part of Speech",
			register: "Register",
			domain: "Domain",
			definition: "Definition",
			etymology: "Etymology",
			cross_references: "Cross References",
			cite: "Cite this Entry"
		},
		navigation: {
			home: "Home",
			browse: "Browse",
			about: "About",
			search: "Search"
		}
	},
	id: {
		search: {
			placeholder: "Cari kamus Bali...",
			button: "Cari",
			no_results: "Tidak ada hasil. Coba kata kunci lain."
		},
		entry: {
			part_of_speech: "Jenis Kata",
			register: "Register",
			domain: "Domain",
			definition: "Definisi",
			etymology: "Etimologi",
			cross_references: "Referensi Silang",
			cite: "Kutip Entri Ini"
		},
		navigation: {
			home: "Beranda",
			browse: "Jelajahi",
			about: "Tentang",
			search: "Cari"
		}
	}
};
