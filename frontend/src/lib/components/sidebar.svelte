<script lang="ts">
    import * as Breadcrumb from '$lib/components/ui/breadcrumb/index.js';
    import AppSidebar from '$lib/components/app-sidebar.svelte';
    import { Separator } from '$lib/components/ui/separator/index.js';
    import * as Sidebar from '$lib/components/ui/sidebar/index.js';

    import { page } from '$app/state';

    let { children }: { children: any } = $props();

    function capitalize(pageName: string): string {
        if (pageName.length === 0) return 'Home';
        return pageName.charAt(0).toUpperCase() + pageName.slice(1);
    }

    function lastSegment(path: string): string {
        let split = path.split('/');
        let popped = split.pop();
        if (!popped) return 'Home';
        return popped;
    }

    let pageName = $derived(capitalize(lastSegment(page.url.pathname)));
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
                        <Breadcrumb.Item>
                            <Breadcrumb.Page>{pageName}</Breadcrumb.Page>
                        </Breadcrumb.Item>
                    </Breadcrumb.List>
                </Breadcrumb.Root>
            </div>
        </header>
        {@render children()}
    </Sidebar.Inset>
</Sidebar.Provider>
