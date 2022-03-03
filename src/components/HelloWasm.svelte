<script lang="ts">
	import { onMount } from 'svelte'

	import init, { greet, get_users, post_user, User } from 'wasm'

	onMount(async () => {
		await init()

		console.log('adding user...', await post_user(new User('Test', 'test@test.test')))
		console.log('users', await get_users())
	})
</script>

<button class="bg-red-300 m-4 p-4 rounded text-lg" on:click={greet}>
	🦀 + 🕸 Hello WebAssembly!
</button>
