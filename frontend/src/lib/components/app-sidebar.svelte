<script lang="ts">
    import NavMain from '$lib/components/nav-main.svelte';
    import NavProjects from '$lib/components/nav-projects.svelte';
    import NavSecondary from '$lib/components/nav-secondary.svelte';
    import NavUser from '$lib/components/nav-user.svelte';
    import * as Sidebar from '$lib/components/ui/sidebar/index.js';
    import TrendingUp from 'lucide-svelte/icons/trending-up';
    import type { ComponentProps } from 'svelte';

    let {
        ref = $bindable(null),
        user = $bindable(),
        ...restProps
    }: ComponentProps<typeof Sidebar.Root> = $props();
</script>

<Sidebar.Root bind:ref variant="inset" {...restProps}>
    <Sidebar.Header>
        <Sidebar.Menu>
            <Sidebar.MenuItem>
                <Sidebar.MenuButton size="lg">
                    {#snippet child({ props })}
                        <a href="##" {...props}>
                            <div
                                class="flex aspect-square size-8 items-center justify-center rounded-lg bg-sidebar-primary text-sidebar-primary-foreground"
                            >
                                <TrendingUp class="size-4" />
                            </div>
                            <div class="grid flex-1 text-left text-sm leading-tight">
                                <span class="truncate font-semibold">Finanalize</span>
                            </div>
                        </a>
                    {/snippet}
                </Sidebar.MenuButton>
            </Sidebar.MenuItem>
        </Sidebar.Menu>
    </Sidebar.Header>
    <Sidebar.Content>
        <!-- <NavMain items={data.navMain} /> -->
        <!-- <NavProjects projects={data.projects} /> -->
        <!-- <NavSecondary items={data.navSecondary} class="mt-auto" /> -->
    </Sidebar.Content>
    <Sidebar.Footer>
        <NavUser bind:user />
    </Sidebar.Footer>
</Sidebar.Root>
