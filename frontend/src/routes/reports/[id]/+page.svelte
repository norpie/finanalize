<script lang="ts">
    import type {PageProps} from './$types';

    let {data}: PageProps = $props();

    import {Button} from '$lib/components/ui/button/index.js';
    import {Badge} from '$lib/components/ui/badge/index.js';
    import {Progress} from '$lib/components/ui/progress/index.js';
    import {ScrollArea} from '$lib/components/ui/scroll-area/index.js';
    import {Skeleton} from '$lib/components/ui/skeleton/index.js';

    import * as Card from '$lib/components/ui/card/index.js';

    import Spinner from '$lib/components/spinner.svelte';

    import {get, post} from '$lib/request';
    import {onDestroy, onMount} from 'svelte';
    import {toast} from 'svelte-sonner';

    let socket: WebSocket | null = $state(null);
    let shouldReconnect: boolean = true;
    let reconnectTimeout: number = 2000;

    function connectSocket() {
        if (!data.id || socket) return;
        const url = import.meta.env.VITE_BACKEND_URL;
        const token = localStorage.getItem('token');
        if(!token){
            console.error("No authentication token found");
            return;
        }
        socket = new WebSocket(`${url}/v1/unprotected/live/reports/${data.id}?bearer=${token}`);
        socket.onopen = (event) => {
            console.log(`Socket opened for ${data.id}: `, event);
        }
        socket.onmessage = (event) => {
            const data = JSON.parse(event.data);
            console.log(data);
        }
        socket.onerror = (event) => {
            console.error(event);
        }
        socket.onclose = (event) => {
            console.log(`Socket closed for ${data.id}: `, event);
            socket = null;
            if (shouldReconnect) {
                setTimeout(() => connectSocket(), reconnectTimeout);
            }
        }
    }

    function disconnectSocket() {
        console.log("Are you sure about that?");
        shouldReconnect = false;
        if(socket){
            socket.close();
            socket = null;
        }
    }

    async function retry() {
        let res = await post(`v1/protected/reports/${data.id}/retry`, {});
        if (res.result) {
            toast.success('Report has been retried');
        } else {
            toast.error('Failed to retry report');
        }
    }

    interface FullReport {
        report: {
            id: string;
            user_input: string;
            status: string;
            created_at: string;
            updated_at: string;
        };
        verdict: { valid: boolean; justification: string } | undefined;
        title: string | undefined;
        headings: { heading: string; paragraphs: string[] }[] | undefined;
        searches: { query: string }[] | undefined;
        sources: { url: string }[] | undefined;
    }

    const startStatuses = ['Pending'];
    const endStatuses = ['Invalid', 'Done'];

    const knownStatuses = [
        'Validation',
        'GenerateTitle',
        'GenerateSectionHeadings',
        'GenerateParagraphBullets',
        'GenerateSearchQueries',
        'SearchQueries',
        'ScrapeTopResults'
    ]

    function statusColor(status: string) {
        if (startStatuses.includes(status)) {
            return 'bg-grey-500';
        } else if (knownStatuses.includes(status)) {
            return 'bg-yellow-500';
        } else if (status === 'Done') {
            return 'bg-green-500';
        } else if (status === 'Invalid') {
            return 'bg-red-500';
        } else {
            return 'bg-blue-500';
        }
    };

    function progress(status: string) {
        return (knownStatuses.indexOf(status) / knownStatuses.length) * 100;
    }

    function verdictColor(verdict: string) {
        if (verdict === 'Valid') {
            return 'bg-green-500';
        } else if (verdict === 'Invalid') {
            return 'bg-red-500';
        } else {
            return 'bg-yellow-500';
        }
    }

    let created_at = $state(new Date());
    let verdict = $state('N/A');

    $effect(() => {
        if (report) {
            if (report.report) {
                created_at = new Date(report.report.created_at);
            }
            if (report.verdict) {
                verdict = report.verdict.valid ? 'Valid' : 'Invalid';
            }
        }
    });

    let report: FullReport | undefined = $state();
    let interval: number | null = $state(null);

    async function refreshReport() {
        const result = await get<FullReport>(`v1/protected/reports/${data.id}`);
        if (result.result) {
            report = result.result;
            // toast.success('Report has been refreshed');
        } else {
            toast.error('Failed to refresh report');
        }
    }

    onMount(async () => {
        //await refreshReport();
        // Start timer to refresh the report every 5 seconds
        // interval = setInterval(async () => {
        //     await refreshReport();
        // }, 5000);
        shouldReconnect = true;
        connectSocket();
    });

    onDestroy(() => {
        if (interval) {
            clearInterval(interval);
        }
        disconnectSocket();
    });
