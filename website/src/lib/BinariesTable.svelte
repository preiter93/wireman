<script>
    import LinuxIcon from '$lib/icons/LinuxIcon.svelte';
    import MacOSIcon from '$lib/icons/MacOSIcon.svelte';
    import WindowsIcon from '$lib/icons/WindowsIcon.svelte';
</script>

{#snippet linux()}
    <td><div class="os-container"><LinuxIcon /> Linux</div> </td>
{/snippet}

{#snippet macos()}
    <td><div class="os-container"><MacOSIcon /> MacOS</div> </td>
{/snippet}

{#snippet windows()}
    <td><div class="os-container"><WindowsIcon /> Windows</div> </td>
{/snippet}

{#snippet link(/** @type {string} */ name)}
    <td>
        <a href="https://github.com/preiter93/wireman/releases/download/v0.2.10/{name}" download>
            {name}
        </a>
    </td>
{/snippet}

<table>
    <thead>
        <tr>
            <th class="os">OS</th>
            <th class="arch">Architecture</th>
            <th>Link</th>
        </tr>
    </thead>
    <tbody>
        <tr>
            {@render linux()}
            <td>ARM (aarch64)</td>
            {@render link('wireman-0.2.10-aarch64-unknown-linux-gnu.tar.gz')}
        </tr>
        <tr>
            {@render linux()}
            <td>x86 (i686)</td>
            {@render link('wireman-0.2.10-i686-unknown-linux-gnu.tar.gz')}
        </tr>
        <tr>
            {@render linux()}
            <td>x86_64</td>
            {@render link('wireman-0.2.10-x86_64-unknown-linux-gnu.tar.gz')}
        </tr>
        <tr>
            {@render macos()}
            <td>ARM (aarch64)</td>
            {@render link('wireman-0.2.10-aarch64-apple-darwin.tar.gz')}
        </tr>
        <tr>
            {@render macos()}
            <td>x86_64</td>
            {@render link('wireman-0.2.10-x86_64-apple-darwin.tar.gz')}
        </tr>
        <tr>
            {@render windows()}
            <td>x86_64</td>
            {@render link('wireman-0.2.10-x86_64-pc-windows-msvc.zip')}
        </tr>
    </tbody>
</table>

<style>
    th,
    td {
        padding: 12px;
        border: 1px solid #ddd;
        color: var(--gray-lightest);
        font-size: var(--font-size-code);
    }

    a {
        font-size: var(--font-size-code);
        color: var(--gray-lightest);
    }

    .os {
        width: 10%;
    }
    .arch {
        width: 25%;
    }

    td:last-child {
        width: 25%;
    }

    .os-container {
        display: flex;
        justify-content: center;
        align-items: center;
        gap: 6px;
    }
    table {
        width: 100%;
    }
</style>
