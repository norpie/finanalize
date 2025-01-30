<script lang="ts">
    import * as Breadcrumb from '$lib/components/ui/breadcrumb/index.js';
    import AppSidebar from '$lib/components/app-sidebar.svelte';
    import { Separator } from '$lib/components/ui/separator/index.js';
    import * as Sidebar from '$lib/components/ui/sidebar/index.js';

    import type User from '../../models/user';

    let {
        user = $bindable(),
        location,
        children
    }: { user: User; location: string; children: any } = $props();
</script>

<Sidebar.Provider>
    <AppSidebar bind:user />
    <Sidebar.Inset>
        <header class="flex h-16 shrink-0 items-center gap-2">
            <div class="flex items-center gap-2 px-4">
                <Sidebar.Trigger class="-ml-1" />
                <Separator orientation="vertical" class="mr-2 h-4" />
                <Breadcrumb.Root>
                    <Breadcrumb.List>
                        <Breadcrumb.Item>
                            <Breadcrumb.Page>{location}</Breadcrumb.Page>
                        </Breadcrumb.Item>
                    </Breadcrumb.List>
                </Breadcrumb.Root>
            </div>
        </header>
        {@render children()}
    </Sidebar.Inset>
</Sidebar.Provider>
