<script>
    import CodeContainer from '#lib/CodeContainer.svelte';
    import ContentCard from '#lib/ContentCard.svelte';
    import GithubButton from '#lib/GithubButton.svelte';
    import InstallTabs from '#lib/InstallTabs.svelte';
    import Tour from '#lib/Tour.svelte';
    import MoreFeatures from '#lib/MoreFeatures.svelte';
    import ImgHero from '#lib/assets/website_326.png';
</script>

<nav class="topbar">
    <a class="brand" href="#top"><span class="prompt">❯</span> wireman</a>
    <div class="nav-links">
        <a href="#install">install</a>
        <a href="#getting-started">start</a>
        <a href="#tour">tour</a>
        <a href="#features">features</a>
        <a href="#reference">reference</a>
    </div>
    <GithubButton />
</nav>

<header class="hero" id="top">
    <h1>
        <span class="prompt" aria-hidden="true">❯</span><span class="name">wireman</span><span class="cursor" aria-hidden="true"></span>
    </h1>
    <p class="subtitle">A grpc client for the terminal</p>
    <div class="hero-install">
        <CodeContainer command="cargo install wireman" />
    </div>
    <div class="window">
        <div class="window-bar">wireman — OrderService &gt; GetOrder</div>
        <img src={ImgHero} alt="Wireman showing a gRPC response" width="830" height="570" />
    </div>
</header>

<div class="content">
    <h2 id="install">Installation</h2>
    <InstallTabs />

    <h2 id="getting-started">Getting Started</h2>

    <ol class="steps">
        <li class="step">
            <div class="step-body">
                <h3><span class="step-number">1.</span> Initialize</h3>
                <p>Run the following command. It asks for a configuration directory and creates a default configuration in it:</p>
                <CodeContainer command="wireman init" />
            </div>
        </li>
        <li class="step">
            <div class="step-body">
                <h3><span class="step-number">2.</span> Configure</h3>
                <p>Tell wireman where your protos are. Say you have a project with the following structure:</p>
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
            </div>
        </li>
        <li class="step">
            <div class="step-body">
                <h3><span class="step-number">3.</span> Run</h3>
                <p>Start wireman from the terminal:</p>
                <CodeContainer command="wireman" />
            </div>
        </li>
    </ol>

    <h2 id="tour">A Tour of Wireman</h2>

    <Tour></Tour>

    <h2 id="features">More Features</h2>

    <MoreFeatures></MoreFeatures>

    <h2>Fits Your Workflow</h2>

    <div class="grid">
        <ContentCard title="Keyboard or mouse" initiallyExpanded={true} hideToggle={true}>
            <p>
                Navigate with vim-style keybindings and hit <code>?</code> to see what's available. Prefer the mouse? That works too, and
                <code>Ctrl+e</code> opens any field in your system editor.
            </p>
        </ContentCard>

        <ContentCard title="Neovim" initiallyExpanded={true} hideToggle={true}>
            <p>
                Run wireman in a floating terminal inside nvim with
                <a href="https://github.com/preiter93/wireman.nvim" target="_blank" rel="noopener noreferrer">wireman.nvim</a>
                <img
                    src="https://upload.wikimedia.org/wikipedia/commons/3/3a/Neovim-mark.svg"
                    alt="Neovim logo"
                    width="14"
                    height="14"
                    style="vertical-align: middle;"
                />.
            </p>
        </ContentCard>
    </div>

    <ContentCard title="Command line" initiallyExpanded={true} hideToggle={true}>
        <p>
            Send one request without opening the TUI. It uses the protos, default address, auth header and TLS settings from your
            configuration and prints the response as JSON:
        </p>
        <CodeContainer command={`wireman call proto.Greeter.SayHello -a 'http://localhost:50051' -d '{"name": "wireman"}'`} />
        <p style="padding-top:18px">
            Add headers with <code>-H "key: value"</code>. Without <code>-a &lt;address&gt;</code>, the default address from your
            configuration is used. In the request tab,
            <code>Alt+y</code> copies the current request as such a command to share. Commands like <code>$(getToken.sh)</code> stay as they
            are, so whoever runs it uses their own token.
        </p>
    </ContentCard>

    <ContentCard title="AI agents (MCP)" initiallyExpanded={true} hideToggle={true}>
        <p>
            <code>wireman mcp</code> lets AI agents like Claude Code use your gRPC services through the
            <a href="https://modelcontextprotocol.io" target="_blank" rel="noopener noreferrer">Model Context Protocol</a>. The agent starts
            wireman itself, so there is no server to run. For Claude Code:
        </p>
        <CodeContainer command="claude mcp add wireman -- wireman mcp" />
        <p style="padding-top:18px">
            The agent can list your services, describe a method's request as JSON Schema, send a request, or return it as a
            <code>wireman call</code> command for you to run. It uses the protos, default address, auth header and TLS settings from your
            configuration; add <code>--config &lt;path&gt;</code> after <code>mcp</code> to use another one. Requests go to your real servers,
            so keep your agent's approval of tool calls turned on.
        </p>
    </ContentCard>

    <h2 id="reference">Advanced & Reference</h2>

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
            <li>
                <code>call &lt;METHOD&gt;</code> - Sends one request without the TUI and prints the response as JSON. Flags:
                <code>-d, --data &lt;JSON&gt;</code> (defaults to <code>{'{}'}</code>), <code>-H, --header &lt;"key: value"&gt;</code>
                (can be repeated), <code>-a, --address &lt;ADDRESS&gt;</code>.
            </li>
            <li><code>mcp</code> - Runs an MCP server on stdin and stdout, so AI agents can list, describe and call your gRPC methods.</li>
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
</div>

