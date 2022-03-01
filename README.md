## Dev Log

1. Created SvelteKit project with `npm init svelte@next my-app`;
2. Added Tailwind CSS as per [these instructions](https://tailwindcss.com/docs/guides/sveltekit);
3. Created `wasm` package with `wasm-pack new wasm`;
4. Installed [vite-plugin-wasm-pack](https://github.com/nshen/vite-plugin-wasm-pack) and added `wasm` package in `svelte.config.js`;
5. Added relevant scripts to `package.json`;
6. Added Svelte component `hello_wasm.svelte` with Wasm example;

# create-svelte

Everything you need to build a Svelte project, powered by [`create-svelte`](https://github.com/sveltejs/kit/tree/master/packages/create-svelte).

## Creating a project

If you're seeing this, you've probably already done this step. Congrats!

```bash
# create a new project in the current directory
npm init svelte@next

# create a new project in my-app
npm init svelte@next my-app
```

> Note: the `@next` is temporary

## Developing

Once you've created a project and installed dependencies with `npm install` (or `pnpm install` or `yarn`), start a development server:

```bash
npm run dev

# or start the server and open the app in a new browser tab
npm run dev -- --open
```

## Building

To create a production version of your app:

```bash
npm run build
```

You can preview the production build with `npm run preview`.

> To deploy your app, you may need to install an [adapter](https://kit.svelte.dev/docs/adapters) for your target environment.
