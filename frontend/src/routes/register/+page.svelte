<script lang="ts">
    import { Button } from '$lib/components/ui/button/index.js';
    import * as Card from '$lib/components/ui/card/index.js';
    import { Input } from '$lib/components/ui/input/index.js';
    import { Label } from '$lib/components/ui/label/index.js';

    import { toast } from 'svelte-sonner';

    import { post } from '$lib/request';

    import { goto } from '$app/navigation';

    async function register() {
        if (password !== password2) {
            toast.error('Passwords do not match');
            return;
        }

        const response = await post<{ access_token: string }>('v1/auth/register', {
            email: email,
            password: password
        });

        if (response.error) {
            toast.error(response.error);
            return;
        }

        if (!response.result) {
            toast.error('An error occurred');
            return;
        }

        localStorage.setItem('accessToken', response.result.access_token);
        toast.success('Logged in successfully');
        goto('/');
    }

    let email = $state();
    let password = $state();
    let password2 = $state();
</script>

<div class="flex h-screen w-full items-center justify-center px-4">
    <Card.Root class="mx-auto max-w-sm">
        <Card.Header>
            <Card.Title class="text-2xl">Register</Card.Title>
            <Card.Description>Enter your email below to register your account</Card.Description>
        </Card.Header>
        <Card.Content>
            <div class="grid gap-4">
                <div class="grid gap-2">
                    <Label for="email">Email</Label>
                    <Input id="email" type="email" placeholder="m@example.com" bind:value={email} required />
                </div>
                <div class="grid gap-2">
                    <Label for="password">Password</Label>
                    <Input id="password" type="password" bind:value={password} required />
                    <Label for="password2">Confirm Password</Label>
                    <Input id="password2" type="password" bind:value={password2} required />
                </div>
                <Button class="w-full" onclick={register}>Register</Button>
                <!-- <Button variant="outline" class="w-full">Login with Google</Button> -->
            </div>
            <div class="mt-4 text-center text-sm">
                Already have an account?
                <a href="/login" class="underline"> Login </a>
            </div>
        </Card.Content>
    </Card.Root>
</div>
