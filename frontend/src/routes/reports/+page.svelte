<script lang="ts">
    import Sidebar from '$lib/components/sidebar.svelte';

    import type User from '../../models/user';
    import { get } from '$lib/request';

    import { onMount } from 'svelte';
    import Spinner from '$lib/components/spinner.svelte';

    let user: User | undefined = $state<User>();

    onMount(async () => {
        user = (await get<User>('v1/protected/me')).result;
    });
</script>

{#if user}
    <Sidebar bind:user location="Reports">
        <div class="flex flex-1 flex-col gap-4 p-4">
            <div class="grid auto-rows-min gap-4 md:grid-cols-3">
                <div class="aspect-video rounded-xl bg-muted/50"></div>
                <div class="aspect-video rounded-xl bg-muted/50"></div>
                <div class="aspect-video rounded-xl bg-muted/50"></div>
            </div>
            <div class="min-h-[100vh] flex-1 rounded-xl bg-muted/50 md:min-h-min"></div>
        </div>
    </Sidebar>
{:else}
    <div class="flex h-screen w-full items-center justify-center">
        <Spinner />
    </div>
{/if}
