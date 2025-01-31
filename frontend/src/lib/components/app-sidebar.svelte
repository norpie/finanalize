<script lang="ts">
    import NavProjects from '$lib/components/nav-projects.svelte';
    import NavUser from '$lib/components/nav-user.svelte';

    import * as Sidebar from '$lib/components/ui/sidebar/index.js';

    import TrendingUp from 'lucide-svelte/icons/trending-up';

    import LayoutDashboard from 'lucide-svelte/icons/layout-dashboard';
    import FileStack from 'lucide-svelte/icons/file-stack';

    import Coins from 'lucide-svelte/icons/coins';
    import User from 'lucide-svelte/icons/user';
    import LogOut from 'lucide-svelte/icons/log-out';

    let { ref = $bindable(null), user = $bindable(), ...restProps } = $props();

    const pages = [
        {
            name: 'Dashboard',
            location: '/dashboard',
            icon: LayoutDashboard
        },
        {
            name: 'Reports',
            location: '/reports',
            icon: FileStack
        }
    ];

    const account = [
        {
            name: 'Account',
            location: '/account',
            icon: User
        },
        {
            name: 'Credits',
            location: '/credits',
            icon: Coins
        },
        {
            name: 'Logout',
            location: '/logout',
            icon: LogOut
        }
    ];
</script>

<Sidebar.Root bind:ref variant="inset" {...restProps}>
    <Sidebar.Header>
        <Sidebar.Menu>
            <Sidebar.MenuItem>
                <Sidebar.MenuButton size="lg">
                    {#snippet child({ props })}
                        <div {...props}>
                            <div
                                class="flex aspect-square size-8 items-center justify-center rounded-lg bg-sidebar-primary text-sidebar-primary-foreground"
                            >
                                <TrendingUp class="size-4" />
                            </div>
                            <div class="grid flex-1 text-left text-sm leading-tight">
                                <span class="truncate font-semibold">Finanalize</span>
                            </div>
                        </div>
                    {/snippet}
                </Sidebar.MenuButton>
            </Sidebar.MenuItem>
        </Sidebar.Menu>
    </Sidebar.Header>
    <Sidebar.Content>
        <!-- <NavMain items={nav} /> -->
        <NavProjects name={'Navigation'} items={pages} />
    </Sidebar.Content>
    <Sidebar.Footer>
        <NavProjects name={'Account'} items={account} />
        <NavUser bind:user />
    </Sidebar.Footer>
</Sidebar.Root>
