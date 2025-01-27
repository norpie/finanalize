<script lang="ts">
    import { Button } from "$lib/components/ui/button/index.js";
    import * as Card from "$lib/components/ui/card/index.js";
    import { Input } from "$lib/components/ui/input/index.js";
    import { Label } from "$lib/components/ui/label/index.js";
    import { toast } from "svelte-sonner";
</script>

<div class="flex h-screen w-full items-center justify-center px-4">
<Card.Root class="mx-auto max-w-sm">
    <Card.Header>
        <Card.Title class="text-2xl">Register</Card.Title>
        <Card.Description
            >Enter your email below to register your account</Card.Description
        >
    </Card.Header>
    <Card.Content>
        <div class="grid gap-4">
            <div class="grid gap-2">
                <Label for="email">Email</Label>
                <Input
                    id="email"
                    type="email"
                    placeholder="m@example.com"
                    required
                />
            </div>
            <div class="grid gap-2">
                <Label for="password">Password</Label>
                <Input id="password" type="password" required />
                <Label for="password2">Confirm Password</Label>
                <Input id="password2" type="password" required />
            </div>
            <!-- <Button class="w-full" onclick={register}>Register</Button> -->
            <!-- <Button variant="outline" class="w-full">Login with Google</Button> -->
        </div>
        <div class="mt-4 text-center text-sm">
            Already have an account?
            <a href="##" class="underline" onclick={() => (current = "login")}>
                Sign in
            </a>
        </div>
    </Card.Content>
</Card.Root>
</div>