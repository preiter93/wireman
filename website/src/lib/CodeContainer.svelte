<script lang="ts">
	let { command }: { command: string } = $props();

	function copyCommand() {
		navigator.clipboard.writeText(command);
	}
</script>

<div class="container">
	<pre>{command}</pre>
	<button class="button" onclick={copyCommand} aria-label="Copy code">
		<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 -960 960 960" aria-hidden="true">
			<path
				d="M360-240q-33 0-56.5-23.5T280-320v-480q0-33 23.5-56.5T360-880h360q33 0 56.5 23.5T800-800v480q0 33-23.5 56.5T720-240H360Zm0-80h360v-480H360v480ZM200-80q-33 0-56.5-23.5T120-160v-560h80v560h440v80H200Zm160-240v-480 480Z"
			/>
		</svg>
	</button>
</div>

<style>
	.container {
		font-family: var(--font-family);
		font-size: var(--font-size-code);
		font-weight: var(--font-weight-code);
		color: var(--gray-lightest);
		text-align: left;
		background-color: var(--gray-darkest);
		border: 1px solid var(--gray-light);
		border-radius: 4px;
		padding: 6px 0px 6px 6px;
		display: flex;
		justify-content: space-between;
		align-items: center;
		box-shadow: 0px 0px 2px var(--gray-light);
		overflow-x: auto;
		gap: 4px;
	}

	button {
		all: unset;
		align-self: flex-start;
		position: sticky;
		right: 0px;
		background-color: var(--gray-darkest);
		padding: 6px;
	}

	svg {
		fill: var(--gray-lightest);
		height: var(--icon-size);
		width: var(--icon-size);
	}

	svg:hover {
		cursor: pointer;
		transform: scale(1.1);
	}

	svg:active {
		fill: var(--green);
	}

	pre {
		margin: 0;
	}
</style>
