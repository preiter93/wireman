<script lang="ts">
    import type { Snippet } from 'svelte';

    let { children }: { children: Snippet } = $props();
</script>

{@render children()}
<svelte:head>
    <style>
        @import url('https://fonts.googleapis.com/css2?family=Raleway:wght@300;400;500;600;700;800;900&display=swap');
        @import url('https://fonts.googleapis.com/css2?family=JetBrains+Mono:wght@300;400;500;600&display=swap');
    </style>
</svelte:head>

<style>
    :root {
        --font-family: 'Raleway', -apple-system, BlinkMacSystemFont, 'Segoe UI', system-ui, sans-serif;
        --font-family-mono: 'JetBrains Mono', 'SF Mono', 'Monaco', 'Inconsolata', 'Roboto Mono', 'Consolas', monospace;

        --font-size-h1: 40px;
        --font-size-h2: 28px;
        --font-size-h3: 18px;
        --font-size-h4: 18px;
        --font-size-h5: 22px;
        --font-size-body: 14px;
        --font-size-code: 12px;

        --font-weight-h1: 900;
        --font-weight-h2: 500;
        --font-weight-h3: 600;
        --font-weight-h4: 400;
        --font-weight-body: 400;
        --font-weight-code: 500;

        --gray-darkest: hsl(230, 16%, 7.5%);
        --gray-dark: hsl(210, 24%, 15%);
        --gray-medium: hsl(210, 27%, 50%);
        --gray-light: hsl(210, 30%, 70%);
        --gray-lightest: hsl(210, 40%, 96%);
        --orange: hsl(36, 100%, 50%);
        --orange-light: hsl(36, 100%, 65%);
        --magenta: hsl(315, 100%, 60%);
        --magenta-light: hsl(315, 100%, 75%);
        --green: hsl(150, 100%, 40%);
        --green-light: hsl(150, 100%, 55%);

        --icon-size: 16px;
        --icon-size-large: 30px;
        --icon-size-xlarge: 40px;

        --text-margin: 14px;
        --content-spacing: 20px;
        --border-radius: 10px;
        --border-radius-small: 6px;

        --max-width: 720px;
        --img-max-width: 540px;

        --card-padding: 24px 20px;
        --card-margin: 20px 14px 0 14px;
        --dot-size: 8px;
        --dot-gap: 10px;
        --chevron-btn-size: 28px;
        --chevron-size: 14px;

        --shadow-small: 0 2px 4px rgba(0, 0, 0, 0.1);
        --shadow-medium: 0 4px 12px rgba(0, 0, 0, 0.15);
        --shadow-large: 0 8px 25px rgba(0, 0, 0, 0.25);

        --transition-fast: 0.15s cubic-bezier(0.4, 0, 0.2, 1);
        --transition-normal: 0.3s cubic-bezier(0.4, 0, 0.2, 1);
        --transition-slow: 0.5s cubic-bezier(0.4, 0, 0.2, 1);
    }

    @media (max-width: 600px) {
        :root {
            --font-size-h1: 22px;
            --font-size-h2: 18px;
            --font-size-h3: 13px;
            --font-size-h4: 13px;
            --font-size-h5: 14px;
            --font-size-body: 13px;
            --font-size-code: 11px;

            --icon-size: 14px;
            --icon-size-large: 20px;
            --icon-size-xlarge: 28px;

            --text-margin: 12px;
            --img-max-width: 100%;

            --card-padding: 20px 14px;
            --card-margin: 16px 10px 0 10px;
            --dot-size: 6px;
            --dot-gap: 8px;
            --chevron-btn-size: 24px;
            --chevron-size: 12px;
        }
    }

    :global(html) {
        scroll-behavior: smooth;
    }

    :global(body) {
        margin: 0;
        padding: 0;
        font-family: var(--font-family);
        background-color: var(--gray-darkest);
        display: flex;
        justify-content: start;
        align-items: center;
        flex-direction: column;
        min-height: 100vh;
        line-height: 1.6;
        -webkit-font-smoothing: antialiased;
        -moz-osx-font-smoothing: grayscale;
        text-rendering: optimizeLegibility;
    }

    :global(h1) {
        font-family: var(--font-family);
        font-size: var(--font-size-h1);
        font-weight: var(--font-weight-h1);
        color: var(--gray-lightest);
        text-align: center;
        margin: var(--text-margin) 0 0 0;
    }

    :global(h2) {
        font-family: var(--font-family);
        font-size: var(--font-size-h2);
        font-weight: var(--font-weight-h2);
        text-align: left;
        color: var(--orange);
        text-align: center;
        margin: var(--text-margin) 0;
    }

    :global(h3) {
        font-family: var(--font-family);
        font-size: var(--font-size-h3);
        font-weight: var(--font-weight-h3);
        text-align: left;
        color: var(--gray-lightest);
        margin: 0;
    }

    :global(h4) {
        font-family: var(--font-family);
        font-size: var(--font-size-h4);
        font-weight: var(--font-weight-h4);
        text-align: center;
        color: var(--gray-lightest);
    }

    :global(p) {
        font-family: var(--font-family);
        font-size: var(--font-size-body);
        font-weight: var(--font-weight-body);
        text-align: left;
        color: var(--gray-lightest);
        line-height: 1.4em;
        margin: 0;
    }

    :global(a) {
        font-family: var(--font-family);
        font-size: var(--font-size-body);
        font-weight: var(--font-weight-body);
        text-decoration: underline;
        color: var(--gray-lightest);
    }

    :global(code) {
        font-family: var(--font-family-mono);
        background-color: var(--gray-darkest);
        color: var(--gray-lightest);
        border-radius: 4px;
        padding: 2px 6px;
        border: 1px solid rgba(124, 139, 154, 0.3);
        font-size: var(--font-size-code);
        font-weight: var(--font-weight-code);
    }

    :global(*) {
        box-sizing: border-box;
    }

    :global(::selection) {
        background: var(--orange);
        color: var(--gray-darkest);
    }

    :global(::-moz-selection) {
        background: var(--orange);
        color: var(--gray-darkest);
    }

    :global(:focus-visible) {
        outline: 2px solid var(--orange);
        outline-offset: 2px;
    }

    /* Scrollbar styling */
    :global(::-webkit-scrollbar) {
        width: 8px;
        height: 8px;
    }

    :global(::-webkit-scrollbar-track) {
        background: var(--gray-dark);
        border-radius: 4px;
    }

    :global(::-webkit-scrollbar-thumb) {
        background: var(--gray-medium);
        border-radius: 4px;
    }

    :global(::-webkit-scrollbar-thumb:hover) {
        background: var(--gray-light);
    }
</style>
