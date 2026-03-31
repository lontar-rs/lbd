<script lang="ts">
	import { onMount } from 'svelte';
	import { page } from '$app/stores';
	import { api } from '$lib/api';
	import { currentLanguage } from '$lib/stores';
	import { translations, type Language } from '$lib/translations';
	import type { Corpus, CorpusAttestation } from '$lib/types';
	import LoadingSkeleton from '$lib/components/LoadingSkeleton.svelte';

	$: currentLanguage;
	$: t = translations[currentLanguage] || translations.en;
	$: corpusId = $page.params.id;

	let corpus: Corpus | null = null;
	let attestations: CorpusAttestation[] = [];
	let loading: boolean = true;
	let error: string | null = null;
	let abortController: AbortController | null = null;

	onMount(async () => {
		await loadCorpus();
	});

	async function loadCorpus() {
		// Cancel any ongoing request
		if (abortController) {
			abortController.abort();
		}

		abortController = new AbortController();
		loading = true;
		error = null;

		try {
			// Load corpus details
			corpus = await api.getCorpus(corpusId);

			// Load attestations for this corpus
			attestations = await api.getCorpusAttestations(corpusId);
		} catch (err) {
			// Don't show error if request was aborted
			if (!abortController.signal.aborted) {
				error = err instanceof Error ? err.message : 'Failed to load corpus';
				corpus = null;
				attestations = [];
			}
		} finally {
			if (!abortController.signal.aborted) {
				loading = false;
			}
			abortController = null;
		}
	}

	function getDigitizationStatusText(status: string) {
		switch (status) {
			case 'not_digitized':
				return t.corpus?.not_digitized || 'Not Digitized';
			case 'partial':
				return t.corpus?.partial_digitization || 'Partially Digitized';
			case 'complete':
				return t.corpus?.complete_digitization || 'Complete';
			default:
				return status;
		}
	}

	function getDigitizationStatusClass(status: string) {
		switch (status) {
			case 'not_digitized':
				return 'status-not-digitized';
			case 'partial':
				return 'status-partial';
			case 'complete':
				return 'status-complete';
			default:
				return '';
		}
	}
</script>

<svelte:head>
	<title>{corpus?.title || 'Corpus'} - Lontar Balinese Dictionary</title>
</svelte:head>

