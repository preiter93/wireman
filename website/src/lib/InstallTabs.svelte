<script lang="ts">
    import BinariesTable from '#lib/BinariesTable.svelte';
    import CodeContainer from '#lib/CodeContainer.svelte';

    const tabs = ['cargo', 'brew', 'binaries'] as const;

    let selected = $state<(typeof tabs)[number]>('cargo');

    function handleKeydown(event: KeyboardEvent) {
        const index = tabs.indexOf(selected);
        if (event.key === 'ArrowRight') {
            selected = tabs[(index + 1) % tabs.length];
        } else if (event.key === 'ArrowLeft') {
            selected = tabs[(index - 1 + tabs.length) % tabs.length];
        }
    }
</script>

<div class="install">
    <div class="tabs" role="tablist" tabindex="-1" onkeydown={handleKeydown}>
        {#each tabs as tab}
            <button
                class="tab {tab === selected ? 'active' : ''}"
                role="tab"
                id="tab-{tab}"
                aria-selected={tab === selected}
                aria-controls="panel-{tab}"
                tabindex={tab === selected ? 0 : -1}
                onclick={() => (selected = tab)}
            >
                {tab}
            </button>
        {/each}
    </div>

    <div class="panel" role="tabpanel" id="panel-{selected}" aria-labelledby="tab-{selected}">
        {#if selected === 'cargo'}
            <p>
                If you have Rust installed, install wireman from
                <a href="https://crates.io/crates/wireman" target="_blank" rel="noopener noreferrer">crates.io</a>:
            </p>
            <CodeContainer command="cargo install wireman" />
        {:else if selected === 'brew'}
            <p>On Linux and macOS you can install wireman with Homebrew:</p>
            <CodeContainer command="brew install preiter93/wireman/wireman" />
        {:else}
            <p>
                Download a pre-compiled binary from the
                <a href="https://github.com/preiter93/wireman/releases" target="_blank" rel="noopener noreferrer">releases page</a>, or grab
                one of the latest:
            </p>
            <BinariesTable />
        {/if}
    </div>
</div>

<style>
    .install {
        background: var(--gray-dark);
        border: 1px solid hsla(215, 18%, 65%, 0.16);
        border-radius: 6px;
        margin: 12px 10px 0 10px;
        overflow: hidden;
    }

    .tabs {
        display: flex;
        border-bottom: 1px solid hsla(215, 18%, 65%, 0.16);
        outline: none;
    }

    .tab {
        all: unset;
        padding: 10px 18px;
        font-family: var(--font-family-mono);
        font-size: var(--font-size-code);
        color: var(--gray-medium);
        cursor: pointer;
        border-right: 1px solid hsla(215, 18%, 65%, 0.16);
        box-shadow: inset 0 -2px 0 transparent;
        transition:
            color 0.2s ease,
            background-color 0.2s ease,
            box-shadow 0.2s ease;
    }

    .tab:hover {
        color: var(--gray-lightest);
        background: hsla(215, 28%, 92%, 0.03);
    }

    .tab.active {
        color: var(--orange);
        background: hsla(215, 21%, 5%, 0.5);
        box-shadow: inset 0 -2px 0 var(--orange);
    }

    .tab:focus-visible {
        outline: 2px solid var(--orange);
        outline-offset: -2px;
    }

    .panel {
        padding: 16px;
    }

    .panel p {
        margin-bottom: 12px;
    }

    @media (max-width: 600px) {
        .install {
            margin: 10px 6px 0 6px;
        }

        .tab {
            padding: 8px 14px;
        }

        .panel {
            padding: 12px;
        }
    }
</style>
