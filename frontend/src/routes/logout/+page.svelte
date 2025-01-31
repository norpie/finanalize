<script lang="ts">
    import Spinner from '$lib/components/spinner.svelte';

    import { goto } from '$app/navigation';
    import { post } from '$lib/request';
    import { toast } from 'svelte-sonner';
    import { onMount } from 'svelte';

    import { token } from '$lib/store';

    async function logout() {
        await post('v1/protected/logout', {});
        token.set(undefined);
        toast.success('Logged out');
        goto('/login');
    }

    onMount(async () => {
        await logout();
        goto('/');
    });
</script>

<Spinner />
