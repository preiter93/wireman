<script lang="ts">
    import type { Snippet } from 'svelte';

    let { children }: { children: Snippet } = $props();
</script>

{@render children()}
<svelte:head>
    <style>
        @import url('https://fonts.googleapis.com/css?family=Raleway:400,700&display=swap');
    </style>
</svelte:head>

<style>
    :root {
        --font-family: 'Raleway', sans-serif;

        --font-size-h1: 48px;
        --font-size-h2: 36px;
        --font-size-h3: 22px;
        --font-size-h4: 22px;
        --font-size-h5: 26px;
        --font-size-body: 16px;
        --font-size-code: 14px;

        --font-weight-h1: 900;
        --font-weight-h2: 500;
        --font-weight-h3: 600;
        --font-weight-h4: 400;
        --font-weight-body: 400;
        --font-weight-code: 400;

        --gray-darkest: hsl(230, 16%, 7.5%);
        --gray-dark: hsl(210, 24%, 15%);
        --gray-medium: hsl(210, 27%, 50%);
        --gray-light: hsl(210, 30%, 70%);
        --gray-lightest: hsl(210, 40%, 96%);
        --orange: hsl(36, 100%, 50%);
        --magenta: hsl(315, 100%, 60%);
        --green: hsl(150, 100%, 40%);

        --icon-size: 18px;
        --icon-size-large: 36px;
        --icon-size-xlarge: 48px;

        --text-margin: 18px;

        --max-width: 800px;
    }

    @media (max-width: 600px) {
        :root {
            --font-size-h1: 26px;
            --font-size-h2: 22px;
            --font-size-h3: 14px;
            --font-size-h4: 14px;
            --font-size-h5: 16px;
            --font-size-body: 14px;
            --font-size-code: 12px;

            --icon-size: 16px;
            --icon-size-large: 24px;
            --icon-size-xlarge: 32px;

            --text-margin: 16px;
        }
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
        height: 100vh;
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
        background-color: var(--gray-darkest);
        color: var(--gray-lightest);
        border-radius: 4px;
        padding: 1px 4px;
        border: 1px solid var(--gray-light);
        font-size: var(--font-size-code);
    }
</style>
