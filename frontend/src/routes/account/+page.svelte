<script lang="ts">
    import Spinner from '$lib/components/spinner.svelte';
    import * as Card from '$lib/components/ui/card/index.js';
    import { Input } from '$lib/components/ui/input/index.js';
    import { Label } from '$lib/components/ui/label/index.js';

    import { user } from '$lib/store';
</script>

<div class="flex h-[100%] w-full items-center justify-center px-4">
    {#if $user}
        <Card.Root class="mx-auto max-w-sm">
            <Card.Header>
                <Card.Title class="text-2xl">Account</Card.Title>
                <Card.Description>This is you.</Card.Description>
            </Card.Header>
            <Card.Content>
                <div class="grid gap-4">
                    <div class="grid gap-2">
                        <Label for="email">Id</Label>
                        <Input id="email" type="text" bind:value={$user.id} required disabled />
                    </div>
                    <div class="grid gap-2">
                        <Label for="email">Email</Label>
                        <Input id="email" type="email" bind:value={$user.email} required disabled />
                    </div>
                </div>
            </Card.Content>
        </Card.Root>
    {:else}
        <Spinner />
    {/if}
</div>
