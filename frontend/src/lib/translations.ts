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
