<!-- <script lang="ts">
	import { onMount } from 'svelte';
	import { Terminal } from 'xterm';
	import { FitAddon } from 'xterm-addon-fit';
	import 'xterm/css/xterm.css';

	// Reference to the HTML element where xterm will inject its canvas
	let terminalContainer: HTMLElement;

	// Buffer to store the current line the user is typing
	let currentLine = '';

	// onMount is critical here: xterm.js needs a real DOM node to attach to,
	// which only exists after the component has been mounted in the browser.
	onMount(() => {
		// 1. Initialize terminal with a VS Code-like dark theme
		const term = new Terminal({
			cursorBlink: true,
			theme: {
				background: '#1e1e1e',
				foreground: '#d4d4d4',
				cursor: '#ffffff'
			},
			fontFamily: '"Fira Code", monospace'
		});

		// 2. Load the FitAddon to make it responsive
		const fitAddon = new FitAddon();
		term.loadAddon(fitAddon);

		// 3. Mount it to the DOM and calculate initial size
		term.open(terminalContainer);
		fitAddon.fit();

		// 4. Initial welcome message
		term.writeln('Welcome to my portfolio! 🚀');
		term.writeln('Type "help" to see available commands.');
		term.write('\r\n$ ');

		// 5. Handle keystrokes directly
		term.onKey(({ key, domEvent }) => {
			const ev = domEvent as KeyboardEvent;
			// Prevent control characters from printing directly
			const printable = !ev.altKey && !ev.ctrlKey && !ev.metaKey;

			if (ev.key === 'Enter') {
				// TODO: Later, we will send 'currentLine' to our Rust WASM core here
				term.write('\r\n');

				if (currentLine.trim().length > 0) {
					// Mock response until Rust is connected
					term.writeln(`Command not found: ${currentLine} (WASM core not connected yet)`);
				}

				term.write('$ ');
				currentLine = ''; // Reset buffer for the next command
			} else if (ev.key === 'Backspace') {
				// Complex logic to prevent the user from deleting the "$ " prompt
				if (currentLine.length > 0) {
					currentLine = currentLine.slice(0, -1);
					term.write('\b \b'); // Move back, print space, move back again
				}
			} else if (printable) {
				currentLine += key;
				term.write(key);
			}
		});

		// Handle window resize to auto-adjust terminal dimensions
		window.addEventListener('resize', () => {
			fitAddon.fit();
		});
	});
</script> -->
<script lang="ts">
	import { onMount } from 'svelte';
	import { Terminal } from 'xterm';
	import { FitAddon } from 'xterm-addon-fit';
	import 'xterm/css/xterm.css';

	// 1. Importamos la magia de nuestro paquete local (como si fuera de internet)
	import init, { get_welcome_message, process_command } from 'portfolio-mind-core';

	let terminalContainer: HTMLElement;
	let currentLine = '';

	// 2. onMount ahora debe ser ASÍNCRONO para esperar a que cargue el archivo .wasm
	onMount(async () => {
		// 3. Inicializamos el cerebro de Rust
		await init();

		const term = new Terminal({
			cursorBlink: true,
			theme: { background: '#1e1e1e', foreground: '#d4d4d4', cursor: '#ffffff' },
			fontFamily: '"Fira Code", monospace'
		});

		const fitAddon = new FitAddon();
		term.loadAddon(fitAddon);

		term.open(terminalContainer);
		fitAddon.fit();

		const welcome = get_welcome_message();
		term.write(welcome); // Pinta el lince y el mensaje desde Rust
		// term.write('$ ');
		// term.writeln('Welcome to my portfolio! 🚀');
		// term.writeln('Type "help" to see available commands.');
		term.write('\r\n$ ');

		term.onKey(({ key, domEvent }) => {
			const ev = domEvent as KeyboardEvent;
			const printable = !ev.altKey && !ev.ctrlKey && !ev.metaKey;

			if (ev.key === 'Enter') {
				term.write('\r\n');

				if (currentLine.trim().length > 0) {
					// 4. ¡AQUÍ ESTÁ LA MAGIA! Pasamos el texto a Rust y guardamos la respuesta
					const response = process_command(currentLine);

					// xterm.js necesita pintar línea por línea. Separamos por \r\n
					const lines = response.split('\r\n');
					for (const line of lines) {
						term.writeln(line);
					}
				}

				term.write('$ ');
				currentLine = ''; // Vaciamos el buffer
			} else if (ev.key === 'Backspace') {
				if (currentLine.length > 0) {
					currentLine = currentLine.slice(0, -1);
					term.write('\b \b');
				}
			} else if (printable) {
				currentLine += key;
				term.write(key);
			}
		});

		window.addEventListener('resize', () => fitAddon.fit());
	});
</script>

<div class="terminal-wrapper" bind:this={terminalContainer}></div>

<style>
	.terminal-wrapper {
		width: 100%;
		height: 60vh; /* Takes 60% of the screen height */
		background-color: #1e1e1e;
		border-radius: 8px;
		box-shadow: 0 10px 15px -3px rgba(0, 0, 0, 0.5);
		overflow: hidden; /* Keeps sharp corners */
	}

	/* Target xterm internals to add some inner padding */
	:global(.xterm) {
		padding: 1.5rem;
	}
</style>
