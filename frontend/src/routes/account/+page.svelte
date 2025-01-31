<script lang="ts">
    import Spinner from '$lib/components/spinner.svelte';
    import { Separator } from '$lib/components/ui/separator/index.js';
    import Button from '$lib/components/ui/button/button.svelte';
    import * as Card from '$lib/components/ui/card/index.js';
    import { Input } from '$lib/components/ui/input/index.js';
    import { Label } from '$lib/components/ui/label/index.js';

    import { user } from '$lib/store';

    let password1 = $state('');
    let password2 = $state('');
    let currentPassword = $state('');
</script>

<div class="flex h-[100%] w-full items-center justify-center p-12">
    {#if $user}
        <Card.Root class="h-full w-full justify-between p-8">
            <Card.Content>
                <Card.Title class="mb-4">Email</Card.Title>
                <div class="grid gap-4">
                    <div class="grid gap-2">
                        <Input id="email" type="email" bind:value={$user.email} required disabled />
                    </div>
                </div>
                <Separator class="my-8" />
                <Card.Title class="mb-4">Password</Card.Title>
                <div class="grid gap-4">
                    <div class="grid gap-2">
                        <Label for="password">Password</Label>
                        <Input id="password" type="password" bind:value={password1} required />
                    </div>
                    <div class="grid gap-2">
                        <Label for="confirm-password">Confirm Password</Label>
                        <Input id="confirm-password" type="password" bind:value={password2} required />
                    </div>
                    <div class="grid gap-2">
                        <Label for="current-password">Current Password</Label>
                        <Input id="current-password" type="password" bind:value={currentPassword} required />
                    </div>
                </div></Card.Content
            >
            <Card.Footer>
                <Button type="submit">Save</Button>
            </Card.Footer>
        </Card.Root>
    {:else}
        <Spinner />
    {/if}
</div>
