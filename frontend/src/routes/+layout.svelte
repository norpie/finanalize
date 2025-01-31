<script lang="ts">
    import '../app.css';

    import { ModeWatcher } from 'mode-watcher';
    import { Toaster } from '$lib/components/ui/sonner/index.js';

    import Sidebar from '$lib/components/sidebar.svelte';
    import Spinner from '$lib/components/spinner.svelte';

    import { onMount } from 'svelte';

    import type User from '../models/user';

    import { get } from '$lib/request';

    import { user } from '$lib/store';
    import { page } from '$app/state';

    const hideSidebarRoutes = ['/', '/login', '/register'];

    let { children } = $props();

    onMount(async () => {
        const newUser = (await get<User>('v1/protected/me')).result;
        user.set(newUser);
    });
</script>

<ModeWatcher />
<Toaster />
{#if user}
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
