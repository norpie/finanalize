<script lang="ts">
    import * as Sidebar from '$lib/components/ui/sidebar/index.js';
    import { useSidebar } from '$lib/components/ui/sidebar/index.js';

    let {
        name,
        items,
    }: {
        name: string;
        item: {
            name: string;
            location: string;
            icon: any;
        }[];
    } = $props();

    const sidebar = useSidebar();
</script>

<Sidebar.Group class="group-data-[collapsible=icon]:hidden">
    <Sidebar.GroupLabel>{name}</Sidebar.GroupLabel>
    <Sidebar.Menu>
        {#each items as item (item.name)}
            <Sidebar.MenuItem>
                <Sidebar.MenuButton   class="text-white hover:text-[#9333ea] transition-all"  >
                    {#snippet child({ props })}
                        <a href={item.location} {...props}>
                            <item.icon />
                            <span>{item.name}</span>
                        </a>
                    {/snippet}
                </Sidebar.MenuButton>
            </Sidebar.MenuItem>
        {/each}
    </Sidebar.Menu>
</Sidebar.Group>
