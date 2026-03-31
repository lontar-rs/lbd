<script lang="ts">
	export let count: number = 3;
	export let type: 'entry' | 'corpus' = 'entry';
</script>

<div class="skeleton-container">
	{#each Array(count) as _, i}
		<div class="skeleton-item skeleton-{type}">
			{#if type === 'entry'}
				<div class="skeleton-lemma"></div>
				<div class="skeleton-meta">
					<div class="skeleton-badge"></div>
					<div class="skeleton-badge"></div>
				</div>
				<div class="skeleton-preview"></div>
			{:else if type === 'corpus'}
				<div class="skeleton-header">
					<div class="skeleton-title"></div>
					<div class="skeleton-subtitle"></div>
				</div>
				<div class="skeleton-content">
					<div class="skeleton-line"></div>
					<div class="skeleton-line short"></div>
					<div class="skeleton-line"></div>
				</div>
			{/if}
		</div>
	{/each}
</div>

<style>
	.skeleton-container {
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}

	.skeleton-item {
		background: var(--surface);
		border: 1px solid var(--border);
		border-radius: 0.5rem;
		padding: 1.5rem;
		animation: pulse 1.5s ease-in-out infinite;
	}

	.skeleton-lemma {
		height: 1.5rem;
		background: linear-gradient(90deg, var(--border) 25%, var(--surface) 50%, var(--border) 75%);
		background-size: 200% 100%;
		animation: shimmer 1.5s ease-in-out infinite;
		border-radius: 0.25rem;
		margin-bottom: 0.75rem;
		width: 60%;
	}

	.skeleton-meta {
		display: flex;
		gap: 0.5rem;
		margin-bottom: 0.75rem;
	}

	.skeleton-badge {
		height: 1rem;
		width: 4rem;
		background: linear-gradient(90deg, var(--border) 25%, var(--surface) 50%, var(--border) 75%);
		background-size: 200% 100%;
		animation: shimmer 1.5s ease-in-out infinite;
		border-radius: 0.25rem;
	}

	.skeleton-preview {
		height: 1rem;
		background: linear-gradient(90deg, var(--border) 25%, var(--surface) 50%, var(--border) 75%);
		background-size: 200% 100%;
		animation: shimmer 1.5s ease-in-out infinite;
		border-radius: 0.25rem;
		width: 80%;
	}

	.skeleton-header {
		margin-bottom: 1rem;
	}

	.skeleton-title {
		height: 1.75rem;
		background: linear-gradient(90deg, var(--border) 25%, var(--surface) 50%, var(--border) 75%);
		background-size: 200% 100%;
		animation: shimmer 1.5s ease-in-out infinite;
		border-radius: 0.25rem;
		margin-bottom: 0.5rem;
		width: 70%;
	}

	.skeleton-subtitle {
		height: 1rem;
		background: linear-gradient(90deg, var(--border) 25%, var(--surface) 50%, var(--border) 75%);
		background-size: 200% 100%;
		animation: shimmer 1.5s ease-in-out infinite;
		border-radius: 0.25rem;
		width: 50%;
	}

	.skeleton-content {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.skeleton-line {
		height: 1rem;
		background: linear-gradient(90deg, var(--border) 25%, var(--surface) 50%, var(--border) 75%);
		background-size: 200% 100%;
		animation: shimmer 1.5s ease-in-out infinite;
		border-radius: 0.25rem;
	}

	.skeleton-line.short {
		width: 60%;
	}

	@keyframes shimmer {
		0% {
			background-position: -200% 0;
		}
		100% {
			background-position: 200% 0;
		}
	}

	@keyframes pulse {
		0%, 100% {
			opacity: 1;
		}
		50% {
			opacity: 0.7;
		}
	}
</style>
