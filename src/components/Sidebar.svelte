<script lang="ts">
	let { onCommand = () => {} }: { onCommand: (cmd: string) => void } = $props();

	// List of commands exposed in the UI
	const menuItems = [
		{ label: '👤 About Me', cmd: 'about' },
		{ label: '⚡ Tech Skills', cmd: 'skills' },
		{ label: '📁 Projects', cmd: 'projects' },
		{ label: '🤖 AI Chat', cmd: 'ai-chat' },
		{ label: '🧹 Clear Terminal', cmd: 'clear' }
	];

	function triggerCommand(cmd: string) {
		if (onCommand) {
			onCommand(cmd);
		}
	}
</script>

<aside class="sidebar">
	<div class="logo">MindCore<span>OS</span></div>

	<nav>
		{#each menuItems as item}
			<button class="menu-btn" onclick={() => triggerCommand(item.cmd)}>
				{item.label}
			</button>
		{/each}
	</nav>
</aside>

<style>
	.sidebar {
		background-color: #121212;
		border-right: 1px solid #333;
		padding: 1.5rem;
		display: flex;
		flex-direction: column;
		gap: 2rem;
		min-width: 250px;
	}

	.logo {
		font-size: 1.5rem;
		font-weight: bold;
		color: #e2e8f0;
	}

	.logo span {
		color: #4facfe;
	}

	nav {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.menu-btn {
		background: transparent;
		border: 1px solid transparent;
		color: #a0aec0;
		text-align: left;
		padding: 0.75rem 1rem;
		border-radius: 6px;
		cursor: pointer;
		transition: all 0.2s ease;
		font-family: inherit;
		font-size: 1rem;
	}

	.menu-btn:hover {
		background-color: #1e1e1e;
		color: #4facfe;
		border-color: #333;
	}

	/* Mobile responsive adjustment */
	@media (max-width: 768px) {
		.sidebar {
			flex-direction: row;
			align-items: center;
			padding: 1rem;
			min-width: auto;
			border-right: none;
			border-bottom: 1px solid #333;
			overflow-x: auto; /* Allow horizontal scrolling */
			white-space: nowrap;
		}

		nav {
			flex-direction: row; /* Buttons side by side */
		}

		.menu-btn {
			padding: 0.5rem 1rem;
			font-size: 0.9rem;
			background-color: #1e1e1e;
			border: 1px solid #333;
			border-radius: 20px; /* Pill-shaped buttons for mobile */
		}
	}
</style>