</script>

<div class="m-4 p-4">
    <Button onclick={retry}>Retry</Button>
</div>
{#if report}
    <div class="flex flex-row">
        <div class="m-4 mb-4 max-w-[50%] p-4">
            <h1 class="text-center text-2xl font-bold">{report.title ?? 'Untitled Report'}</h1>
            <div id="badges" class="flex flex-wrap gap-2">
                <Badge class={statusColor(report.report.status)}>{report.report.status}</Badge>
                <Badge>N/A Credits</Badge>
                <Badge>{created_at}</Badge>
                <Badge class={verdictColor(verdict)}>Verdict: {verdict}</Badge>
            </div>
            <p>Requested subject: {report.report.user_input}</p>
            {#if verdict == 'Invalid'}
                <p>Verdict: {verdict}</p>
                <p>Justification: {report.verdict?.justification}</p>
            {/if}
            {#if report.report.status !== 'Done' && report.report.status !== 'Invalid'}
                <Progress value={progress(report.report.status)} max={100} class="mb-4 w-[100%]"/>
            {/if}
            <Card.Root>
                <Card.Content class="p-4">
                    {#if report.headings && report.headings.length !== 0}
                        {#each report.headings as heading, i (i)}
                            <h3 class="mb-4 text-lg font-bold">{heading.heading}</h3>
                            {#each heading.paragraphs as paragraph, j (j)}
                                <p class="mb-4">{paragraph}</p>
                            {/each}
                        {/each}
                    {:else}
                        {#each Array(3) as _, i}
                            <Skeleton class="mb-2 h-6 w-[500px] p-6"/>
                            {#each Array(3) as _, j}
                                <Skeleton class="mb-1 h-4 w-[300px] p-4"/>
                                <Skeleton class="mb-1 h-4 w-[250px] p-4"/>
                                <Skeleton class="mb-1 h-4 w-[200px] p-4"/>
                            {/each}
                        {/each}
                    {/if}
                </Card.Content>
            </Card.Root>
        </div>
        <div class="max-w-[50%] p-4">
            <Card.Root>
                <Card.Header>
                    <Card.Title>Search Queries</Card.Title>
                </Card.Header>
                <Card.Content>
                    {#if report.searches && report.searches.length !== 0}
                        <ScrollArea>
                            <ul class="list-inside list-disc">
                                {#each report.searches as search, i (i)}
                                    <li>{search.query}</li>
                                {/each}
                            </ul>
                        </ScrollArea>
                    {:else}
                        {#each Array(3) as _, i}
                            <Skeleton class="mb-1 h-4 w-[300px] p-4"/>
                            <Skeleton class="mb-1 h-4 w-[250px] p-4"/>
                            <Skeleton class="mb-1 h-4 w-[200px] p-4"/>
                        {/each}
                    {/if}
                </Card.Content>
            </Card.Root>
            <Card.Root>
                <Card.Header>
                    <Card.Title>Sources</Card.Title>
                </Card.Header>
                <Card.Content>
                    {#if report.sources && report.sources.length !== 0}
                        <ScrollArea>
                            <ul class="list-inside list-disc">
                                {#each report.sources as source, i (i)}
                                    <li>{source.url}</li>
                                {/each}
                            </ul>
                        </ScrollArea>
                    {:else}
                        {#each Array(3) as _, i}
                            <Skeleton class="mb-1 h-4 w-[300px] p-4"/>
                            <Skeleton class="mb-1 h-4 w-[250px] p-4"/>
                            <Skeleton class="mb-1 h-4 w-[200px] p-4"/>
                        {/each}
                    {/if}
                </Card.Content>
            </Card.Root>
        </div>
    </div>
{:else}
    <div class="flex h-screen w-full items-center justify-center">
        <Spinner/>
    </div>
{/if}
