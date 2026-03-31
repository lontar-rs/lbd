<script lang="ts">
	import { onMount } from 'svelte';
	import { api } from '$lib/api';
	import { currentLanguage } from '$lib/stores';
	import { translations, type Language } from '$lib/translations';
	import LoadingSkeleton from '$lib/components/LoadingSkeleton.svelte';

	$: currentLanguage;
	$: t = translations[currentLanguage] || translations.en;

	let entries: any[] = [];
	let loading: boolean = true;
	let error: string | null = null;
	let abortController: AbortController | null = null;
	let browseMode: 'alphabetical' | 'domain' | 'register' | 'etymology' = 'alphabetical';
	let selectedLetter: string = 'A';
	let selectedDomain: string = '';
	let selectedRegister: string = '';
	let selectedEtymology: string = '';

	const letters = 'ABCDEFGHIJKLMNOPQRSTUVWXYZ'.split('');
	const domains = ['general', 'medical', 'ritual', 'agricultural', 'legal', 'literary', 'botanical'];
	const registers = ['alus singgih', 'alus sor', 'alus mider', 'andap', 'kasar'];
	const etymologyOrigins = ['Sanskrit', 'Kawi', 'Austronesian', 'Dutch', 'Portuguese', 'Arabic'];

	onMount(async () => {
		await loadEntries();
	});

	async function loadEntries() {
		// Cancel any ongoing request
		if (abortController) {
			abortController.abort();
		}

		abortController = new AbortController();
		loading = true;
		error = null;

		try {
			let query = '';
			let filter = '';

			switch (browseMode) {
				case 'alphabetical':
					query = selectedLetter;
					filter = `lemma_latin:${selectedLetter}*`;
					break;
				case 'domain':
					query = selectedDomain;
					filter = `domain:${selectedDomain}`;
					break;
				case 'register':
					query = selectedRegister;
					filter = `register:"${selectedRegister}"`;
					break;
				case 'etymology':
					query = selectedEtymology;
					filter = `etymology_origin:${selectedEtymology}`;
					break;
			}

			const results = await api.searchEntries(query, 'bali', filter);
			// Only update if request wasn't aborted
			if (!abortController.signal.aborted) {
				entries = results.hits;
			}
		} catch (err) {
			// Don't show error if request was aborted
			if (!abortController.signal.aborted) {
				error = err instanceof Error ? err.message : 'Failed to load entries';
				entries = [];
			}
		} finally {
			if (!abortController.signal.aborted) {
				loading = false;
			}
			abortController = null;
		}
	}

	function selectLetter(letter: string) {
		selectedLetter = letter;
		loadEntries();
	}

	function selectDomain(domain: string) {
		selectedDomain = domain;
		loadEntries();
	}

	function selectRegister(register: string) {
		selectedRegister = register;
		loadEntries();
	}

	function selectEtymology(origin: string) {
		selectedEtymology = origin;
		loadEntries();
	}

	function setBrowseMode(mode: 'alphabetical' | 'domain' | 'register' | 'etymology') {
		browseMode = mode;
		loadEntries();
	}
</script>

<svelte:head>
	<title>Browse Dictionary - Lontar Balinese Dictionary</title>
</svelte:head>

