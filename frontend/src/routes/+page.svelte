<script lang="ts">
    import { Button } from '$lib/components/ui/button/index.js';
    import * as Alert from '$lib/components/ui/alert/index.js';
    import { toast } from 'svelte-sonner';

    function handleClick() {
        toast('Hello world!');
    }
</script>

<Button onclick={handleClick}>Click me</Button>

<Alert.Root>
    <Alert.Title>Heads up!</Alert.Title>
    <Alert.Description>You can add components to your app using the cli.</Alert.Description>
</Alert.Root>
