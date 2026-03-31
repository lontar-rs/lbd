import type { Entry, SearchResponse, Corpus, CorpusAttestation } from '$lib/types';

const API_BASE = '/api';

export class ApiClient {
	private baseUrl: string;

	constructor(baseUrl: string = API_BASE) {
		this.baseUrl = baseUrl;
	}

	async searchEntries(query: string, lang?: string, filter?: string): Promise<SearchResponse> {
		const params = new URLSearchParams({
			q: query,
			...(lang && { lang }),
			...(filter && { filter })
		});

		try {
			const response = await fetch(`${this.baseUrl}/entries/search?${params}`);
			if (!response.ok) {
				throw new Error(`Search failed: ${response.statusText}`);
			}
			return response.json();
		} catch (error) {
			if (error instanceof Error) {
				// Re-throw HTTP errors as-is
				if (error.message.includes('Search failed:')) {
					throw error;
				}
				// Handle network errors
				if (error.name === 'TypeError' && error.message.includes('fetch')) {
					throw new Error('Network error: Unable to connect to server. Please check your internet connection.');
				}
				// Handle timeout errors
				if (error.name === 'AbortError') {
					throw new Error('Request timeout: The search request took too long.');
				}
			}
			throw new Error('Search failed: An unexpected error occurred.');
		}
	}

	async getEntry(lemma: string): Promise<Entry> {
		try {
			const response = await fetch(`${this.baseUrl}/entries/${encodeURIComponent(lemma)}`);
			if (!response.ok) {
				if (response.status === 404) {
					throw new Error('Entry not found');
				}
				throw new Error(`Failed to fetch entry: ${response.statusText}`);
			}
			return response.json();
		} catch (error) {
			if (error instanceof Error) {
				// Re-throw known errors as-is
				if (error.message.includes('Entry not found') || error.message.includes('Failed to fetch entry:')) {
					throw error;
				}
				// Handle network errors
				if (error.name === 'TypeError' && error.message.includes('fetch')) {
					throw new Error('Network error: Unable to connect to server. Please check your internet connection.');
				}
			}
			throw new Error('Failed to load entry: An unexpected error occurred.');
		}
	}

	async getEntryAttestations(id: string) {
		const response = await fetch(`${this.baseUrl}/entries/${id}/attestations`);
		if (!response.ok) {
			throw new Error(`Failed to fetch attestations: ${response.statusText}`);
		}

		return response.json();
	}

	async getCorpus(id: string): Promise<Corpus> {
		try {
			const response = await fetch(`${this.baseUrl}/corpus/${id}`);
			if (!response.ok) {
				if (response.status === 404) {
					throw new Error('Corpus not found');
				}
				throw new Error(`Failed to fetch corpus: ${response.statusText}`);
			}
			return response.json();
		} catch (error) {
			if (error instanceof Error) {
				// Re-throw known errors as-is
				if (error.message.includes('Corpus not found') || error.message.includes('Failed to fetch corpus:')) {
					throw error;
				}
				// Handle network errors
				if (error.name === 'TypeError' && error.message.includes('fetch')) {
					throw new Error('Network error: Unable to connect to server. Please check your internet connection.');
				}
			}
			throw new Error('Failed to load corpus: An unexpected error occurred.');
		}
	}

	async getCorpusAttestations(id: string): Promise<CorpusAttestation[]> {
		try {
			const response = await fetch(`${this.baseUrl}/corpus/${id}/attestations`);
			if (!response.ok) {
				throw new Error(`Failed to fetch corpus attestations: ${response.statusText}`);
			}
			return response.json();
		} catch (error) {
			if (error instanceof Error) {
				// Re-throw known errors as-is
				if (error.message.includes('Failed to fetch corpus attestations:')) {
					throw error;
				}
				// Handle network errors
				if (error.name === 'TypeError' && error.message.includes('fetch')) {
					throw new Error('Network error: Unable to connect to server. Please check your internet connection.');
				}
			}
			throw new Error('Failed to load attestations: An unexpected error occurred.');
		}
	}
}

// Singleton instance
export const api = new ApiClient();