<div class="container">
	<header class="browse-header">
		<h1>{t.navigation?.browse || 'Browse Dictionary'}</h1>
		<p>Browse entries by different categories</p>
	</header>

	<nav class="browse-modes">
		<div class="mode-buttons">
			<button
				class="mode-btn"
				class:active={browseMode === 'alphabetical'}
				on:click={() => setBrowseMode('alphabetical')}
			>
				Alphabetical
			</button>
			<button
				class="mode-btn"
				class:active={browseMode === 'domain'}
				on:click={() => setBrowseMode('domain')}
			>
				By Domain
			</button>
			<button
				class="mode-btn"
				class:active={browseMode === 'register'}
				on:click={() => setBrowseMode('register')}
			>
				By Register
			</button>
			<button
				class="mode-btn"
				class:active={browseMode === 'etymology'}
				on:click={() => setBrowseMode('etymology')}
			>
				By Etymology
			</button>
		</div>
	</nav>

	<nav class="browse-nav">
		{#if browseMode === 'alphabetical'}
			<div class="letter-buttons">
				{#each letters as letter}
					<button
						class="letter-btn"
						class:active={selectedLetter === letter}
						on:click={() => selectLetter(letter)}
					>
						{letter}
					</button>
				{/each}
			</div>
		{:else if browseMode === 'domain'}
			<div class="category-buttons">
				{#each domains as domain}
					<button
						class="category-btn"
						class:active={selectedDomain === domain}
						on:click={() => selectDomain(domain)}
					>
						{domain}
					</button>
				{/each}
			</div>
		{:else if browseMode === 'register'}
			<div class="category-buttons">
				{#each registers as register}
					<button
						class="category-btn"
						class:active={selectedRegister === register}
						on:click={() => selectRegister(register)}
					>
						{register}
					</button>
				{/each}
			</div>
		{:else if browseMode === 'etymology'}
			<div class="category-buttons">
				{#each etymologyOrigins as origin}
					<button
						class="category-btn"
						class:active={selectedEtymology === origin}
						on:click={() => selectEtymology(origin)}
					>
						{origin}
					</button>
				{/each}
			</div>
		{/if}
	</nav>

	<main class="browse-content">
		{#if loading && entries.length === 0}
			<LoadingSkeleton count={6} type="entry" />
		{:else if loading}
			<div class="loading">Loading entries...</div>
		{:else if error}
			<div class="error">
				<p>{error}</p>
			</div>
		{:else if entries.length === 0}
			<div class="no-entries">
				<p>No entries found for this selection.</p>
			</div>
		{:else}
			<div class="entries-grid">
				{#each entries as entry}
					<a href="/entry/{entry.lemma_latin}" class="entry-card">
						<div class="entry-lemma">
							{entry.lemma_latin}
							{#if entry.lemma_aksara}
								<span class="bali-text">{entry.lemma_aksara}</span>
							{/if}
						</div>
						<div class="entry-meta">
							<span class="badge badge-register">{entry.register}</span>
							<span class="badge badge-domain">{entry.domain}</span>
						</div>
						<div class="entry-preview">
							{entry.first_sense}
						</div>
					</a>
				{/each}
			</div>
		{/if}
	</main>
</div>

<style>
	.browse-header {
		text-align: center;
		margin-bottom: 2rem;
	}

	.browse-header p {
		color: var(--text-secondary);
		font-size: 1.125rem;
	}

	.browse-modes {
		margin-bottom: 2rem;
	}

	.mode-buttons {
		display: flex;
		justify-content: center;
		flex-wrap: wrap;
		gap: 0.5rem;
		max-width: 800px;
		margin: 0 auto;
	}

	.mode-btn {
		padding: 0.75rem 1rem;
		border: 1px solid var(--border);
		background: white;
		border-radius: 0.5rem;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.2s;
	}

	.mode-btn:hover {
		border-color: var(--color-primary);
		background: var(--surface);
	}

	.mode-btn.active {
		background: var(--color-primary);
		color: white;
		border-color: var(--color-primary);
	}

	.browse-nav {
		margin-bottom: 2rem;
	}

	.letter-buttons, .category-buttons {
		display: flex;
		justify-content: center;
		flex-wrap: wrap;
		gap: 0.5rem;
		max-width: 800px;
		margin: 0 auto;
	}

	.letter-btn {
		padding: 0.5rem 0.75rem;
		border: 1px solid var(--border);
		background: white;
		border-radius: 0.375rem;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s;
		min-width: 3rem;
	}

	.letter-btn:hover {
		border-color: var(--color-primary);
		background: var(--surface);
	}

	.letter-btn.active {
		background: var(--color-primary);
		color: white;
		border-color: var(--color-primary);
	}

	.category-btn {
		padding: 0.5rem 1rem;
		border: 1px solid var(--border);
		background: white;
		border-radius: 0.375rem;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.2s;
	}

	.category-btn:hover {
		border-color: var(--color-primary);
		background: var(--surface);
	}

	.category-btn.active {
		background: var(--color-primary);
		color: white;
		border-color: var(--color-primary);
	}

	.browse-content {
		max-width: 1000px;
		margin: 0 auto;
	}

	.loading, .error, .no-entries {
		text-align: center;
		padding: 3rem 1rem;
	}

	.entries-grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
		gap: 1rem;
	}

	.entry-card {
		display: block;
		text-decoration: none;
		color: inherit;
		padding: 1.5rem;
		background: var(--surface);
		border: 1px solid var(--border);
		border-radius: 0.5rem;
		transition: all 0.2s;
	}

	.entry-card:hover {
		border-color: var(--color-primary);
		box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
		transform: translateY(-1px);
	}

	.entry-lemma {
		font-size: 1.25rem;
		font-weight: 600;
		color: var(--color-primary);
		margin-bottom: 0.5rem;
		display: flex;
		flex-direction: column;
	}

	.entry-lemma .bali-text {
		font-size: 1rem;
		color: var(--text-primary);
		margin-top: 0.25rem;
	}

	.entry-meta {
		display: flex;
		gap: 0.5rem;
		margin-bottom: 0.75rem;
		flex-wrap: wrap;
	}

	.entry-preview {
		color: var(--text-secondary);
		line-height: 1.5;
		font-size: 0.875rem;
	}

	@media (max-width: 768px) {
		.mode-buttons {
			gap: 0.25rem;
		}

		.mode-btn {
			padding: 0.5rem 0.75rem;
			font-size: 0.875rem;
		}

		.letter-buttons, .category-buttons {
			gap: 0.25rem;
		}

		.letter-btn {
			padding: 0.375rem 0.5rem;
			min-width: 2.5rem;
			font-size: 0.875rem;
		}

		.category-btn {
			padding: 0.375rem 0.75rem;
			font-size: 0.875rem;
		}

		.entries-grid {
			grid-template-columns: 1fr;
		}
	}
</style>
