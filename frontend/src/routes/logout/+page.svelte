<script lang="ts">
    import Spinner from '$lib/components/spinner.svelte';

    import { goto } from '$app/navigation';
    import { post } from '$lib/request';
    import { toast } from 'svelte-sonner';
    import { onMount } from 'svelte';
    import { user } from '$lib/store';

    async function logout() {
        await post('v1/protected/logout', {});
        toast.success('Logged out');
        user.set(null);
        localStorage.removeItem('token');
        goto('/login');
    }

    onMount(async () => {
        await logout();
        goto('/');
    });
</script>

<Spinner />
