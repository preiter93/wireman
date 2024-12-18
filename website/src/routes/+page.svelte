<script>
    import BinariesTable from '$lib/BinariesTable.svelte';
    import CodeContainer from '$lib/CodeContainer.svelte';
    import ContentCard from '$lib/ContentCard.svelte';

    import ImgHelp from '$lib/assets/website_001.png';
    import ImgService from '$lib/assets/website_026.png';
    import ImgMethod from '$lib/assets/website_051.png';
    import ImgAuth from '$lib/assets/website_170.png';
    import ImgRequest from '$lib/assets/website_225.png';
    import ImgResponse from '$lib/assets/website_300.png';
    import ImgCopy from '$lib/assets/website_301.png';

    $effect(() => {
        // Preload all images to avoid jumping animations when
        // expanding the cards for the first time.
        const imageSources = [ImgHelp, ImgService, ImgMethod, ImgAuth, ImgRequest, ImgResponse, ImgCopy];

        imageSources.forEach((src) => {
            const img = new Image();
            img.src = src;
        });
    });
</script>

<div>
    <h1>Wireman</h1>
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

    <h2>Setup</h2>

    <ContentCard title="Initialization" initiallyExpanded={true}>
        <p>
            After installing wireman, execute the following command, which prompts you to specify an installation directory (the
            configuration path) in which a default configuration is created:
        </p>
        <CodeContainer command="wireman init" />
        <!-- <p style="padding-top:24px"> -->
        <!-- 	If you've specified a non-default directory, don't forget to add the following environment -->
        <!-- 	variable to your .bashrc file: -->
        <!-- </p> -->
        <!-- <CodeContainer command="export WIREMAN_CONFIG_DIR=$HOME/.config/wireman" /> -->
    </ContentCard>

    <ContentCard title="Configuration" initiallyExpanded={true}>
        <p>
            After initializing wireman, the most important step is to specify proto includes and files. Say you have a project with the
            following structure
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

    <ContentCard title="Advanced Options">
        <p>
            Wireman is very customizable. You can change the default configuration directory by exporting the following environment
            variable:
        </p>
        <CodeContainer command="export WIREMAN_CONFIG_DIR=$HOME/.config/wireman" />
        <p style="padding-top:22px">
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
    </ContentCard>

    <h2>Usage</h2>
    <ContentCard title="Start wireman" initiallyExpanded={true}>
        <p style="padding-bottom:0">
            You can now start <code>wireman</code> from the terminal:
        </p>
        <CodeContainer command="wireman" />
    </ContentCard>

    <ContentCard title="Open help">
        <div class="img-container">
            <p>
                Open the help dialog by pressing <code>?</code>. It provides key mappings for each page to help you navigate.
            </p>
            <img src={ImgHelp} alt="The help dialog" />
        </div>
    </ContentCard>

    <ContentCard title="Select service">
        <div class="img-container">
            <p>
                Get started by selecting a proto service. Use <code>j/k</code> to scroll and
                <code>Enter</code> to select a service.
            </p>
            <img src={ImgService} alt="Select a service" />
        </div>
    </ContentCard>

    <ContentCard title="Select method">
        <div class="img-container">
            <p>
                Similar to selecting a service, use <code>j/k</code> to scroll through the methods and
                <code>Enter</code>
                to select. Press <code>Tab</code> to go to the next step.
            </p>
            <img src={ImgMethod} alt="Select a method" />
        </div>
    </ContentCard>

    <ContentCard title="Set address and headers">
        <div class="img-container">
            <p>On this page, set the server address and metadata headers.</p>
            <p>
                Use shell scripts for dynamic authorization headers by enclosing them in <code>$()</code>, e.g.
                <code>$(getToken.sh)</code>
                . This approach is more secure than hardcoded credentials. Press <code>Tab</code> to continue to the next step.
            </p>
            <img src={ImgAuth} alt="Set address and headers" />
        </div>
    </ContentCard>

    <ContentCard title="Send request">
        <div class="img-container">
            <p>This page displays both the request and response messages. Begin by editing the request.</p>

            <p>
                The editor uses vim motions. Navigate with <code>h/j/k/l</code>, start editing with
                <code>i</code> and go back to normal mode with <code>Esc</code>. For more details about the editor, visit
                <a href="https://github.com/preiter93/edtui" class="custom-link" target="_blank" rel="noopener noreferrer">edtui </a>. Press
                <code>Enter</code> to send the request to your server.
            </p>
            <img src={ImgRequest} alt="Send request" />
        </div>
    </ContentCard>

    <ContentCard title="Copy response">
        <div class="img-container">
            <p>
                Switch between the request and response editors using <code>J/K</code>. Copy the response with
                <code>VGy</code> to your clipboard.
            </p>
            <img src={ImgCopy} alt="Copy response" />
        </div>
    </ContentCard>

    <ContentCard title="Saved history">
        <div class="img-container">
            <p>
                After you have sent the request, request and headers are saved automatically. This is indicated visually by the highlighted <code
                    >1</code
                >. The next time you open wireman, the data will still be available. You can switch between save buffers by typing the
                corresponding numbers
                <code>1-5</code>.
            </p>
            <img src={ImgResponse} alt="Saved history" />
        </div>
    </ContentCard>
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

    @media (max-width: 850px) {
        .content {
            width: 100%;
        }
    }

    pre {
        text-align: left;
        color: var(--gray-lightest);
        font-size: var(--font-size-code);
    }

    .img-container {
        display: flex;
        flex-direction: column;
        justify-content: center;
        align-items: center;
    }

    p {
        align-self: flex-start;
    }

    img {
        width: 85%;
        height: 100%;
        display: block;
    }

    @media (max-width: 600px) {
        img {
            width: 100%;
        }
    }
</style>
