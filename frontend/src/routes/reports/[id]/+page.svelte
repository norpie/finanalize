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

	let timerStart: number | null = $state(null);
	let elapsedTime: string = $state('00:00:00');
	let timerInterval: number | null = $state(null);

	function formatTime(seconds: number): string {
		const hrs = String(Math.floor(seconds / 3600)).padStart(2, '0');
		const mins = String(Math.floor((seconds % 3600) / 60)).padStart(2, '0');
		const secs = String(seconds % 60).padStart(2, '0');
		return `${hrs}:${mins}:${secs}`;
	}

	function startTimer() {
		const savedStartTime = localStorage.getItem(`reportStartTime_${data.id}`);

		if (savedStartTime) {
			timerStart = parseInt(savedStartTime, 10);
		} else {
			timerStart = Date.now();
			localStorage.setItem(`reportStartTime_${data.id}`, timerStart.toString());
		}

		if (timerInterval) clearInterval(timerInterval);

		// Update elapsedTime every second
		timerInterval = setInterval(() => {
			if (timerStart) {
				const secondsElapsed = Math.floor((Date.now() - timerStart) / 1000);
				elapsedTime = formatTime(secondsElapsed);
			}
		}, 1000);
	}

	function stopTimer() {
		if (timerInterval) {
			clearInterval(timerInterval);
			timerInterval = null;
		}

		// Remove the start time from localStorage when the report is completed
		localStorage.removeItem(`reportStartTime_${data.id}`);
	}

	// Status-specific timer variables
	let statusTimerStart: number | null = $state(null);
	let statusElapsedTime: string = $state('00:00:00');
	let statusTimerInterval: number | null = $state(null);
	let currentStatus: string | null = $state(null);

	// Status-specific Timer
	function startStatusTimer(newStatus: string) {
		if (currentStatus === newStatus) return; // Don't reset if status hasn't changed

		currentStatus = newStatus;

		const savedStatusStartTime = localStorage.getItem(
			`reportStatusStartTime_${data.id}_${currentStatus}`
		);

		if (savedStatusStartTime) {
			statusTimerStart = parseInt(savedStatusStartTime, 10);
		} else {
			statusTimerStart = Date.now();
			localStorage.setItem(
				`reportStatusStartTime_${data.id}_${currentStatus}`,
				statusTimerStart.toString()
			);
		}

		statusElapsedTime = '00:00:00';

		if (statusTimerInterval) clearInterval(statusTimerInterval);

		statusTimerInterval = setInterval(() => {
			if (statusTimerStart) {
				const secondsElapsed = Math.floor((Date.now() - statusTimerStart) / 1000);
				statusElapsedTime = formatTime(secondsElapsed);
			}
		}, 1000);
	}

	function stopStatusTimer() {
		if (statusTimerInterval) {
			clearInterval(statusTimerInterval);
			statusTimerInterval = null;
		}

		if (currentStatus) {
			localStorage.removeItem(`reportStatusStartTime_${data.id}_${currentStatus}`);
		}
	}

	// Detect status change
	$effect(() => {
		if (report && report.status) {
			startStatusTimer(report.status);
		}
	});

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

			if (report.status === 'Done') {
				stopTimer();
			}
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

	function getStatusLabel(status: string): string {
		const statusMapping: Record<string, string> = {
			Pending: 'Report Pending',
			Validation: 'Validating input',
			GenerateTitle: 'Generating title',
			GenerateSectionNames: 'Creating sections',
			GenerateSubSectionNames: 'Creating sub-sections',
			GenerateSubSectionQuestions: 'Generating subsection questions',
			GenerateSearchQueries: 'Generating search queries',
			SearchQueries: 'Searching queries',
			ScrapeTopResults: 'Scraping results',
			ExtractContent: 'Extracting content',
			FormatContent: 'Formatting content',
			ClassifyContent: 'Classifying content',
			ChunkContent: 'Chunking content',
			IndexChunks: 'Indexing chunks',
			AnswerQuestions: 'Answering questions',
			RenderLaTeXPdf: 'Rendering PDF',
			Done: 'Report generated',
			Invalid: 'Input was invalid'
		};

		return statusMapping[status] || status;
	}
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
			startTimer();
			// toast.success('Report has been refreshed');
		} else {
			toast.error('Failed to refresh report');
		}
	}

	onMount(async () => {
		await refreshReport();
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

<div class="m-4 flex justify-center space-y-4 p-4">
	<Button onclick={retry} class="mb-6 hover:bg-[#9333ea] hover:text-white">Retry</Button>

	{#if report}
		<div class="flex w-full flex-row justify-center p-4">
			<Card.Root
					class="flex w-full max-w-4xl flex-col items-center space-y-6 rounded-lg bg-sidebar backdrop-blur-md p-8 text-center text-white shadow-lg border border-gray-700 md:max-w-5xl lg:max-w-6xl xl:max-w-7xl"
			>
			<!-- Report Title -->
				<Card.Title class="mb-4 text-2xl font-bold text-white">
					Title: {report.title ?? 'Untitled Report'}
				</Card.Title>

				<div class="space-y-2">
					<p class="text-sm text-gray-300">
						<span class="font-semibold">Requested subject:</span>
						{report.user_input}
					</p>
					<p class="text-sm text-gray-300">
						<span class="font-semibold">Generation time:</span>
						{elapsedTime}
					</p>
				</div>

				<Card.Content class="flex w-full flex-col items-center space-y-6">
					<div class="mb-6 flex flex-wrap justify-center gap-4">
						<Badge
							class={'rounded-full px-3 py-1 text-sm font-medium ' + statusColor(report.status)}
						>
							{getStatusLabel(report.status)}
						</Badge>
						<Badge class={'rounded-full px-3 py-1 text-sm font-medium ' + verdictColor(verdict)}>
							Verdict: {verdict}
						</Badge>
					</div>

					{#if verdict === 'Invalid'}
						<div class="mb-4 w-full rounded-md bg-red-800 p-4">
							<p class="text-sm text-red-300">
								<span class="font-semibold">Justification:</span>
								{report.error}
							</p>
						</div>
					{/if}

					{#if report.status !== 'Done' && report.status !== 'Invalid'}
						<div class="mt-4 flex w-full flex-col items-center space-y-4">
							<!-- Progress Bar -->
							<div class="relative h-2.5 w-full rounded-full bg-gray-700">
								<div
									class="h-2.5 rounded-full bg-[#9333ea] transition-all"
									style="width: {progress(report.status)}%"
								></div>
							</div>

							<!-- Status Timer -->
							<p class="text-sm text-gray-300">
								<span class="font-semibold">{getStatusLabel(currentStatus)} started:</span>
								{statusElapsedTime}
							</p>
						</div>
					{/if}
				</Card.Content>
			</Card.Root>
		</div>

		{#if report.status === 'Done'}
			<div class="mt-6 flex justify-center">
				<a
					href={`${import.meta.env.VITE_BACKEND_URL}/v1/unprotected/reports/${data.id}/document.pdf`}
					target="_blank"
					rel="noopener noreferrer"
				>
					<Button class="mb-6 hover:bg-[#9333ea] hover:text-white">Download {report.title}</Button>
				</a>
			</div>
		{/if}
	{:else}
		<div class="flex h-screen w-full items-center justify-center">
			<Spinner />
		</div>
	{/if}
</div>