<footer class="footer">
    <span>wireman v0.3.9</span>
    <span class="footer-links">
        <a href="https://github.com/preiter93/wireman" target="_blank" rel="noopener noreferrer">github</a>
        <a href="https://crates.io/crates/wireman" target="_blank" rel="noopener noreferrer">crates.io</a>
        <a href="https://github.com/preiter93/wireman.nvim" target="_blank" rel="noopener noreferrer">wireman.nvim</a>
    </span>
</footer>

<style>
    .topbar {
        position: sticky;
        top: 0;
        z-index: 10;
        width: 100%;
        display: flex;
        align-items: center;
        gap: 24px;
        padding: 10px max(16px, calc((100% - var(--max-width)) / 2));
        background: hsla(215, 21%, 7%, 0.92);
        border-bottom: 1px solid hsla(215, 18%, 65%, 0.12);
        --icon-size-large: 20px;
    }

    .topbar a {
        font-size: var(--font-size-code);
        text-decoration: none;
        color: var(--gray-light);
    }

    .topbar a:hover {
        color: var(--orange);
    }

    .topbar .brand {
        font-weight: 700;
        color: var(--gray-lightest);
    }

    .nav-links {
        flex: 1;
        display: flex;
        gap: 18px;
    }

    .hero {
        display: flex;
        flex-direction: column;
        align-items: center;
        width: var(--max-width);
        padding: 48px 10px 0 10px;
        scroll-margin-top: 60px;
    }

    .hero h1 {
        display: flex;
        align-items: center;
        gap: 0.35em;
        margin: 0;
        letter-spacing: -1px;
    }

    .prompt {
        color: var(--orange);
        font-weight: 500;
    }

    .name {
        color: var(--gray-lightest);
    }

    .cursor {
        display: inline-block;
        width: 0.5em;
        height: 0.9em;
        margin-left: -0.25em;
        background: var(--orange);
        animation: blink 1.1s steps(1) infinite;
    }

    @keyframes blink {
        50% {
            opacity: 0;
        }
    }

    @media (prefers-reduced-motion: reduce) {
        .cursor {
            animation: none;
        }
    }

    .subtitle {
        align-self: center;
        text-align: center;
        margin: 8px 0 0 0;
        color: var(--gray-light);
        font-size: calc(var(--font-size-body) + 1px);
    }

    .subtitle::before {
        content: '// ';
        color: var(--gray-medium);
    }

    .hero-install {
        width: 100%;
        max-width: 380px;
        margin: 28px 0 40px 0;
    }

    .window {
        width: 100%;
        max-width: 600px;
        border: 1px solid hsla(215, 18%, 65%, 0.2);
        border-radius: 8px;
        overflow: hidden;
        background: var(--gray-dark);
        box-shadow: 0 24px 60px rgba(0, 0, 0, 0.45);
    }

    .window-bar {
        padding: 8px 14px;
        font-size: 11px;
        color: var(--gray-medium);
        text-align: center;
        border-bottom: 1px solid hsla(215, 18%, 65%, 0.14);
    }

    .window img {
        display: block;
        width: 100%;
        height: auto;
    }

    p {
        margin-bottom: 1rem;
        align-self: flex-start;
    }

    .content {
        width: var(--max-width);
        padding-bottom: 12px;
    }

    .content h2 {
        scroll-margin-top: 60px;
    }

    .steps {
        list-style: none;
        margin: 12px 10px 0 10px;
        padding: 0;
    }

    .step {
        margin: 0;
        padding-bottom: 28px;
    }

    .step-number {
        color: var(--gray-medium);
        font-weight: 400;
    }

    .step-body h3 {
        margin: 0 0 10px 0;
        font-size: var(--font-size-h3);
    }

    .step-body p {
        margin-bottom: 12px;
    }

    .grid {
        display: grid;
        grid-template-columns: 1fr 1fr;
        align-items: stretch;
    }

    .grid :global(.content-card) {
        height: calc(100% - 12px);
    }

    .footer {
        width: var(--max-width);
        display: flex;
        justify-content: space-between;
        gap: 16px;
        flex-wrap: wrap;
        margin-top: 64px;
        padding: 20px 10px 32px 10px;
        border-top: 1px solid hsla(215, 18%, 65%, 0.12);
        font-size: var(--font-size-code);
        color: var(--gray-medium);
    }

    .footer-links {
        display: flex;
        gap: 18px;
    }

    .footer a {
        font-size: var(--font-size-code);
        color: var(--gray-light);
        text-decoration: none;
    }

    .footer a:hover {
        color: var(--orange);
    }

    @media (max-width: 960px) {
        .content,
        .hero,
        .footer {
            width: 100%;
        }
    }

    @media (max-width: 600px) {
        .topbar {
            flex-wrap: wrap;
            justify-content: space-between;
            row-gap: 6px;
        }

        .nav-links {
            order: 3;
            flex-basis: 100%;
            gap: 16px;
            overflow-x: auto;
            scrollbar-width: none;
        }

        .nav-links::-webkit-scrollbar {
            display: none;
        }

        .nav-links a {
            flex-shrink: 0;
            padding: 4px 0;
        }

        .hero,
        .content h2 {
            scroll-margin-top: 88px;
        }

        .hero {
            padding: 28px 16px 0 16px;
        }

        .steps {
            margin: 12px 6px 0 6px;
        }

        .grid {
            grid-template-columns: 1fr;
        }

        .footer {
            padding: 20px 16px 32px 16px;
        }
    }

    pre {
        font-family: var(--font-family-mono);
        font-size: var(--font-size-code);
        font-weight: var(--font-weight-code);
        color: var(--gray-light);
        text-align: left;
        background-color: hsla(215, 21%, 5%, 0.8);
        border: 1px solid hsla(215, 18%, 65%, 0.18);
        border-radius: 8px;
        padding: 12px 16px;
        margin: 0 0 12px 0;
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
</style>
