<script lang="ts">
	import Terminal from '../components/Terminal.svelte';
</script>

<main class="portfolio-container">
	<header>
		<h1>4Brewebs Developer Portfolio</h1>
		<p>Interact with the terminal below to explore my projects and experience.</p>
	</header>

	<Terminal />
</main>

<style>
	/* Global reset for a dark theme look */
	:global(body) {
		background-color: #0f0f0f;
		color: #e2e8f0;
		font-family:
			system-ui,
			-apple-system,
			sans-serif;
		margin: 0;
		padding: 0;
	}

	.portfolio-container {
		max-width: 900px;
		margin: 0 auto;
		padding: 2rem;
		display: flex;
		flex-direction: column;
		gap: 2rem;
	}

	header {
		text-align: center;
	}

	h1 {
		font-size: 2.5rem;
		margin-bottom: 0.5rem;
		background: linear-gradient(to right, #4facfe 0%, #00f2fe 100%);
		-webkit-background-clip: text;
		-webkit-text-fill-color: transparent;
	}
</style>
