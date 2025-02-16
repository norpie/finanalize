<script lang="ts">
	import '../app.css';

	import { ModeWatcher } from 'mode-watcher';
	import { Toaster } from '$lib/components/ui/sonner/index.js';

	import Sidebar from '$lib/components/sidebar.svelte';
	import Spinner from '$lib/components/spinner.svelte';

	import { onMount } from 'svelte';

	import type User from '../models/user';

	import { request } from '$lib/request';

	import { user } from '$lib/store';
	import { page } from '$app/state';

	const hideSidebarRoutes = ['/', '/login', '/register', '/about', '/contact'];

	let { children } = $props();

	onMount(async () => {
		const newUser = (await request<User>('GET', 'v1/protected/me', true)).result;
        console.log(newUser);
		user.set(newUser);
	});

    $inspect($user);
</script>

<ModeWatcher />
<Toaster />
{#if $user || $user === null}
	{#if !hideSidebarRoutes.includes(page.url.pathname)}
		<Sidebar>
			{@render children()}
		</Sidebar>
	{:else}
		{@render children()}
	{/if}
{:else}
	<div class="flex h-screen w-full items-center justify-center">
		<Spinner />
	</div>
{/if}
