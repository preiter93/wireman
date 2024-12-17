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
	}

	@media (max-width: 850px) {
		.content {
			width: 100%;
		}
	}
</style>
