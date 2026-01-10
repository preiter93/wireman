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
        <h3 class="title" id="card-title-{title.replace(/\s+/g, '-').toLowerCase()}">{title}</h3>
        {#if !hideToggle}
            <div class="chevron-container">
                <svg
                    class="chevron {isExpanded ? 'expanded' : 'collapsed'}"
                    xmlns="http://www.w3.org/2000/svg"
                    viewBox="0 0 24 24"
                    aria-hidden="true"
                >
                    <path d="M12 15L6 9H18L12 15Z" />
                </svg>
            </div>
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
        background-color: var(--gray-dark);
        border: 1px solid rgba(124, 139, 154, 0.2);
        border-radius: 12px;
        padding: 0;
        margin: 14px 10px 0 10px;
        display: flex;
        flex-direction: column;
        justify-content: flex-start;
        position: relative;
        overflow: hidden;
        transition:
            border-color 0.3s ease,
            box-shadow 0.3s ease;
        box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
    }

    .content-card:hover {
        border-color: rgba(124, 139, 154, 0.4);
        box-shadow: 0 6px 16px rgba(0, 0, 0, 0.2);
    }

    .content-card.expanded {
        border-color: rgba(255, 165, 0, 0.2);
    }

    .header {
        all: unset;
        display: flex;
        justify-content: space-between;
        align-items: center;
        padding: 10px 14px;
        cursor: pointer;
        transition: background-color 0.2s ease;
        border-radius: 12px;
        position: relative;
    }

    .header:not([disabled]):hover {
        background: rgba(255, 255, 255, 0.05);
    }

    .header:focus-visible {
        outline: 2px solid var(--orange);
        outline-offset: -2px;
    }

    .header[disabled] {
        cursor: default;
    }

    .title {
        margin: 0;
        color: var(--gray-lightest);
        font-weight: var(--font-weight-h3);
        font-size: var(--font-size-h3);
        transition: color 0.3s ease;
    }

    .content-card.collapsed:hover .title {
        color: var(--orange);
    }

    .chevron-container {
        display: flex;
        align-items: center;
        justify-content: center;
        width: 32px;
        height: 32px;
        border-radius: 50%;
        background: rgba(255, 255, 255, 0.1);
        transition: all 0.3s ease;
    }

    .header:hover .chevron-container {
        background: rgba(255, 165, 0, 0.2);
        transform: scale(1.05);
    }

    .chevron {
        height: var(--icon-size);
        width: var(--icon-size);
        fill: var(--gray-lightest);
        transition: all 0.4s ease;
        transform-origin: center;
    }

    .chevron.collapsed {
        transform: rotate(180deg);
    }

    .chevron.expanded {
        transform: rotate(0deg);
        fill: var(--orange);
    }

    .content {
        padding: 0 14px 14px 14px;
        overflow: hidden;
        background: rgba(255, 255, 255, 0.02);
        border-top: 1px solid rgba(255, 255, 255, 0.1);
    }

    .content :global(p:first-child) {
        margin-top: 10px;
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

    .content :global(code) {
        font-family: var(--font-family-mono);
        background-color: var(--gray-darkest);
        color: var(--gray-lightest);
        border: 1px solid rgba(124, 139, 154, 0.3);
        border-radius: 4px;
        padding: 2px 6px;
        font-size: var(--font-size-code);
        font-weight: var(--font-weight-code);
    }

    /* Mobile optimizations */
    @media (max-width: 600px) {
        .content-card {
            margin: 10px 6px 0 6px;
            border-radius: 8px;
        }

        .header {
            padding: 8px 12px;
        }

        .content {
            padding: 0 12px 12px 12px;
        }

        .title {
            font-size: calc(var(--font-size-h3) - 1px);
        }

        .chevron-container {
            width: 28px;
            height: 28px;
        }
    }

    /* Reduced motion support */
    @media (prefers-reduced-motion: reduce) {
        .content-card,
        .header,
        .chevron,
        .chevron-container,
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

        .title {
            color: var(--gray-lightest);
        }
    }
</style>
