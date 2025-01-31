<script lang="ts">
    import * as Card from '$lib/components/ui/card/index.js';
    import { Input } from '$lib/components/ui/input/index.js';
    import { Label } from '$lib/components/ui/label/index.js';

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
    <Sidebar bind:user location="Dashboard">
        <div class="flex h-[100%] w-full items-center justify-center px-4">
            <Card.Root class="mx-auto max-w-sm">
                <Card.Header>
                    <Card.Title class="text-2xl">Profile</Card.Title>
                    <Card.Description>This is you.</Card.Description>
                </Card.Header>
                <Card.Content>
                    <div class="grid gap-4">
                        <div class="grid gap-2">
                            <Label for="email">Id</Label>
                            <Input id="email" type="text" bind:value={user.id} required disabled />
                        </div>
                        <div class="grid gap-2">
                            <Label for="email">Email</Label>
                            <Input id="email" type="email" bind:value={user.email} required disabled />
                        </div>
                    </div>
                </Card.Content>
            </Card.Root>
        </div>
    </Sidebar>
{:else}
    <div class="flex h-screen w-full items-center justify-center">
        <Spinner />
    </div>
{/if}
