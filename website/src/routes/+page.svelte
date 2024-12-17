<script>
	import BinariesTable from '$lib/BinariesTable.svelte';
	import CodeContainer from '$lib/CodeContainer.svelte';
	import ContentCard from '$lib/ContentCard.svelte';
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
			<a
				href="https://crates.io/crates/wireman"
				class="custom-link"
				target="_blank"
				rel="noopener noreferrer"
				>crates.io
			</a>
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
			<a
				href="https://github.com/preiter93/wireman/releases"
				class="custom-link"
				target="_blank"
				rel="noopener noreferrer"
				>releases page
			</a>
			. Here are the binaries from the latest release:
		</p>
		<BinariesTable />
	</ContentCard>

	<h2>Configuration</h2>

	<ContentCard title="Setup" initiallyExpanded={true}>
		<p>
			After installing wireman run the following command, which will prompt you to specify an
			install directory (the config path) in which it will create a default configuration:
		</p>
		<CodeContainer command="wireman init" />
		<p style="padding-top:24px">
			If you've specified a non-default directory, don't forget to add the following environment
			variable to your .bashrc file:
		</p>
		<CodeContainer command="export WIREMAN_CONFIG_DIR=$HOME/.config/wireman" />
	</ContentCard>

	<ContentCard title="Configuring Wireman" initiallyExpanded={true}>
		<p>
			After initializing Wireman, you should have configuration file like <code
				>~/.config/wireman.toml</code
			>. The most important step is to specify proto includes and files.
		</p>
		<p>
			Say you have a project with protos <code>$HOME/my-project/services/order/api.proto</code>
			and
			<code>$HOME/my-project/services/price/api.proto</code> and your <code>api.proto</code>'s
			reference proto's from
			<code>$HOME/my-project/protos</code>, then your configuration file may look like this:
		</p>
		<CodeContainer
			command="includes = [
    '$HOME/my-project/services',
    '$HOME/my-project/protos'
]

files = [
    'order/api.proto',
    'price/api.proto'
]

[server]
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
disabled = false"
		/>
	</ContentCard>

	<h2>Basic Usage</h2>

	<div class="image-container">
		<img src="/images/wireman-selection.png" alt="Shows how to select a proto service" />
	</div>
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
		width: 850px;
		padding-bottom: 12px;
	}

	@media (max-width: 850px) {
		.content {
			width: 100%;
		}
	}

	code {
		background-color: var(--gray-darkest);
		color: var(--gray-lightest);
		border-radius: 4px;
		padding: 1px 1px;
		border: 1px solid var(--gray-light);
	}

	.image-container {
		margin: 0;
		width: 830px;
		box-sizing: border-box;
		padding: 12px;
		overflow: hidden;
	}

	img {
		width: 100%;
		height: auto;
		border: 1px solid var(--gray-light);
	}

	@media (max-width: 850px) {
		.image-container {
			width: 100%;
		}
	}
</style>
