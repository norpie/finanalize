<script lang="ts">
	import type { PageProps } from './$types';
	let { data }: PageProps = $props();

	import { Button } from '$lib/components/ui/button/index.js';
	import { Badge } from '$lib/components/ui/badge/index.js';
	import { Progress } from '$lib/components/ui/progress/index.js';

	import Spinner from '$lib/components/spinner.svelte';

	import { get, post } from '$lib/request';
	import { onDestroy, onMount } from 'svelte';
	import { toast } from 'svelte-sonner';

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

	const endStatuses = ['Invalid', 'Done'];

	const knownStatuses = [
		'Pending',
		'Validation',
		'GenerateTitle',
		'GenerateSectionHeadings',
		'GenerateParagraphBullets',
		'GenerateSearchQueries',
		'SearchQueries',
		'ScrapeTopResults'
	];

	function progress(status: string) {
		return (knownStatuses.indexOf(status) / knownStatuses.length) * 100;
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
		await refreshReport();
		// Start timer to refresh the report every 5 seconds
		interval = setInterval(async () => {
			await refreshReport();
		}, 5000);
	});

	onDestroy(() => {
		if (interval) {
			clearInterval(interval);
		}
	});
</script>

<div class="m-4 p-4">
	<Button onclick={retry}>Retry</Button>
</div>
{#if report}
	<div class="m-4 mb-4 p-4">
		<h1 class="text-center text-2xl font-bold">{report.title ?? 'Untitled Report'}</h1>
		<div id="badges" class="flex flex-wrap gap-2">
			<Badge>{report.report.status}</Badge>
			<Badge>N/A Credits</Badge>
			<Badge>{created_at}</Badge>
			<Badge>Verdict: {verdict}</Badge>
		</div>
		<p>Requested subject: {report.report.user_input}</p>
		{#if verdict == 'Invalid'}
			<p>Verdict: {verdict}</p>
			<p>Justification: {report.verdict?.justification}</p>
		{/if}
		{#if report.report.status !== 'Done' && report.report.status !== 'Invalid'}
			<Progress value={progress(report.report.status)} max={100} class="w-[60%]" />
		{/if}
		<h2 class="mb-4 text-xl font-bold">Justification: {report.verdict?.justification}</h2>
		<h2 class="mb-4 text-xl font-bold">Headings:</h2>
		{#if report.headings}
			{#each report.headings as heading, i (i)}
				<h3 class="mb-4 text-lg font-bold">{heading.heading}</h3>
				{#each heading.paragraphs as paragraph, j (j)}
					<p class="mb-4">{paragraph}</p>
				{/each}
			{/each}
		{/if}
		<h2 class="mb-4 text-xl font-bold">Searches:</h2>
		{#if report.searches}
			{#each report.searches as search, i (i)}
				<p class="mb-4">{search.query}</p>
			{/each}
		{/if}
		<h2 class="mb-4 text-xl font-bold">Sources:</h2>
		{#if report.sources}
			{#each report.sources as source, i (i)}
				<p class="mb-4">{source.url}</p>
			{/each}
		{/if}
	</div>
{:else}
	<div class="flex h-screen w-full items-center justify-center">
		<Spinner />
	</div>
{/if}
