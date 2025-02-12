<script lang="ts">
    import { Button } from '$lib/components/ui/button/index.js';
    import * as Card from '$lib/components/ui/card/index.js';
    import { Input } from '$lib/components/ui/input/index.js';
    import { Label } from '$lib/components/ui/label/index.js';
    import { user } from '$lib/store';
    import { toast } from 'svelte-sonner';
    import { post } from '$lib/request';
    import type User from '../../models/user';
    import { get } from '$lib/request';

    import { goto } from '$app/navigation';

    async function login() {
        const response = await post<{ access_token: string }>('v1/auth/login', {
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

        localStorage.setItem('token', response.result.access_token);
        toast.success('Logged in successfully');
        const newUser = (await get<User>('v1/protected/me')).result;
        user.set(newUser);
        goto('/dashboard');
    }

    let email = $state();
    let password = $state();

    
    
</script>

<div class="flex h-screen w-full items-center justify-center px-4">
    <Card.Root class="mx-auto max-w-sm">
        <Card.Header>
            <Card.Title class="text-2xl">Login</Card.Title>
            <Card.Description>Enter your email below to login to your account</Card.Description>
        </Card.Header>
        <Card.Content>
            <div class="grid gap-4">
                <div class="grid gap-2">
                    <Label for="email">Email</Label>
                    <Input id="email" type="email" placeholder="me@example.com" required bind:value={email} />
                </div>
                <div class="grid gap-2">
                    <div class="flex items-center">
                        <Label for="password">Password</Label>
                        <!-- <a href="##" class="ml-auto inline-block text-sm underline"> -->
                        <!--    Forgot your password? -->
                        <!-- </a> -->
                    </div>
                    <Input id="password" type="password" required bind:value={password} />
                </div>
                <Button class="w-full" onclick={login}>Login</Button>
                <!-- <Button variant="outline" class="w-full">Login with Google</Button> -->
            </div>
            <div class="mt-4 text-center text-sm">
                Don't have an account?
                <a href="/register" class="underline"> Register </a>
            </div>
        </Card.Content>
    </Card.Root>
</div>
