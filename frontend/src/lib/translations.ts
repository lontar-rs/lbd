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
	corpus: {
		title: string;
		type: string;
		date_range: string;
		location_held: string;
		digitization_status: string;
		attestations: string;
		source_image: string;
		not_digitized: string;
		partial_digitization: string;
		complete_digitization: string;
		view_entry: string;
		no_attestations: string;
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
		corpus: {
			title: "Title",
			type: "Type",
			date_range: "Date Range",
			location_held: "Location Held",
			digitization_status: "Digitization Status",
			attestations: "Attestations",
			source_image: "Source Image",
			not_digitized: "Not Digitized",
			partial_digitization: "Partially Digitized",
			complete_digitization: "Complete",
			view_entry: "View Entry",
			no_attestations: "No attestations found for this corpus."
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
		corpus: {
			title: "Judul",
			type: "Tipe",
			date_range: "Rentang Tanggal",
			location_held: "Lokasi Disimpan",
			digitization_status: "Status Digitalisasi",
			attestations: "Atestasi",
			source_image: "Gambar Sumber",
			not_digitized: "Belum Didigitalkan",
			partial_digitization: "Sebagian Didigitalkan",
			complete_digitization: "Lengkap",
			view_entry: "Lihat Entri",
			no_attestations: "Tidak ada attestasi yang ditemukan untuk korpus ini."
		},
		navigation: {
			home: "Beranda",
			browse: "Jelajahi",
			about: "Tentang",
			search: "Cari"
		}
	}
};
