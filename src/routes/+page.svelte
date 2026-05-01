<script lang="ts">
	import Terminal from '../components/Terminal.svelte';
	import Sidebar from '../components/Sidebar.svelte';

	// Reference to the Terminal component instance
	let terminalComponent: Terminal;

	// Handles the event emitted by the Sidebar
	function handleMenuClick(commandToRun: string) {
		if (terminalComponent) {
			terminalComponent.executeGuiCommand(commandToRun);
		}
	}
</script>

<main class="os-layout">
	<Sidebar onCommand={handleMenuClick} />

	<section class="terminal-container">
		<header>
			<h1>MindCoreOs Portfolio</h1>
			<p>Navigate using the terminal or the sidebar menu.</p>
		</header>

		<Terminal bind:this={terminalComponent} />
	</section>
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
	.os-layout {
		display: grid;
		grid-template-columns: auto 1fr;
		min-height: 100vh;
	}
	@media (max-width: 768px) {
		.os-layout {
			/* On mobile, stack them vertically: sidebar on top, terminal below */
			display: flex;
			flex-direction: column;
		}

		.terminal-container {
			padding: 1rem;
		}
	}
	.terminal-container {
		padding: 2rem;
		display: flex;
		flex-direction: column;
		gap: 2rem;
		max-width: 1000px;
		margin: 0 auto;
		width: 100%;
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
