<script>
    import BinariesTable from '$lib/BinariesTable.svelte';
    import CodeContainer from '$lib/CodeContainer.svelte';
    import ContentCard from '$lib/ContentCard.svelte';
    import GithubButton from '$lib/GithubButton.svelte';
    import Tour from '$lib/Tour.svelte';
    import MoreFeatures from '$lib/MoreFeatures.svelte';
</script>

<div class="header">
    <h1>Wireman</h1>
    <div class="github">
        <GithubButton />
    </div>
</div>
<div>
    <p class="subtitle">A grpc client for the terminal</p>
</div>

<div class="content">
    <h2>Installation</h2>
    <ContentCard title="Cargo" initiallyExpanded={true}>
        <p>
            If you have Rust installed you can install wireman from
            <a href="https://crates.io/crates/wireman" class="custom-link" target="_blank" rel="noopener noreferrer">crates.io </a>
            with cargo:
        </p>
        <CodeContainer command="cargo install wireman" />
    </ContentCard>

    <ContentCard title="Brew">
        <p>On linux and macOS you can install wireman with brew:</p>
        <CodeContainer command="brew install preiter93/wireman/wireman" />
    </ContentCard>

    <ContentCard title="Binary Releases">
        <p>
            Alternatively you can download a pre-compiled binary from the
            <a href="https://github.com/preiter93/wireman/releases" class="custom-link" target="_blank" rel="noopener noreferrer"
                >releases page
            </a>
            . Here are the binaries from the latest release:
        </p>
        <BinariesTable />
    </ContentCard>

    <h2>Getting Started</h2>

    <ContentCard title="Initialization" initiallyExpanded={true}>
        <p>
            After installing wireman, execute the following command, which prompts you to specify an installation directory (the
            configuration path) in which a default configuration is created:
        </p>
        <CodeContainer command="wireman init" />
    </ContentCard>

    <ContentCard title="Configuration" initiallyExpanded={true}>
        <p>
            After initializing wireman, the most important step is to specify proto includes and files. Say you have a project with the
            following structure:
        </p>
        <pre>
$HOME/my-project/
├── services/
│   ├── order/
│   │   └── api.proto
│   └── price/
│       └── api.proto
└── protos/common.proto
</pre>
        <p>Then modify <code>~/.config/wireman/wireman.toml</code>:</p>
        <CodeContainer
            command="includes = [
    '$HOME/my-project/services',
    '$HOME/my-project/protos'
]

files = [
    'order/api.proto',
    'price/api.proto'
]"
        />
    </ContentCard>

    <ContentCard title="Start wireman" initiallyExpanded={true}>
        <p style="padding-bottom:0">
            You can now start <code>wireman</code> from the terminal:
        </p>
        <CodeContainer command="wireman" />
    </ContentCard>

    <ContentCard title="Advanced configuration">
        <p>
            Wireman is very customizable. You can change the default configuration directory by exporting the following environment
            variable:
        </p>
        <CodeContainer command="export WIREMAN_CONFIG_DIR=$HOME/.config/wireman" />
        <p style="padding-top:18px">
            The <code>wireman.toml</code> provides the following configuration options:
        </p>
        <CodeContainer
            command="[server]
# Optional. Prefills the server address.
default_address = 'http://localhost:50051'

# Optional. Prefills the auth header.
default_auth_header = 'Bearer $(getToken.sh)'

[history]
# Optional. Defaults to $WIREMAN_CONFIG_DIR/history.
directory = '$WIREMAN_CONFIG_DIR/history'

# Optional. Autosaves history on request. Defaults to true.
autosave = true

# Optional. History is enabled by default.
disabled = false

[logging]
# Optional. Defaults to $WIREMAN_CONFIG_DIR.
directory = '$WIREMAN_CONFIG_DIR'

# Optional. Defaults to Debug.
level = 'Debug'

[ui]
# Optional. Set a UI theme.
skin = '$WIREMAN_CONFIG_DIR/skins/dracula.toml'"
        />
        <p style="padding-top: 18px;">
            If you are interested in configuring wireman with your own theme, check out the custom
            <a
                href="https://github.com/preiter93/wireman/tree/main/wireman-theme/assets"
                class="custom-link"
                target="_blank"
                rel="noopener noreferrer"
                >skins
            </a>.
        </p>
    </ContentCard>

    <ContentCard title="Command line flags" initiallyExpanded={false}>
        <h3>Commands</h3>
        <ul>
            <li><code>check</code> - Runs a health check and displays configuration details to ensure proper setup.</li>
            <li><code>init</code> - Sets up Wireman by creating a default configuration file.</li>
        </ul>
        <h3>Options</h3>
        <ul>
            <li>
                <code>-c, --config &lt;CONFIG&gt;</code> - Specifies an optional path to a configuration file. If not provided, Wireman uses
                the default configuration path.
            </li>
            <li><code>-l, --local-protos</code> - Reads the proto's from the current directory. Ignores the config.</li>
        </ul>
    </ContentCard>

    <h2>A Tour of Wireman</h2>

    <Tour></Tour>

    <h2>More Features</h2>

    <MoreFeatures></MoreFeatures>
</div>

<style>
    .subtitle {
        margin: 0;
        font-style: italic;
    }

    p {
        margin-bottom: 1rem;
    }

    .content {
        width: var(--max-width);
        padding-bottom: 12px;
    }

    pre {
        font-family: var(--font-family-mono);
        font-size: var(--font-size-code);
        font-weight: var(--font-weight-code);
        color: var(--gray-lightest);
        text-align: left;
        background-color: var(--gray-darkest);
        border: 1px solid var(--gray-light);
        border-radius: 8px;
        padding: 12px 16px;
        margin: 16px 0;
        overflow-x: auto;
        line-height: 1.5;
    }
    h3,
    ul {
        text-align: left;
        color: var(--gray-lightest);
        font-size: var(--font-size-code);
    }

    li {
        margin: 10px 0;
    }

    .header {
        display: flex;
        justify-content: center;
        align-items: center;
        width: var(--max-width);
        position: relative;
    }

    .github {
        position: absolute;
        right: 0;
        margin-right: 20px;
        padding-top: 20px;
    }

    @media (max-width: 600px) {
        .github {
            margin-right: 16px;
            padding-top: 16px;
        }
    }

    .subtitle {
        margin: 0;
        font-style: italic;
    }

    p {
        margin-bottom: 1rem;
    }

    .content {
        width: var(--max-width);
        padding-bottom: 12px;
    }

    @media (max-width: 800px) {
        .content {
            width: 100%;
        }
        .header {
            width: 100%;
        }
    }

    p {
        align-self: flex-start;
    }
</style>
