<script lang="ts">
    let { command }: { command: string } = $props();

    let copied = $state(false);
    let isShellCommand = $derived(!command.includes('\n'));

    async function copyCommand() {
        await navigator.clipboard.writeText(command);
        copied = true;
        setTimeout(() => {
            copied = false;
        }, 2000);
    }
</script>

<div class="container">
    <pre>{#if isShellCommand}<span class="prompt" aria-hidden="true">$ </span>{/if}{command}</pre>
    <button class="copy-button {copied ? 'copied' : ''}" onclick={copyCommand} aria-label="Copy code">
        {#if copied}
            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" aria-hidden="true" class="check-icon">
                <path d="M5 12.5l4.5 4.5L19 7.5" />
            </svg>
            <span>copied</span>
        {:else}
            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" aria-hidden="true">
                <rect x="9" y="9" width="11" height="11" rx="2" />
                <path d="M5 15V6a2 2 0 0 1 2-2h9" />
            </svg>
            <span>copy</span>
        {/if}
    </button>
</div>

<style>
    .container {
        font-family: var(--font-family-mono);
        font-size: var(--font-size-code);
        font-weight: var(--font-weight-code);
        color: var(--gray-lightest);
        text-align: left;
        background-color: hsla(215, 21%, 5%, 0.8);
        border: 1px solid hsla(215, 18%, 65%, 0.18);
        border-radius: 8px;
        padding: 12px 16px;
        margin-top: 4px;
        position: relative;
        display: flex;
        align-items: flex-start;
        min-height: 44px;
        box-shadow: inset 0 1px 0 hsla(215, 28%, 92%, 0.03);
        overflow: hidden;
        transition: border-color 0.2s ease;
    }

    .container:hover {
        border-color: hsla(36, 100%, 50%, 0.3);
    }

    .prompt {
        color: var(--orange);
        user-select: none;
    }

    .copy-button {
        all: unset;
        display: flex;
        align-items: center;
        gap: 5px;
        position: absolute;
        top: 8px;
        right: 8px;
        padding: 3px 8px;
        font-family: var(--font-family-mono);
        font-size: 11px;
        color: var(--gray-light);
        background: hsla(215, 19%, 13%, 0.9);
        border: 1px solid hsla(215, 18%, 65%, 0.2);
        border-radius: 6px;
        cursor: pointer;
        opacity: 0.55;
        transition:
            opacity 0.2s ease,
            color 0.2s ease,
            border-color 0.2s ease,
            background-color 0.2s ease;
    }

    .container:hover .copy-button,
    .copy-button:focus-visible,
    .copy-button.copied {
        opacity: 1;
    }

    .copy-button:hover {
        color: var(--orange);
        border-color: hsla(36, 100%, 50%, 0.5);
        background: hsla(36, 100%, 50%, 0.08);
    }

    .copy-button:active {
        transform: scale(0.96);
    }

    .copy-button.copied {
        color: var(--green-light);
        border-color: hsla(150, 100%, 40%, 0.45);
        background: hsla(150, 100%, 40%, 0.08);
    }

    svg {
        width: 13px;
        height: 13px;
        fill: none;
        stroke: currentColor;
        stroke-width: 2;
        stroke-linecap: round;
        stroke-linejoin: round;
    }

    .check-icon {
        animation: pop 0.25s ease-out;
    }

    @keyframes pop {
        0% {
            opacity: 0;
            transform: scale(0.6);
        }
        100% {
            opacity: 1;
            transform: scale(1);
        }
    }

    pre {
        margin: 0;
        white-space: pre-wrap;
        word-break: break-all;
        line-height: 1.6;
        color: var(--gray-lightest);
        padding-right: 80px;
        flex: 1;
        overflow-x: auto;
    }

    /* Mobile optimizations */
    @media (max-width: 600px) {
        .container {
            padding: 10px 12px;
            border-radius: 6px;
            min-height: 40px;
        }

        .copy-button {
            opacity: 1;
        }

        pre {
            font-size: calc(var(--font-size-code) - 1px);
        }
    }

    @media (prefers-reduced-motion: reduce) {
        .copy-button,
        .check-icon {
            transition: none;
            animation: none;
        }
    }
</style>
