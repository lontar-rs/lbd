import { writable } from 'svelte/store';
import type { Language } from '$lib/translations';

// Language preference store
export const currentLanguage = writable<Language>('en');

// UI theme store (for future dark mode support)
export const theme = writable<'light' | 'dark'>('light');

// Search state store
export const searchQuery = writable<string>('');
export const searchResults = writable<any[]>([]);
export const searchLoading = writable<boolean>(false);
export const searchError = writable<string | null>(null);

// Entry view state
export const currentEntry = writable<any>(null);
export const entryLoading = writable<boolean>(false);
export const entryError = writable<string | null>(null);

// UI state
export const mobileMenuOpen = writable<boolean>(false);
export const searchLanguage = writable<'bali' | 'id' | 'en'>('bali');
