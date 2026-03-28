<script lang="ts">
	import { currentLanguage } from '$lib/stores';
	import { translations, type Language } from '$lib/translations';

	$: currentLanguage;
	$: t = translations[currentLanguage] || translations.en;
</script>

<svelte:head>
	<title>About LBD - Lontar Balinese Dictionary</title>
</svelte:head>

<div class="container">
	<article class="about-page">
		<header class="about-header">
			<h1>About the Lontar Balinese Dictionary</h1>
			<p class="subtitle">A comprehensive digital lexicographic resource for the Balinese language</p>
		</header>

		<section class="section">
			<h2>Project Overview</h2>
			<p>
				The Lontar Balinese Dictionary (LBD) is a digital lexicographic project that aims to create a comprehensive,
				scholarly dictionary of the Balinese language with trilingual definitions (Balinese, Indonesian, English),
				etymological information, and corpus attestations.
			</p>
			<p>
				Following the methodology established by the Oxford English Dictionary (OED), LBD documents word usage
				through time using attestations from historical manuscripts, printed sources, and contemporary usage.
			</p>
		</section>

		<section class="section">
			<h2>Scope and Methodology</h2>
			<h3>Entry Structure</h3>
			<p>Each dictionary entry includes:</p>
			<ul>
				<li><strong>Lemma:</strong> Headword in Latin romanization (with Aksara Bali in Phase 2)</li>
				<li><strong>Part of Speech:</strong> Grammatical classification</li>
				<li><strong>Register:</strong> Speech level (alus singgih, alus sor, alus mider, andap, kasar)</li>
				<li><strong>Definitions:</strong> Trilingual definitions in Indonesian and English</li>
				<li><strong>Domain:</strong> Subject area classification (medical, ritual, agricultural, etc.)</li>
				<li><strong>Etymology:</strong> Historical development from Proto-Austronesian through Sanskrit and Kawi</li>
				<li><strong>Attestations:</strong> Documented usage from primary sources with dates and context</li>
				<li><strong>Cross-references:</strong> Synonyms, antonyms, and related terms</li>
			</ul>
		</section>

		<section class="section">
			<h2>Data Sources</h2>
			<h3>Seed Data</h3>
			<p>Initial entries are imported from:</p>
			<ul>
				<li><strong>Balai Bahasa Bali Kamus Bali–Indonesia (3rd ed.)</strong> - 45,188 draft entries</li>
				<li><strong>Sutjaja Tuttle Balinese-English Dictionary</strong> - English definitions (pending integration)</li>
			</ul>

			<h3>Corpus Sources</h3>
			<p>Attestations are drawn from:</p>
			<ul>
				<li><strong>Prasasti (inscriptions)</strong> - Earliest documented usage</li>
				<li><strong>Gedong Kirtya manuscripts</strong> - Colonial-era lontar collections</li>
				<li><strong>Internet Archive lontar collection</strong> - ~130,000 digitized manuscripts</li>
				<li><strong>Van der Tuuk KBNDW</strong> - 19th century Dutch-Balinese dictionary</li>
			</ul>
		</section>

		<section class="section">
			<h2>Editorial Standards</h2>
			<h3>Entry Lifecycle</h3>
			<ol>
				<li><strong>Draft:</strong> Initial import or submission</li>
				<li><strong>Review:</strong> Editorial validation and enhancement</li>
				<li><strong>Published:</strong> Public-facing with full attestations</li>
			</ol>

			<h3>Attestation Requirements</h3>
			<p>Valid entries require:</p>
			<ul>
				<li>Minimum of one independent attestation from ranked sources</li>
				<li>Clear source citation with date range</li>
				<li>Contextual quote in original language and translation</li>
				<li>Confidence assessment for source reliability</li>
			</ul>
		</section>

		<section class="section">
			<h2>Technical Architecture</h2>
			<p>LBD is built with modern web technologies:</p>
			<ul>
				<li><strong>Backend:</strong> Rust with Axum framework</li>
				<li><strong>Database:</strong> PostgreSQL with Meilisearch for full-text search</li>
				<li><strong>Frontend:</strong> SvelteKit with server-side rendering</li>
				<li><strong>Search:</strong> Meilisearch with ranking optimized for dictionary lookup</li>
				<li><strong>API:</strong> RESTful OpenAPI specification</li>
			</ul>
		</section>

		<section class="section">
			<h2>Development Phases</h2>
			<ol>
				<li><strong>Phase 0:</strong> Foundation - Schema, governance, and seed data</li>
				<li><strong>Phase 1:</strong> API and Search - Backend with Meilisearch integration</li>
				<li><strong>Phase 1.5:</strong> Public Web UI - User interface with Latin romanization</li>
				<li><strong>Phase 2:</strong> Aksara Bali Rendering - Unicode Balinese script support</li>
				<li><strong>Phase 3:</strong> Corpus Attestation Pipeline - OCR integration</li>
				<li><strong>Phase 4:</strong> Editorial Interface - Content management tools</li>
				<li><strong>Phase 5:</strong> Institutional Partnerships - Academic collaborations</li>
			</ol>
		</section>

		<section class="section">
			<h2>Contributing</h2>
			<p>LBD welcomes contributions from:</p>
			<ul>
				<li><strong>Linguists and philologists</strong> - Etymology and historical usage</li>
				<li><strong>Native speakers</strong> - Contemporary usage and pronunciation</li>
				<li><strong>Academic researchers</strong> - Specialized domain terminology</li>
				<li><strong>Technical contributors</strong> - Development and infrastructure</li>
			</ul>
			<p>
				See our <a href="https://github.com/lontar-project/lbd">GitHub repository</a> for technical contributions
				and <a href="/how-to-contribute">contribution guidelines</a> for content submissions.
			</p>
		</section>

		<section class="section">
			<h2>Licensing and Access</h2>
			<p>
				LBD is published under an open license to ensure broad access for educational and research purposes.
				The API is freely available for non-commercial use, and the complete dataset can be exported for
				offline research applications.
			</p>
		</section>

		<section class="section">
			<h2>Contact</h2>
			<p>
				For inquiries about LBD, please contact the project team through our
				<a href="https://github.com/lontar-project/lbd/issues">GitHub issues</a> or
				<a href="mailto:team@lontar-project.org">email</a>.
			</p>
		</section>
	</article>
