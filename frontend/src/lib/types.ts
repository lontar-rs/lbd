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
	first_sense_en?: string;
	domain: string;
}

export interface SearchResponse {
	hits: SearchResult[];
	estimated_total_hits: number;
	processing_time_ms: number;
	query: string;
}

// Corpus-related types
export interface Corpus {
	id: string;
	title: string;
	type: string;
	date_range?: string;
	location_held: string;
	digitization_status: 'not_digitized' | 'partial' | 'complete';
	source_url?: string;
	institution?: string;
	description?: string;
}

export interface CorpusAttestation {
	id: string;
	entry_lemma: string;
	quote_latin: string;
	quote_translation: string;
	page_number?: string;
	confidence: number;
}
