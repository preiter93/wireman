<script>
    import LinuxIcon from '#lib/icons/LinuxIcon.svelte';
    import MacOSIcon from '#lib/icons/MacOSIcon.svelte';
    import WindowsIcon from '#lib/icons/WindowsIcon.svelte';
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
        <a href="https://github.com/preiter93/wireman/releases/download/v0.3.8/{name}" download>
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
            {@render link('wireman-0.3.8-aarch64-unknown-linux-gnu.tar.gz')}
        </tr>
        <tr>
            {@render linux()}
            <td>x86 (i686)</td>
            {@render link('wireman-0.3.8-i686-unknown-linux-gnu.tar.gz')}
        </tr>
        <tr>
            {@render linux()}
            <td>x86_64</td>
            {@render link('wireman-0.3.8-x86_64-unknown-linux-gnu.tar.gz')}
        </tr>
        <tr>
            {@render macos()}
            <td>ARM (aarch64)</td>
            {@render link('wireman-0.3.8-aarch64-apple-darwin.tar.gz')}
        </tr>
        <tr>
            {@render macos()}
            <td>x86_64</td>
            {@render link('wireman-0.3.8-x86_64-apple-darwin.tar.gz')}
        </tr>
        <tr>
            {@render windows()}
            <td>x86_64</td>
            {@render link('wireman-0.3.8-x86_64-pc-windows-msvc.zip')}
        </tr>
    </tbody>
</table>

<style>
    table {
        width: 100%;
        margin-top: 4px;
        border-collapse: separate;
        border-spacing: 0;
        border: 1px solid hsla(215, 18%, 65%, 0.18);
        border-radius: 8px;
        overflow: hidden;
        background-color: hsla(215, 21%, 5%, 0.8);
    }

    th,
    td {
        padding: 10px 12px;
        color: var(--gray-lightest);
        font-size: var(--font-size-code);
        text-align: left;
        border-bottom: 1px solid hsla(215, 18%, 65%, 0.1);
    }

    tbody tr:last-child td {
        border-bottom: none;
    }

    th {
        color: var(--gray-medium);
        font-weight: 500;
        text-transform: lowercase;
        border-bottom-color: hsla(215, 18%, 65%, 0.2);
    }

    tbody tr {
        transition: background-color 0.15s ease;
    }

    tbody tr:hover {
        background: hsla(36, 100%, 50%, 0.05);
    }

    a {
        font-size: var(--font-size-code);
        color: var(--gray-light);
        text-decoration: none;
        word-break: break-all;
    }

    a:hover {
        color: var(--orange);
        text-decoration: underline;
    }

    .os {
        width: 15%;
    }
    .arch {
        width: 22%;
    }

    .os-container {
        display: flex;
        align-items: center;
        gap: 8px;
    }
</style>
