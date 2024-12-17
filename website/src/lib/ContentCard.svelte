<script lang="ts">
	import type { Snippet } from 'svelte';
	import { slide } from 'svelte/transition';

	let {
		children,
		title,
		hideToggle = false,
		initiallyExpanded = false
	}: {
		children: Snippet;
		title: string;
		hideToggle?: boolean;
		initiallyExpanded?: boolean;
	} = $props();

	let isExpanded = $state(initiallyExpanded);

	function toggle() {
		isExpanded = !isExpanded;
	}
</script>

<div class="content-card">
	<div class="header {isExpanded ? 'true' : 'false'}">
		<h3>{title}</h3>
		{#if !hideToggle}
			<button onclick={toggle} class="chevron">
				<div>
					<svg
						class="chevron {isExpanded ? 'true' : 'false'}"
						xmlns="http://www.w3.org/2000/svg"
						viewBox="0 0 24 24"
					>
						<path class="chevron path" d="M12 15L6 9H18L12 15Z" />
						>
					</svg>
				</div>
			</button>
		{/if}
	</div>
	{#if isExpanded}
		<div class="content" transition:slide={{ duration: 500 }}>
			{@render children()}
		</div>
	{/if}
</div>

<style>
	.content-card {
		background-color: var(--gray-dark);
		border-radius: 8px;
		padding: 16px 20px 16px 20px;
		margin: 20px 16px 0px 16px;
		text-align: center;
		display: flex;
		flex-direction: column;
		justify-content: start;
	}

	@media (max-width: 600px) {
		.content-card {
			padding: 10px 16px 10px 16px;
			margin: 16px 10px 0px 10px;
		}
	}

	.header {
		display: flex;
		justify-content: space-between;
	}

	.content {
		margin-top: 1rem;
	}

	button {
		all: unset;
	}

	svg {
		fill: var(--gray-lightest);
		height: var(--icon-size);
		width: var(--icon-size);
	}

	.chevron {
		transition: transform 0.3s ease-in-out;
	}

	.chevron.false {
		transform: rotate(180deg);
	}

	.chevron.true {
		transform: rotate(0deg);
	}
</style>
