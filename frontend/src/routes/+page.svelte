<script lang="ts">
	import { onMount } from 'svelte';
	import { searchQuery, searchResults, searchLoading, searchError, searchLanguage, currentLanguage } from '$lib/stores';
	import { api } from '$lib/api';
	import { translations, type Language } from '$lib/translations';

	let query = '';
	let debouncedQuery = '';
	let searchTimeout: ReturnType<typeof setTimeout>;

	$: currentLanguage;
	$: t = translations[currentLanguage] || translations.en;

	// Debounced search
	$: if (debouncedQuery) {
		performSearch();
	}

	function performSearch() {
		searchLoading.set(true);
		searchError.set(null);

		api.searchEntries(debouncedQuery, $searchLanguage)
			.then(results => {
				searchResults.set(results.hits);
			})
			.catch(error => {
				searchError.set(error.message);
				searchResults.set([]);
			})
			.finally(() => {
				searchLoading.set(false);
			});
	}

	function handleInput(event: Event) {
		const target = event.target as HTMLInputElement;
		query = target.value;

		// Clear existing timeout
		if (searchTimeout) {
			clearTimeout(searchTimeout);
		}

		// Set new timeout for debounced search
		searchTimeout = setTimeout(() => {
			debouncedQuery = query;
		}, 300);
	}

	function handleSubmit(event: Event) {
		event.preventDefault();
		debouncedQuery = query;
	}

	onMount(() => {
		// Restore saved search language
		const saved = localStorage.getItem('searchLanguage');
		if (saved && ['bali', 'id', 'en'].includes(saved)) {
			searchLanguage.set(saved as 'bali' | 'id' | 'en');
		}
	});

	$: if ($searchLanguage && typeof localStorage !== 'undefined') {
		localStorage.setItem('searchLanguage', $searchLanguage);
	}
</script>

<svelte:head>
	<title>Lontar Balinese Dictionary - Search</title>
</svelte:head>

<div class="container">
	<section class="welcome-section">
		<div class="welcome">
			<h1>Welcome to LBD</h1>
			<p>Comprehensive Balinese-Indonesian-English dictionary</p>
			<div class="welcome-actions">
				<a href="/browse" class="btn btn-secondary">Browse Dictionary</a>
				<a href="/about" class="btn btn-secondary">About LBD</a>
			</div>
		</div>
	</section>

	<section class="search-section">
		<h2>{t.navigation?.search}</h2>

		<form class="search-form" on:submit={handleSubmit}>
			<div class="search-input-group">
				<input
					type="text"
					bind:value={query}
					on:input={handleInput}
					placeholder={t.search?.placeholder}
					class="search-input"
				/>
				<button type="submit" class="btn btn-primary" disabled={$searchLoading}>
					{$searchLoading ? '...' : t.search?.button}
				</button>
			</div>

			<div class="search-languages">
				<label>
					<input
						type="radio"
						bind:group={$searchLanguage}
						value="bali"
					/>
					Bali
				</label>
				<label>
					<input
						type="radio"
						bind:group={$searchLanguage}
						value="id"
					/>
					Indonesia
				</label>
				<label>
					<input
						type="radio"
						bind:group={$searchLanguage}
						value="en"
					/>
					English
				</label>
			</div>
		</form>

		{#if $searchError}
			<div class="error-message">
				{$searchError}
			</div>
		{/if}
	</section>

	{#if $searchLoading}
		<section class="results-section">
			<div class="loading">Searching...</div>
		</section>
	{:else if $searchResults.length > 0}
		<section class="results-section">
			<div class="results-header">
				<h2>Results ({$searchResults.length})</h2>
			</div>

			<div class="results-list">
				{#each $searchResults as result}
					<a href="/entry/{result.lemma_latin}" class="result-card">
						<div class="result-header">
							<h3 class="result-lemma">
								{result.lemma_latin}
								{#if result.lemma_latin !== result.lemma_latin.toUpperCase()}
									<span class="bali-text">{result.lemma_latin}</span>
								{/if}
							</h3>
							<div class="result-meta">
								<span class="badge badge-register">{result.register}</span>
								<span class="badge badge-domain">{result.domain}</span>
							</div>
						</div>
						<div class="result-definition">
							{result.first_sense}
						</div>
					</a>
				{/each}
			</div>
		</section>
	{:else if query && !$searchLoading}
		<section class="results-section">
			<div class="no-results">
				<p>{t.search?.no_results}</p>
			</div>
		</section>
	{/if}
</div>

<style>
	.welcome-section {
		text-align: center;
		margin-bottom: 3rem;
	}

	.search-section {
		text-align: center;
		margin-bottom: 3rem;
	}

	.welcome h1 {
		margin-bottom: 1rem;
		font-size: 2.5rem;
		color: var(--color-primary);
	}

	.welcome p {
		color: var(--text-secondary);
		font-size: 1.25rem;
		margin-bottom: 2rem;
	}

	.search-form {
		max-width: 600px;
		margin: 0 auto;
	}

	.search-input-group {
		display: flex;
		gap: 0.5rem;
		margin-bottom: 1rem;
	}

	.search-input {
		flex: 1;
		padding: 0.75rem 1rem;
		border: 2px solid var(--border);
		border-radius: 0.5rem;
		font-size: 1rem;
		transition: border-color 0.2s;
	}

	.search-input:focus {
		outline: none;
		border-color: var(--color-primary);
	}

	.search-languages {
		display: flex;
		justify-content: center;
		gap: 1rem;
	}

	.search-languages label {
		display: flex;
		align-items: center;
		gap: 0.25rem;
		cursor: pointer;
	}

	.error-message {
		background: #fef2f2;
		color: #dc2626;
		padding: 1rem;
		border-radius: 0.5rem;
		margin-top: 1rem;
		text-align: left;
	}

	.results-section {
		max-width: 800px;
		margin: 0 auto;
	}

	.results-header {
		margin-bottom: 1.5rem;
	}

	.results-list {
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}

	.result-card {
		display: block;
		text-decoration: none;
		color: inherit;
		padding: 1.5rem;
		background: var(--surface);
		border: 1px solid var(--border);
		border-radius: 0.5rem;
		transition: all 0.2s;
	}

	.result-card:hover {
		border-color: var(--color-primary);
		box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
	}

	.result-header {
		display: flex;
		justify-content: space-between;
		align-items: flex-start;
		margin-bottom: 0.75rem;
		gap: 1rem;
	}

	.result-lemma {
		margin: 0;
		font-size: 1.25rem;
		font-weight: 600;
		color: var(--color-primary);
	}

	.result-meta {
		display: flex;
		gap: 0.5rem;
		flex-wrap: wrap;
	}

	.result-definition {
		color: var(--text-secondary);
		line-height: 1.5;
	}

	.loading, .no-results {
		text-align: center;
		padding: 3rem 1rem;
	}

	.welcome-actions {
		display: flex;
		justify-content: center;
		gap: 1rem;
		margin-top: 2rem;
		flex-wrap: wrap;
	}

	@media (max-width: 768px) {
		.search-input-group {
			flex-direction: column;
		}

		.result-header {
			flex-direction: column;
			align-items: flex-start;
		}

		.welcome-actions {
			flex-direction: column;
			align-items: center;
		}
	}
</style>
