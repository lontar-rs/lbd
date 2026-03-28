<script lang="ts">
	import { onMount } from 'svelte';
	import { page } from '$app/stores';
	import { api } from '$lib/api';
	import { currentEntry, entryLoading, entryError, currentLanguage } from '$lib/stores';
	import { translations, type Language } from '$lib/translations';

	$: currentLanguage;
	$: t = translations[currentLanguage] || translations.en;
	$: lemma = $page.params.lemma;

	onMount(() => {
		loadEntry(lemma);
	});

	async function loadEntry(lemmaParam: string) {
		entryLoading.set(true);
		entryError.set(null);

		try {
			const entry = await api.getEntry(lemmaParam);
			currentEntry.set(entry);
		} catch (error) {
			entryError.set(error instanceof Error ? error.message : 'Failed to load entry');
			currentEntry.set(null);
		} finally {
			entryLoading.set(false);
		}
	}

	$: if (lemma && $currentEntry?.lemma_latin !== lemma) {
		loadEntry(lemma);
	}

	function formatCitation(entry: any) {
		const date = new Date().toLocaleDateString('en-US', { year: 'numeric', month: 'long', day: 'numeric' });
		return `Lontar Balinese Dictionary. "${entry.lemma_latin}." Accessed ${date}.`;
	}

	function copyCitation(entry: any) {
		const citation = formatCitation(entry);
		navigator.clipboard.writeText(citation);
	}
</script>

