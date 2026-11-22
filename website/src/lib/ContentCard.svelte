<script lang="ts">
    import type { Snippet } from 'svelte';
    import { slide } from 'svelte/transition';
    import { quintOut } from 'svelte/easing';

    let {
        children,
        title,
        hideToggle = false,
        initiallyExpanded = false
    }: {
        children: Snippet;
        title: string;
        hideToggle?: boolean;
        initiallyExpanded?: boolean;
    } = $props();

    let isExpanded = $state(initiallyExpanded);

    function toggle() {
        if (!hideToggle) {
            isExpanded = !isExpanded;
        }
    }

    function handleKeydown(event: KeyboardEvent) {
        if (event.key === 'Enter' || event.key === ' ') {
            event.preventDefault();
            toggle();
        }
    }
</script>

<div
    class="content-card {isExpanded ? 'expanded' : 'collapsed'}"
    role="region"
    aria-labelledby="card-title-{title.replace(/\s+/g, '-').toLowerCase()}"
>
    <button
        class="header"
        onclick={toggle}
        onkeydown={handleKeydown}
        tabindex={hideToggle ? -1 : 0}
        aria-expanded={isExpanded}
        aria-label={hideToggle ? title : `${isExpanded ? 'Collapse' : 'Expand'} ${title}`}
        disabled={hideToggle}
    >
        {#if !hideToggle}
            <svg class="chevron" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" aria-hidden="true">
                <path d="M9 6l6 6-6 6" />
            </svg>
        {/if}
        <h3 class="title" id="card-title-{title.replace(/\s+/g, '-').toLowerCase()}">{title}</h3>
        {#if !hideToggle}
            <span class="hint" aria-hidden="true">{isExpanded ? '[-]' : '[+]'}</span>
        {/if}
    </button>

    {#if isExpanded}
        <div
            class="content"
            transition:slide={{
                duration: 400,
                easing: quintOut
            }}
        >
            {@render children()}
        </div>
    {/if}
</div>

<style>
    .content-card {
        background: var(--gray-dark);
        border: 1px solid hsla(215, 18%, 65%, 0.16);
        border-radius: 6px;
        margin: 12px 10px 0 10px;
        display: flex;
        flex-direction: column;
        position: relative;
        overflow: hidden;
        transition: border-color 0.2s ease;
    }

    .content-card:hover {
        border-color: hsla(215, 18%, 65%, 0.3);
    }

    .header {
        all: unset;
        display: flex;
        align-items: center;
        gap: 10px;
        padding: 12px 16px;
        cursor: pointer;
        transition: background-color 0.2s ease;
    }

    .header:not([disabled]):hover {
        background: hsla(215, 28%, 92%, 0.03);
    }

    .header:focus-visible {
        outline: 2px solid var(--orange);
        outline-offset: -2px;
    }

    .header[disabled] {
        cursor: default;
    }

    .chevron {
        flex-shrink: 0;
        width: var(--icon-size);
        height: var(--icon-size);
        fill: none;
        stroke: var(--gray-medium);
        stroke-width: 2.5;
        stroke-linecap: round;
        stroke-linejoin: round;
        transition:
            transform 0.3s cubic-bezier(0.4, 0, 0.2, 1),
            stroke 0.2s ease;
    }

    .expanded .chevron {
        transform: rotate(90deg);
        stroke: var(--orange);
    }

    .header:hover .chevron {
        stroke: var(--orange);
    }

    .title {
        flex: 1;
        margin: 0;
        color: var(--gray-lightest);
        font-weight: var(--font-weight-h3);
        font-size: var(--font-size-h3);
        transition: color 0.2s ease;
    }

    .header:hover .title {
        color: var(--orange-light);
    }

    .hint {
        font-family: var(--font-family-mono);
        font-size: var(--font-size-code);
        color: var(--gray-medium);
        transition: color 0.2s ease;
    }

    .header:hover .hint {
        color: var(--orange);
    }

    .content {
        padding: 0 16px 16px 16px;
        overflow: hidden;
        border-top: 1px dashed hsla(215, 18%, 65%, 0.16);
    }

    .content :global(p:first-child) {
        margin-top: 14px;
    }

    .content :global(p:last-child) {
        margin-bottom: 0;
    }

    .content :global(h3) {
        color: var(--orange);
        margin-top: 20px;
        margin-bottom: 12px;
        font-size: calc(var(--font-size-h3) - 2px);
    }

    .content :global(ul) {
        margin: 12px 0;
        padding-left: 20px;
    }

    .content :global(li) {
        margin: 8px 0;
        line-height: 1.6;
    }

    .content :global(li::marker) {
        color: var(--orange);
    }

    .content :global(code) {
        font-family: var(--font-family-mono);
        background-color: hsla(36, 100%, 50%, 0.08);
        color: var(--orange-light);
        border: 1px solid hsla(36, 100%, 50%, 0.18);
        border-radius: 4px;
        padding: 1px 6px;
        font-size: var(--font-size-code);
        font-weight: var(--font-weight-code);
    }

    /* Mobile optimizations */
    @media (max-width: 600px) {
        .content-card {
            margin: 10px 6px 0 6px;
            border-radius: 6px;
        }

        .header {
            padding: 10px 12px;
        }

        .content {
            padding: 0 12px 12px 12px;
        }
    }

    /* Reduced motion support */
    @media (prefers-reduced-motion: reduce) {
        .content-card,
        .header,
        .chevron,
        .title {
            transition: none;
        }
    }

    /* High contrast mode */
    @media (prefers-contrast: high) {
        .content-card {
            border-width: 2px;
            border-color: var(--gray-light);
        }
    }
</style>
