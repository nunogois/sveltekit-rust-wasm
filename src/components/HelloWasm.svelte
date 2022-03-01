<script lang="ts">
	import { onMount } from 'svelte'

	import init, { greet } from 'wasm'

	onMount(async () => {
		await init()
	})
</script>

<button class="bg-red-300 m-4 p-4 rounded text-lg" on:click={greet}>
	🦀 + 🕸 Hello WebAssembly!
</button>