<svelte:head>
	{#if $currentEntry}
		<title>{$currentEntry.lemma_latin} - Lontar Balinese Dictionary</title>
		<meta name="description" content="Definition and etymology for {$currentEntry.lemma_latin} in the Lontar Balinese Dictionary" />
	{/if}
</svelte:head>

<div class="container">
	{#if $entryLoading}
		<div class="loading">Loading entry...</div>
	{:else if $entryError}
		<div class="error">
			<h1>Entry Not Found</h1>
			<p>{$entryError}</p>
			<a href="/" class="btn btn-primary">Return to Search</a>
		</div>
	{:else if $currentEntry}
		<article class="entry">
			<header class="entry-header">
				<div class="lemma-section">
					<h1 class="lemma">
						{$currentEntry.lemma_latin}
						{#if $currentEntry.lemma_aksara}
							<span class="bali-text">{$currentEntry.lemma_aksara}</span>
						{/if}
					</h1>
					<div class="lemma-meta">
						<span class="badge badge-register">{$currentEntry.register}</span>
						<span class="badge badge-domain">{$currentEntry.pos}</span>
					</div>
				</div>
			</header>

			{#if $currentEntry.senses && $currentEntry.senses.length > 0}
				<section class="senses">
					<h2>{t.entry?.definition || 'Definitions'}</h2>
					{#each $currentEntry.senses as sense, index}
						<div class="sense">
							<div class="sense-header">
								<span class="sense-number">{index + 1}.</span>
								<span class="sense-domain">{sense.domain}</span>
							</div>
							<div class="sense-definitions">
								<div class="definition">
									<strong>ID:</strong> {sense.definition_indonesian}
								</div>
								<div class="definition">
									<strong>EN:</strong> {sense.definition_english}
								</div>
							</div>

							{#if sense.attestations && sense.attestations.length > 0}
								<div class="attestations">
									<h4>Attestations</h4>
									{#each sense.attestations as attestation}
										<div class="attestation">
											<div class="attestation-source">
												<strong>{attestation.source}</strong>
												{#if attestation.date_range}
													<span class="date-range">({attestation.date_range})</span>
												{/if}
											</div>
											<blockquote class="attestation-quote">
												<div class="quote-latin">"{attestation.quote_latin}"</div>
												<div class="quote-translation">{attestation.quote_translation}</div>
											</blockquote>
										</div>
									{/each}
								</div>
							{/if}
						</div>
					{/each}
				</section>
			{/if}

			{#if $currentEntry.etymology}
				<section class="etymology">
					<h2>{t.entry?.etymology || 'Etymology'}</h2>
					<div class="etymology-chain">
						{#each $currentEntry.etymology.chain as step}
							<div class="etymology-step">
								<span class="language">{step.language}</span>
								<span class="form">{step.form}</span>
								{#if step.meaning}
									<span class="meaning">"{step.meaning}"</span>
								{/if}
							</div>
						{/each}
					</div>
				</section>
			{/if}

			{#if $currentEntry.cross_references && $currentEntry.cross_references.length > 0}
				<section class="cross-references">
					<h2>{t.entry?.cross_references || 'Cross References'}</h2>
					<div class="cross-ref-list">
						{#each $currentEntry.cross_references as ref}
							<a href="/entry/{ref.lemma}" class="cross-ref">
								<span class="ref-type">{ref.type}:</span>
								<span class="ref-lemma">{ref.lemma}</span>
							</a>
						{/each}
					</div>
				</section>
			{/if}

			<section class="citation">
				<h2>{t.entry?.cite || 'Cite this Entry'}</h2>
				<div class="citation-text">
					{formatCitation($currentEntry)}
				</div>
				<button class="btn btn-secondary" on:click={() => copyCitation($currentEntry)}>
					Copy Citation
				</button>
			</section>
		</article>
	{/if}
</div>

<style>
	.loading, .error {
		text-align: center;
		padding: 3rem 1rem;
	}

	.entry {
		max-width: 800px;
		margin: 0 auto;
	}

	.entry-header {
		margin-bottom: 3rem;
		padding-bottom: 2rem;
		border-bottom: 2px solid var(--border);
	}

	.lemma {
		margin: 0 0 1rem 0;
		font-size: 2.5rem;
		font-weight: 700;
		color: var(--color-primary);
		line-height: 1.2;
	}

	.lemma .bali-text {
		display: block;
		font-size: 1.5rem;
		margin-top: 0.5rem;
		color: var(--text-primary);
	}

	.lemma-meta {
		display: flex;
		gap: 0.5rem;
		flex-wrap: wrap;
	}

	.senses, .etymology, .cross-references, .citation {
		margin-bottom: 3rem;
	}

	.senses h2, .etymology h2, .cross-references h2, .citation h2 {
		margin-bottom: 1.5rem;
		color: var(--text-primary);
	}

	.sense {
		margin-bottom: 2rem;
		padding: 1.5rem;
		background: var(--surface);
		border-radius: 0.5rem;
		border: 1px solid var(--border);
	}

	.sense-header {
		display: flex;
		align-items: center;
		gap: 1rem;
		margin-bottom: 1rem;
	}

	.sense-number {
		font-weight: 600;
		color: var(--color-primary);
		font-size: 1.25rem;
	}

	.sense-domain {
		background: var(--accent);
		color: white;
		padding: 0.25rem 0.5rem;
		border-radius: 0.25rem;
		font-size: 0.875rem;
	}

	.sense-definitions {
		display: flex;
		flex-direction: column;
		gap: 0.75rem;
	}

	.definition {
		line-height: 1.6;
	}

	.attestations {
		margin-top: 1.5rem;
	}

	.attestations h4 {
		margin-bottom: 1rem;
		font-size: 1rem;
		color: var(--text-secondary);
	}

	.attestation {
		margin-bottom: 1rem;
		padding: 1rem;
		background: white;
		border-radius: 0.25rem;
		border: 1px solid var(--border);
	}

	.attestation-source {
		margin-bottom: 0.5rem;
		font-size: 0.875rem;
	}

	.date-range {
		color: var(--text-secondary);
		font-weight: normal;
	}

	.attestation-quote {
		margin: 0;
		padding-left: 1rem;
		border-left: 3px solid var(--color-primary);
		font-style: italic;
	}

	.quote-latin {
		font-weight: 500;
		margin-bottom: 0.5rem;
	}

	.quote-translation {
		font-size: 0.875rem;
		color: var(--text-secondary);
		font-style: normal;
	}

	.etymology-chain {
		display: flex;
		flex-direction: column;
		gap: 0.75rem;
	}

	.etymology-step {
		display: flex;
		align-items: center;
		gap: 1rem;
		padding: 0.75rem;
		background: var(--surface);
		border-radius: 0.25rem;
		border: 1px solid var(--border);
	}

	.language {
		background: var(--color-secondary);
		color: white;
		padding: 0.25rem 0.5rem;
		border-radius: 0.25rem;
		font-size: 0.75rem;
		font-weight: 500;
		min-width: 80px;
		text-align: center;
	}

	.form {
		font-weight: 600;
		font-family: var(--font-bali);
	}

	.meaning {
		color: var(--text-secondary);
		font-style: italic;
	}

	.cross-ref-list {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.cross-ref {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem;
		text-decoration: none;
		color: inherit;
		border-radius: 0.25rem;
		transition: background-color 0.2s;
	}

	.cross-ref:hover {
		background: var(--surface);
	}

	.ref-type {
		background: var(--color-secondary);
		color: white;
		padding: 0.25rem 0.5rem;
		border-radius: 0.25rem;
		font-size: 0.75rem;
		font-weight: 500;
		text-transform: capitalize;
	}

	.ref-lemma {
		font-weight: 500;
		color: var(--color-primary);
	}

	.citation-text {
		background: var(--surface);
		padding: 1rem;
		border-radius: 0.25rem;
		border: 1px solid var(--border);
		font-family: monospace;
		font-size: 0.875rem;
		margin-bottom: 1rem;
		word-break: break-all;
	}

	@media (max-width: 768px) {
		.lemma {
			font-size: 2rem;
		}

		.sense-header {
			flex-direction: column;
			align-items: flex-start;
		}

		.etymology-step {
			flex-wrap: wrap;
		}
	}
</style>
