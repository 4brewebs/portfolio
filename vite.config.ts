import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';

export default defineConfig({
	plugins: [sveltekit()],
	server: {
		fs: {
			// Allow Vite to serve the WebAssembly files from our Rust core
			allow: ['./mind-core']
		}
	}
});
