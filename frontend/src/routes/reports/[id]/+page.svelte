<script lang="ts">
	import type { PageProps } from './$types';

	let { data }: PageProps = $props();

	import { Button } from '$lib/components/ui/button/index.js';
	import { Badge } from '$lib/components/ui/badge/index.js';
	import { Progress } from '$lib/components/ui/progress/index.js';
	import { ScrollArea } from '$lib/components/ui/scroll-area/index.js';
	import { Skeleton } from '$lib/components/ui/skeleton/index.js';
	import PdfViewer from 'svelte-pdf';

	import * as Card from '$lib/components/ui/card/index.js';

	import Spinner from '$lib/components/spinner.svelte';

	import { get, post } from '$lib/request';
	import { onDestroy, onMount } from 'svelte';
	import { toast } from 'svelte-sonner';

	let socket: WebSocket | null = $state(null);
	let shouldReconnect: boolean = true;
	let reconnectTimeout: number = 2000;

	function connectSocket() {
		if (!data.id || socket) return;
		const url = import.meta.env.VITE_BACKEND_URL;
		const token = localStorage.getItem('token');
		if (!token) {
			console.error('No authentication token found');
			return;
		}
		socket = new WebSocket(`${url}/v1/unprotected/live/reports/${data.id}?bearer=${token}`);
		socket.onopen = (event) => {
			console.log(`Socket opened for ${data.id}: `, event);
		};
		socket.onmessage = (event) => {
			const data = JSON.parse(event.data);
			console.log(data);
			report = data;
		};
		socket.onerror = (event) => {
			console.error(event);
		};
		socket.onclose = (event) => {
			console.log(`Socket closed for ${data.id}: `, event);
			socket = null;
			if (shouldReconnect) {
				setTimeout(() => connectSocket(), reconnectTimeout);
			}
		};
	}

	function disconnectSocket() {
		console.log('Are you sure about that?');
		shouldReconnect = false;
		if (socket) {
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

	interface FrontendReport {
		title?: string;
		user_input: string;
		status: string;
		error?: string;
		valid?: boolean;
		src?: string;
	}

	const startStatuses = ['Pending'];
	const endStatuses = ['Invalid', 'Done'];

	const knownStatuses = [
		'Validation',
		'GenerateTitle',
		'GenerateSectionNames',
		'GenerateSubSectionNames',
		'GenerateSubSectionQuestions',
		'GenerateSearchQueries',
		'SearchQueries',
		'ScrapeTopResults',
		'ExtractContent',
		'FormatContent',
		'ClassifyContent',
		'ChunkContent',
		'IndexChunks',
		'AnswerQuestions',
		'RenderLaTeXPdf'
	];

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
	}

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

	let verdict = $state('N/A');

	$effect(() => {
		if (report) {
			if (report.valid) {
				verdict = report.valid ? 'Valid' : 'Invalid';
			}
		}
	});

	let report: FrontendReport | undefined = $state();
	let interval: number | null = $state(null);

	async function refreshReport() {
		const result = await get<FrontendReport>(`v1/protected/reports/${data.id}`);
		if (result.result) {
			report = result.result;
			// toast.success('Report has been refreshed');
		} else {
			toast.error('Failed to refresh report');
		}
	}

	onMount(async () => {
		await refreshReport();
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
	<div class="align flex flex-row justify-center p-4">
		<Card.Root class="w-full max-w-3xl rounded-lg bg-gray-900 p-6 text-white shadow-lg">
			<!-- Report Title -->
			<Card.Title class="mb-4 text-xl font-bold text-white">
				Title: {report.title ?? 'Untitled Report'}
			</Card.Title>

			<Card.Content>
				<div class="mb-4 flex flex-wrap items-center gap-3">
					<Badge class={'rounded-full px-3 py-1 text-sm font-medium ' + statusColor(report.status)}>
						{report.status}
					</Badge>
					<Badge class={'rounded-full px-3 py-1 text-sm font-medium ' + verdictColor(verdict)}>
						Verdict: {verdict}
					</Badge>
				</div>

				<p class="mb-2 text-sm text-gray-300">
					<span class="font-semibold">Requested subject:</span>
					{report.user_input}
				</p>

				{#if verdict === 'Invalid'}
					<p class="text-sm text-red-400">
						<span class="font-semibold">Justification:</span>
						{report.error}
					</p>
				{/if}

				{#if report.status !== 'Done' && report.status !== 'Invalid'}
					<div class="relative mt-4 h-2.5 w-full rounded-full bg-gray-700">
						<div
							class="h-2.5 rounded-full bg-blue-500 transition-all"
							style="width: {progress(report.status)}%"
						></div>

						<div
							class="absolute inset-0 flex items-center justify-center text-xs font-semibold text-white"
						>
							{Math.round(progress(report.status))}%
						</div>
					</div>
				{/if}
			</Card.Content>
		</Card.Root>
	</div>
	{#if report.status === 'Done'}
		<PdfViewer
			url={`${import.meta.env.VITE_BACKEND_URL}/v1/unprotected/reports/${data.id}/document.pdf`}
		/>
	{/if}
{:else}
	<div class="flex h-screen w-full items-center justify-center">
		<Spinner />
	</div>
{/if}
