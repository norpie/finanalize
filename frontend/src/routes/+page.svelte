<script lang="ts">
    import { Button } from '$lib/components/ui/button/index.js';
    import Spinner from '$lib/components/spinner.svelte';

    import { request } from '$lib/request';

    import { onMount } from 'svelte';
    import type User from '../models/user';
    import { goto } from '$app/navigation';

    let user: User | undefined | null = $state<User>();

    onMount(async () => {
        user = (await request<User>('GET', 'v1/protected/me', true)).result;
        if (user === undefined) {
            user = null;
        }
    });
</script>

{#if user}
    <Button onclick={() => goto('/dashboard')}>Dashboard</Button>
{:else if user === null}
    <Button onclick={() => goto('/login')}>Login</Button>
{:else if user === undefined}
    <Button><Spinner color="purple" /></Button>
{/if}
