<script lang="ts">
	import { onMount } from 'svelte'
	import init, { greet, User, users_load, users_new, users_update, users_delete } from 'wasm'

	let loading = true
	let error
	let users = [] as User[]
	let editId

	onMount(async () => {
		await init()
		try {
			users = await users_load()
		} catch (e) {
			error = e.message
		}
		loading = false
	})

	let newName = ''
	let newEmail = ''

	function createUser() {
		if (newName !== '' && newEmail !== '') {
			users_new(new User(newName, newEmail)).then(newUser => {
				users = [...users, newUser]
			})
			newName = ''
			newEmail = ''
		}
	}

	function editUser(id: string, name: string, email: string) {
		editId = id
		newName = name
		newEmail = email
	}

	function updateUser(id: string) {
		if (newName !== '' && newEmail !== '') {
			users_update(id, new User(newName, newEmail)).then(newUser => {
				users = users.map(user => (user.id === newUser.id ? newUser : user))
			})
			newName = ''
			newEmail = ''
			editId = undefined
		}
	}

	function deleteUser(id: string) {
		users_delete(id)
		users = users.filter(u => u.id !== id)
		if (editId === id) {
			newName = ''
			newEmail = ''
			editId = undefined
		}
	}
</script>

<button class="bg-red-300 m-4 p-4 rounded text-lg" on:click={greet}>
	🦀 + 🕸 Hello WebAssembly!
</button>

<div class="flex flex-col items-center w-full p-2">
	<h2 class="text-xl font-bold pb-1">
		{loading ? 'Loading...' : error ? `Error: ${error}` : 'Users'}
	</h2>
	{#if !loading && !error}
		<div class="flex flex-col p-3 my-1 mb-3 bg-indigo-300 rounded max-w-md w-full gap-2">
			<h3 class="text-lg font-bold text-center">
				{editId ? `Edit user ${editId}` : 'Create new user'}
			</h3>
			<div class="flex justify-between">
				<span class="font-bold w-20">Name: </span>
				<input
					class="bg-transparent outline outline-1 rounded px-2 py-1 text-sm w-full"
					bind:value={newName}
				/>
			</div>
			<div class="flex justify-between">
				<span class="font-bold w-20">Email: </span>
				<input
					class="bg-transparent outline outline-1 rounded px-2 py-1 text-sm w-full"
					bind:value={newEmail}
				/>
			</div>
			<button
				class="outline outline-1 rounded p-1 bg-indigo-200"
				on:click={() => (editId ? updateUser(editId) : createUser())}
				>✅ {editId ? 'Save Changes' : 'Create'}</button
			>
		</div>
	{/if}
	{#each users as { id, name, email }}
		<div class="flex flex-col p-3 my-1 bg-indigo-300 rounded max-w-md w-full">
			<div>
				<span class="font-bold">ID: </span>{id}
			</div>
			<div>
				<span class="font-bold">Name: </span>{name}
			</div>
			<div>
				<span class="font-bold">Email: </span>{email}
			</div>
			<div class="flex justify-between mt-2">
				<button
					class="outline outline-1 rounded p-1 bg-indigo-200"
					on:click={() => editUser(id, name, email)}>✏ Edit</button
				>
				<button class="outline outline-1 rounded p-1 bg-indigo-200" on:click={() => deleteUser(id)}
					>❌ Delete</button
				>
			</div>
		</div>
	{/each}
</div>
