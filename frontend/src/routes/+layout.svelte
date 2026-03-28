<script lang="ts">
	import '../app.css';
	import { onMount } from 'svelte';
	import { currentLanguage, theme } from '$lib/stores';
	import { translations, type Language, type Translation } from '$lib/translations';
	import { page } from '$app/stores';
	import { browser } from '$app/environment';

	// Get current year
	const currentYear = new Date().getFullYear();

	// Theme handling
	onMount(() => {
		if (browser) {
			const savedTheme = localStorage.getItem('theme') as 'light' | 'dark' | null;
			if (savedTheme) {
				theme.set(savedTheme);
			}
		}
	});

	$: if ($theme && browser) {
		localStorage.setItem('theme', $theme);
		document.documentElement.setAttribute('data-theme', $theme);
	}

	// Language handling
	$: if ($currentLanguage && browser) {
		localStorage.setItem('language', $currentLanguage);
	}

	// Get current translation
	$: t = translations[$currentLanguage] || translations.en;
</script>

<svelte:head>
	<title>Lontar Balinese Dictionary</title>
	<meta name="description" content="A comprehensive Balinese-Indonesian-English dictionary with etymology and attestations" />
</svelte:head>

<div class="app">
	<header class="header">
		<nav class="nav container">
			<div class="nav-brand">
				<a href="/" class="brand-link">
					<h1>LBD</h1>
					<span class="brand-subtitle">Lontar Balinese Dictionary</span>
				</a>
			</div>

			<div class="nav-actions">
				<!-- Language toggle -->
				<div class="language-toggle">
					<button
						class="btn btn-secondary"
						on:click={() => currentLanguage.set($currentLanguage === 'en' ? 'id' : 'en')}
					>
						{$currentLanguage === 'en' ? 'ID' : 'EN'}
					</button>
				</div>
			</div>
		</nav>
	</header>

	<main class="main">
		<slot />
	</main>

	<footer class="footer">
		<div class="container">
			<div class="footer-content">
				<p>&copy; {currentYear} Lontar Balinese Dictionary. Part of the Lontar Project.</p>
				<nav class="footer-nav">
					<a href="/about">{t.navigation?.about}</a>
					<a href="/api/docs">API</a>
					<a href="https://github.com/lontar-project/lbd">GitHub</a>
				</nav>
			</div>
		</div>
	</footer>
</div>

<style>
	.app {
		min-height: 100vh;
		display: flex;
		flex-direction: column;
	}

	.header {
		background: var(--surface);
		border-bottom: 1px solid var(--border);
		padding: 1rem 0;
	}

	.nav {
		display: flex;
		justify-content: space-between;
		align-items: center;
	}

	.nav-brand {
		display: flex;
		align-items: center;
	}

	.brand-link {
		text-decoration: none;
		color: inherit;
		display: flex;
		align-items: baseline;
		gap: 0.5rem;
	}

	.brand-link h1 {
		margin: 0;
		font-size: 1.5rem;
		font-weight: 700;
		color: var(--color-primary);
	}

	.brand-subtitle {
		font-size: 0.875rem;
		color: var(--text-secondary);
	}

	.nav-actions {
		display: flex;
		align-items: center;
		gap: 1rem;
	}

	.language-toggle {
		display: flex;
		align-items: center;
	}

	.main {
		flex: 1;
		padding: 2rem 0;
	}

	.footer {
		background: var(--surface);
		border-top: 1px solid var(--border);
		padding: 2rem 0;
		margin-top: auto;
	}

	.footer-content {
		display: flex;
		justify-content: space-between;
		align-items: center;
		flex-wrap: wrap;
		gap: 1rem;
	}

	.footer-nav {
		display: flex;
		gap: 1rem;
	}

	.footer-nav a {
		color: var(--text-secondary);
		text-decoration: none;
	}

	.footer-nav a:hover {
		color: var(--color-primary);
	}

	@media (max-width: 768px) {
		.footer-content {
			flex-direction: column;
			text-align: center;
		}

		.nav {
			flex-direction: column;
			gap: 1rem;
		}
	}
</style>
