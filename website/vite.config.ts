import adapter from '@sveltejs/adapter-static';
import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';

export default defineConfig({
    plugins: [
        sveltekit({
            adapter: adapter(),
            paths: {
                base: process.env.BASE_PATH as `/${string}` | undefined
            }
        })
    ]
});