</div>

<style>
	.about-page {
		max-width: 800px;
		margin: 0 auto;
	}

	.about-header {
		text-align: center;
		margin-bottom: 3rem;
	}

	.about-header h1 {
		margin-bottom: 1rem;
		font-size: 2.5rem;
		color: var(--color-primary);
	}

	.subtitle {
		color: var(--text-secondary);
		font-size: 1.25rem;
		margin: 0;
	}

	.section {
		margin-bottom: 3rem;
	}

	.section h2 {
		margin-bottom: 1.5rem;
		color: var(--text-primary);
		border-bottom: 2px solid var(--border);
		padding-bottom: 0.5rem;
	}

	.section h3 {
		margin: 2rem 0 1rem 0;
		color: var(--text-primary);
	}

	.section p {
		line-height: 1.7;
		margin-bottom: 1rem;
	}

	.section ul, .section ol {
		margin: 1.5rem 0;
		padding-left: 2rem;
	}

	.section li {
		margin-bottom: 0.5rem;
		line-height: 1.6;
	}

	.section a {
		color: var(--color-primary);
		text-decoration: none;
	}

	.section a:hover {
		text-decoration: underline;
	}

	strong {
		color: var(--text-primary);
	}

	@media (max-width: 768px) {
		.about-header h1 {
			font-size: 2rem;
		}

		.subtitle {
			font-size: 1.125rem;
		}

		.section ul, .section ol {
			padding-left: 1.5rem;
		}
	}
</style>
