<script lang="ts">
	import * as Dialog from '$lib/components/ui/dialog/index.js';
	import * as Table from '$lib/components/ui/table/index.js';
	import * as Pagination from '$lib/components/ui/pagination/index.js';
	import { Textarea } from '$lib/components/ui/textarea/index.js';
	import { Button } from '$lib/components/ui/button/index.js';
	import { Badge } from '$lib/components/ui/badge/index.js';
	import ChevronLeft from 'lucide-svelte/icons/chevron-left';
	import ChevronRight from 'lucide-svelte/icons/chevron-right';
	import { get, post } from '$lib/request';
	import { onMount } from 'svelte';
	import Spinner from '$lib/components/spinner.svelte';
	import { toast } from 'svelte-sonner';
	import { goto } from '$app/navigation';

	interface Report {
		id: string;
		user_input: string;
		created_at: string;
		status: string;
	}

	let reports: Report[] = $state([]);

	function boop(page: number) {
		console.log(page);
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

	// async function getPage(page: number) {
	// 	let size = currentPage?.size ?? defaultSize;
	// 	const query = `v1/reports/?page=${page}&size=${size}`;
	// 	currentPage = (await get<ReportPage>(query)).result;
	// }

	function formatDate(date: Date): string {
		return date.toISOString().split('T')[0];
	}

	let newReportSubject: string = $state('');
	let dialogOpen = $state(false);

	async function newReport() {
		dialogOpen = false;
		let newReport = (
			await post<Report>('v1/protected/reports', {
				user_input: newReportSubject
			})
		).result;
		if (!newReport) {
			toast.error('Failed to create new report');
		}
		goto(`/reports/${newReport.id}`);
	}

	onMount(async () => {
		reports = (await get<Report[]>('v1/protected/reports?page=0&perPage=20')).result;
	});
</script>

<div class="flex flex-1 flex-col gap-4 p-4">
	<Dialog.Root bind:open={dialogOpen}>
		<Dialog.Trigger>New Report</Dialog.Trigger>
		<Dialog.Header>New Report</Dialog.Header>
		<Dialog.Content>
			<Dialog.Title>What is the subject of your report?</Dialog.Title>
			<Textarea class="mt-4 resize-none" bind:value={newReportSubject} />
			<Button onclick={newReport}>Submit</Button>
		</Dialog.Content>
	</Dialog.Root>
	<div class="min-h-[100vh] flex-1 rounded-xl bg-muted/50 md:min-h-min">
		{#if reports}
			<Table.Root>
				<Table.Header>
					<Table.Row>
						<Table.Head class="w-[100px]">Status</Table.Head>
						<Table.Head class="w-[100px]">Date</Table.Head>
						<Table.Head>User's Input</Table.Head>
						<Table.Head class="text-right">Cost</Table.Head>
					</Table.Row>
				</Table.Header>
				<Table.Body>
					{#each reports as report, i (i)}
						<Table.Row onclick={() => goto(`/reports/${report.id}`)}>
							<Table.Cell class="font-medium"
								><Badge class={statusColor(report.status)}>{report.status}</Badge></Table.Cell
							>
							<Table.Cell>{formatDate(new Date(report.created_at))}</Table.Cell>
							<Table.Cell>{report.user_input}</Table.Cell>
							<Table.Cell class="text-right"
								>{#if report.cost}{report.cost} Credits{:else}Unavailable{/if}</Table.Cell
							>
						</Table.Row>
					{/each}
				</Table.Body>
			</Table.Root>
			<Pagination.Root
				count={reports.length}
				perPage={reports.length}
				onPageChange={(page) => boop(page)}
			>
				{#snippet children({ pages, currentPage })}
					<Pagination.Content>
						<Pagination.Item>
							<Pagination.PrevButton>
								<ChevronLeft class="size-4" />
								<span class="hidden sm:block">Previous</span>
							</Pagination.PrevButton>
						</Pagination.Item>
						{#each pages as page (page.key)}
							{#if page.type === 'ellipsis'}
								<Pagination.Item>
									<Pagination.Ellipsis />
								</Pagination.Item>
							{:else}
								<Pagination.Item>
									<Pagination.Link {page} isActive={currentPage === page.value}>
										{page.value}
									</Pagination.Link>
								</Pagination.Item>
							{/if}
						{/each}
						<Pagination.Item>
							<Pagination.NextButton>
								<span class="hidden sm:block">Next</span>
								<ChevronRight class="size-4" />
							</Pagination.NextButton>
						</Pagination.Item>
					</Pagination.Content>
				{/snippet}
			</Pagination.Root>
		{:else}
			<div class="flex h-screen w-full items-center justify-center">
				<Spinner />
			</div>
		{/if}
	</div>
</div>
