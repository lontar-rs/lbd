import type { Entry, SearchResponse } from '$lib/types';

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

		const response = await fetch(`${this.baseUrl}/entries/search?${params}`);
		if (!response.ok) {
			throw new Error(`Search failed: ${response.statusText}`);
		}

		return response.json();
	}

	async getEntry(lemma: string): Promise<Entry> {
		const response = await fetch(`${this.baseUrl}/entries/${encodeURIComponent(lemma)}`);
		if (!response.ok) {
			if (response.status === 404) {
				throw new Error('Entry not found');
			}
			throw new Error(`Failed to fetch entry: ${response.statusText}`);
		}

		return response.json();
	}

	async getEntryAttestations(id: string) {
		const response = await fetch(`${this.baseUrl}/entries/${id}/attestations`);
		if (!response.ok) {
			throw new Error(`Failed to fetch attestations: ${response.statusText}`);
		}

		return response.json();
	}

	async getCorpus(id: string) {
		const response = await fetch(`${this.baseUrl}/corpus/${id}`);
		if (!response.ok) {
			throw new Error(`Failed to fetch corpus: ${response.statusText}`);
		}

		return response.json();
	}
}

// Singleton instance
export const api = new ApiClient();