<div class="container">
	{#if loading}
		<div class="loading">
			<p>Loading corpus information...</p>
		</div>
	{:else if error}
		<div class="error">
			<p>{error}</p>
		</div>
	{:else if corpus}
		<header class="corpus-header">
			<nav class="breadcrumb">
				<a href="/">{t.navigation?.home || 'Home'}</a>
				<span class="separator">/</span>
				<a href="/browse">{t.navigation?.browse || 'Browse'}</a>
				<span class="separator">/</span>
				<span class="current">Corpus</span>
			</nav>

			<h1>{corpus.title}</h1>
			{#if corpus.description}
				<p class="corpus-description">{corpus.description}</p>
			{/if}
		</header>

		<main class="corpus-content">
			<section class="corpus-metadata">
				<h2>{t.corpus?.title || 'Manuscript Information'}</h2>

				<div class="metadata-grid">
					<div class="metadata-item">
						<h3>{t.corpus?.type || 'Type'}</h3>
						<p>{corpus.type}</p>
					</div>

					{#if corpus.date_range}
						<div class="metadata-item">
							<h3>{t.corpus?.date_range || 'Date Range'}</h3>
							<p>{corpus.date_range}</p>
						</div>
					{/if}

					<div class="metadata-item">
						<h3>{t.corpus?.location_held || 'Location Held'}</h3>
						<p>{corpus.location_held}</p>
					</div>

					{#if corpus.institution}
						<div class="metadata-item">
							<h3>Institution</h3>
							<p>{corpus.institution}</p>
						</div>
					{/if}

					<div class="metadata-item">
						<h3>{t.corpus?.digitization_status || 'Digitization Status'}</h3>
						<p class="digitization-status {getDigitizationStatusClass(corpus.digitization_status)}">
							{getDigitizationStatusText(corpus.digitization_status)}
						</p>
					</div>
				</div>

				{#if corpus.source_url}
					<div class="source-link">
						<h3>{t.corpus?.source_image || 'Source Image'}</h3>
						<a href={corpus.source_url} target="_blank" rel="noopener noreferrer" class="btn btn-primary">
							View Source Image
						</a>
					</div>
				{/if}
			</section>

			<section class="corpus-attestations">
				<h2>{t.corpus?.attestations || 'Attestations'} ({attestations.length})</h2>

				{#if loading && attestations.length === 0}
					<LoadingSkeleton count={3} type="corpus" />
				{:else if attestations.length === 0}
					<div class="no-attestations">
						<p>{t.corpus?.no_attestations || 'No attestations found for this corpus.'}</p>
					</div>
				{:else}
					<div class="attestations-list">
						{#each attestations as attestation}
							<div class="attestation-card">
								<div class="attestation-header">
									<h3>
										<a href="/entry/{attestation.entry_lemma}" class="entry-link">
											{attestation.entry_lemma}
										</a>
									</h3>
									{#if attestation.page_number}
										<span class="page-number">Page {attestation.page_number}</span>
									{/if}
								</div>

								<div class="attestation-content">
									<div class="quote">
										<p class="quote-latin">{attestation.quote_latin}</p>
										<p class="quote-translation">{attestation.quote_translation}</p>
									</div>

									<div class="attestation-footer">
										<a href="/entry/{attestation.entry_lemma}" class="btn btn-secondary">
											{t.corpus?.view_entry || 'View Entry'}
										</a>
									</div>
								</div>
							</div>
						{/each}
					</div>
				{/if}
			</section>
		</main>
	{:else}
		<div class="not-found">
			<h2>Corpus Not Found</h2>
			<p>The corpus with ID "{corpusId}" could not be found.</p>
			<a href="/browse" class="btn btn-primary">Browse Dictionary</a>
		</div>
	{/if}
</div>

<style>
	.corpus-header {
		margin-bottom: 3rem;
	}

	.breadcrumb {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		margin-bottom: 2rem;
		font-size: 0.875rem;
		color: var(--text-secondary);
	}

	.breadcrumb a {
		color: var(--color-primary);
		text-decoration: none;
	}

	.breadcrumb a:hover {
		text-decoration: underline;
	}

	.breadcrumb .separator {
		color: var(--text-secondary);
	}

	.breadcrumb .current {
		color: var(--text-primary);
	}

	.corpus-header h1 {
		margin-bottom: 1rem;
		font-size: 2.5rem;
		color: var(--color-primary);
	}

	.corpus-description {
		color: var(--text-secondary);
		font-size: 1.125rem;
		line-height: 1.6;
	}

	.corpus-content {
		max-width: 1000px;
		margin: 0 auto;
	}

	.corpus-metadata {
		margin-bottom: 3rem;
		padding: 2rem;
		background: var(--surface);
		border-radius: 0.5rem;
		border: 1px solid var(--border);
	}

	.corpus-metadata h2 {
		margin-bottom: 1.5rem;
		color: var(--text-primary);
	}

	.metadata-grid {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
		gap: 2rem;
		margin-bottom: 2rem;
	}

	.metadata-item h3 {
		margin-bottom: 0.5rem;
		font-size: 0.875rem;
		font-weight: 600;
		color: var(--text-secondary);
		text-transform: uppercase;
		letter-spacing: 0.05em;
	}

	.metadata-item p {
		font-size: 1rem;
		color: var(--text-primary);
		font-weight: 500;
	}

	.digitization-status {
		padding: 0.25rem 0.75rem;
		border-radius: 1rem;
		font-size: 0.875rem;
		font-weight: 500;
		display: inline-block;
	}

	.status-not-digitized {
		background: #fef2f2;
		color: #dc2626;
	}

	.status-partial {
		background: #fef3c7;
		color: #d97706;
	}

	.status-complete {
		background: #f0fdf4;
		color: #16a34a;
	}

	.source-link {
		padding-top: 1.5rem;
		border-top: 1px solid var(--border);
	}

	.source-link h3 {
		margin-bottom: 1rem;
		font-size: 1rem;
		font-weight: 600;
		color: var(--text-secondary);
	}

	.corpus-attestations h2 {
		margin-bottom: 2rem;
		color: var(--text-primary);
	}

	.no-attestations {
		text-align: center;
		padding: 3rem 1rem;
		color: var(--text-secondary);
	}

	.attestations-list {
		display: flex;
		flex-direction: column;
		gap: 1.5rem;
	}

	.attestation-card {
		background: var(--surface);
		border: 1px solid var(--border);
		border-radius: 0.5rem;
		padding: 1.5rem;
		transition: all 0.2s;
	}

	.attestation-card:hover {
		border-color: var(--color-primary);
		box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
	}

	.attestation-header {
		display: flex;
		justify-content: space-between;
		align-items: flex-start;
		margin-bottom: 1rem;
		gap: 1rem;
	}

	.attestation-header h3 {
		margin: 0;
		font-size: 1.25rem;
	}

	.entry-link {
		color: var(--color-primary);
		text-decoration: none;
		font-weight: 600;
	}

	.entry-link:hover {
		text-decoration: underline;
	}

	.page-number {
		background: var(--color-secondary);
		color: white;
		padding: 0.25rem 0.5rem;
		border-radius: 0.25rem;
		font-size: 0.75rem;
		font-weight: 500;
	}

	.quote {
		margin-bottom: 1rem;
	}

	.quote-latin {
		font-style: italic;
		margin-bottom: 0.5rem;
		color: var(--text-primary);
	}

	.quote-translation {
		color: var(--text-secondary);
		font-size: 0.875rem;
	}

	.attestation-footer {
		text-align: right;
	}

	.loading, .error, .not-found {
		text-align: center;
		padding: 3rem 1rem;
	}

	.error {
		color: #dc2626;
	}

	.not-found h2 {
		margin-bottom: 1rem;
		color: var(--text-primary);
	}

	.not-found p {
		margin-bottom: 2rem;
		color: var(--text-secondary);
	}

	@media (max-width: 768px) {
		.corpus-header h1 {
			font-size: 2rem;
		}

		.metadata-grid {
			grid-template-columns: 1fr;
			gap: 1.5rem;
		}

		.attestation-header {
			flex-direction: column;
			align-items: flex-start;
		}

		.attestation-footer {
			text-align: left;
		}
	}
</style>
