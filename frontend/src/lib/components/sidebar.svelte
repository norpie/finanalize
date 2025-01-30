<script lang="ts">
    import * as Breadcrumb from '$lib/components/ui/breadcrumb/index.js';
    import AppSidebar from '$lib/components/app-sidebar.svelte';
    import { Separator } from '$lib/components/ui/separator/index.js';
    import * as Sidebar from '$lib/components/ui/sidebar/index.js';

    import type User from '../../models/user';

    let { user = $bindable(), children }: { user: User; children: any } = $props();
</script>

<Sidebar.Provider>
    <AppSidebar />
    <Sidebar.Inset>
        <header class="flex h-16 shrink-0 items-center gap-2">
            <div class="flex items-center gap-2 px-4">
                <Sidebar.Trigger class="-ml-1" />
                <Separator orientation="vertical" class="mr-2 h-4" />
                <Breadcrumb.Root>
                    <Breadcrumb.List>
                        <Breadcrumb.Item class="hidden md:block">
                            <Breadcrumb.Link href="#">Building Your Application</Breadcrumb.Link>
                        </Breadcrumb.Item>
                        <Breadcrumb.Separator class="hidden md:block" />
                        <Breadcrumb.Item>
                            <Breadcrumb.Page>Data Fetching</Breadcrumb.Page>
                        </Breadcrumb.Item>
                    </Breadcrumb.List>
                </Breadcrumb.Root>
            </div>
        </header>
        {@render children()}
    </Sidebar.Inset>
</Sidebar.Provider>
