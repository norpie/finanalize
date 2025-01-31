<script lang="ts">
    import * as Avatar from '$lib/components/ui/avatar/index.js';
    import * as Sidebar from '$lib/components/ui/sidebar/index.js';
    import { useSidebar } from '$lib/components/ui/sidebar/index.js';

    import { user } from '$lib/store';

    const sidebar = useSidebar();
</script>

{#if $user}
    <Sidebar.Menu>
        <Sidebar.MenuItem>
            <div class="flex items-center gap-2 px-1 py-1.5 text-left text-sm">
                <Avatar.Root class="h-8 w-8 rounded-lg">
                    <Avatar.Fallback class="rounded-lg">{$user.email[0].toUpperCase()}</Avatar.Fallback>
                </Avatar.Root>
                <div class="grid flex-1 text-left text-sm leading-tight">
                    <span class="text-semibold truncate">{$user.email}</span>
                </div>
            </div>
        </Sidebar.MenuItem>
    </Sidebar.Menu>
{/if}
