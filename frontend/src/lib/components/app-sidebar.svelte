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
                                <svg
                                    xmlns="http://www.w3.org/2000/svg"
                                    width="512"
                                    height="512"
                                    viewBox="0 0 24 24"
                                    fill="none"
                                    stroke="currentColor"
                                    stroke-width=""
                                    stroke-linecap="round"
                                    stroke-linejoin="round"
                                    class="lucide lucide-trending-up"
                                >
                                    <rect width="100%" height="100%" rx="6" stroke="none" fill="#9333ea" />
                                    <g transform="scale(0.70)" transform-origin="center">
                                        <polyline
                                            points="22 7 13.5 15.5 8.5 10.5 2 17"
                                            stroke="white"
                                            stroke-width="2.5"
                                        />
                                        <polyline points="16 7 22 7 22 13" stroke="white" stroke-width="2.5" />
                                    </g>
                                </svg>
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
