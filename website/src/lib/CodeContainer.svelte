<script lang="ts">
    let { command }: { command: string } = $props();

    let copied = $state(false);

    function copyCommand() {
        navigator.clipboard.writeText(command);
        copied = true;
        setTimeout(() => {
            copied = false;
        }, 2000);
    }
</script>

<div class="container">
    <pre>{command}</pre>
    <button class="copy-button" onclick={copyCommand} aria-label="Copy code">
        {#if copied}
            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 -960 960 960" aria-hidden="true" class="check-icon">
                <path d="M382-240 154-468l57-57 171 171 367-367 57 57-424 424Z" />
            </svg>
        {:else}
            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 -960 960 960" aria-hidden="true" class="copy-icon">
                <path
                    d="M360-240q-33 0-56.5-23.5T280-320v-480q0-33 23.5-56.5T360-880h360q33 0 56.5 23.5T800-800v480q0 33-23.5 56.5T720-240H360Zm0-80h360v-480H360v480ZM200-80q-33 0-56.5-23.5T120-160v-560h80v560h440v80H200Zm160-240v-480 480Z"
                />
            </svg>
        {/if}
    </button>
</div>

<style>
    .container {
        font-family: 'JetBrains Mono', 'SF Mono', 'Monaco', 'Inconsolata', 'Roboto Mono', 'Consolas', monospace;
        font-size: var(--font-size-code);
        font-weight: var(--font-weight-code);
        color: var(--gray-lightest);
        text-align: left;
        background-color: var(--gray-darkest);
        border: 1px solid var(--gray-light);
        border-radius: 8px;
        padding: 12px 16px;
        position: relative;
        display: flex;
        align-items: flex-start;
        min-height: 44px;
        box-shadow: 0px 2px 4px rgba(0, 0, 0, 0.2);
        overflow: hidden;
        transition: border-color 0.2s ease;
    }

    .container:hover {
        border-color: rgba(124, 139, 154, 0.6);
    }

    .copy-button {
        all: unset;
        display: flex;
        align-items: center;
        justify-content: center;
        position: absolute;
        top: 8px;
        right: 8px;
        background: var(--gray-dark);
        border: 1px solid var(--gray-medium);
        border-radius: 6px;
        padding: 6px;
        cursor: pointer;
        transition: all 0.2s ease;
        width: 14px;
        height: 14px;
        z-index: 1;
    }

    .copy-button:hover {
        background: var(--gray-medium);
        border-color: var(--gray-light);
    }

    .copy-button:active {
        transform: scale(0.95);
    }

    svg {
        width: 16px;
        height: 16px;
        transition: all 0.2s ease;
    }

    .copy-icon {
        fill: var(--gray-lightest);
    }

    .check-icon {
        fill: var(--green);
        animation: checkmark 0.3s ease-out;
    }

    @keyframes checkmark {
        0% {
            opacity: 0;
            transform: scale(0.8);
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
        line-height: 1.5;
        color: var(--gray-lightest);
        padding-right: 40px;
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
            width: 16px;
            height: 16px;
            padding: 2px;
        }

        svg {
            width: 14px;
            height: 14px;
        }

        pre {
            font-size: calc(var(--font-size-code) - 1px);
        }
    }
</style>
