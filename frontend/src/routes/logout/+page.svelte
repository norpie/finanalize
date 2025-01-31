<script lang="ts">
    import Spinner from '$lib/components/spinner.svelte';

    import { goto } from '$app/navigation';
    import { post } from '$lib/request';
    import { toast } from 'svelte-sonner';
    import { onMount } from 'svelte';

    async function logout() {
        await post('v1/protected/logout', {});
        localStorage.removeItem('accessToken');
        toast.success('Logged out');
        goto('/login');
    }

    onMount(async () => {
        await logout();
        goto('/');
    });
</script>

<Spinner />
