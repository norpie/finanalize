<script lang="ts">
    import BadgeCheck from 'lucide-svelte/icons/badge-check';
    import Bell from 'lucide-svelte/icons/bell';
    import ChevronsUpDown from 'lucide-svelte/icons/chevrons-up-down';
    import CreditCard from 'lucide-svelte/icons/credit-card';
    import LogOut from 'lucide-svelte/icons/log-out';
    import Sparkles from 'lucide-svelte/icons/sparkles';

    import * as Avatar from '$lib/components/ui/avatar/index.js';
    import * as DropdownMenu from '$lib/components/ui/dropdown-menu/index.js';
    import * as Sidebar from '$lib/components/ui/sidebar/index.js';
    import { useSidebar } from '$lib/components/ui/sidebar/index.js';
    import type User from '../../models/user';
    import { post } from '$lib/request';
    import { toast } from 'svelte-sonner';
    import { goto } from '$app/navigation';

    let {
        user
    }: {
        user: User;
    } = $props();

    async function logout() {
        await post('v1/protected/logout', {});
        localStorage.removeItem('accessToken');
        toast.success('Logged out');
        goto('/login');
    }

    const sidebar = useSidebar();
</script>

<Sidebar.Menu>
    <Sidebar.MenuItem>
        <DropdownMenu.Root>
            <DropdownMenu.Trigger>
                {#snippet child({ props })}
                    <Sidebar.MenuButton
                        {...props}
                        size="lg"
                        class="data-[state=open]:bg-sidebar-accent data-[state=open]:text-sidebar-accent-foreground"
                    >
                        <Avatar.Root class="h-8 w-8 rounded-lg">
                            <Avatar.Image src={user.avatar} alt={user.name} />
                            <Avatar.Fallback class="rounded-lg">CN</Avatar.Fallback>
                        </Avatar.Root>
                        <div class="grid flex-1 text-left text-sm leading-tight">
                            <span class="text-semibold truncate">{user.email}</span>
                        </div>
                        <ChevronsUpDown class="ml-auto size-4" />
                    </Sidebar.MenuButton>
                {/snippet}
            </DropdownMenu.Trigger>
            <DropdownMenu.Content
                class="w-[--bits-dropdown-menu-anchor-width] min-w-56 rounded-lg"
                side={sidebar.isMobile ? 'bottom' : 'right'}
                align="end"
                sideOffset={4}
            >
                <DropdownMenu.Label class="p-0 font-normal">
                    <div class="flex items-center gap-2 px-1 py-1.5 text-left text-sm">
                        <Avatar.Root class="h-8 w-8 rounded-lg">
                            <Avatar.Image src={user.avatar} alt={user.name} />
                            <Avatar.Fallback class="rounded-lg">CN</Avatar.Fallback>
                        </Avatar.Root>
                        <div class="grid flex-1 text-left text-sm leading-tight">
                            <span class="text-semibold truncate">{user.email}</span>
                        </div>
                    </div>
                </DropdownMenu.Label>
                <DropdownMenu.Separator />
                <DropdownMenu.Item onclick={logout}>
                    <LogOut />
                    Log out
                </DropdownMenu.Item>
            </DropdownMenu.Content>
        </DropdownMenu.Root>
    </Sidebar.MenuItem>
</Sidebar.Menu>
