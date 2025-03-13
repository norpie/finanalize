<script lang="ts">
	import * as Table from '$lib/components/ui/table/index.js';

	import Spinner from '$lib/components/spinner.svelte';
	import { user } from '$lib/store';
	import { get, getWalletBalance } from '$lib/request';
	import { onMount } from 'svelte';

	// TODO: Fetch user data from the server
	let credits = $state(0);
	let reports: {
		id: string;
		user_input: string;
		created_at: string;
		status: string;
		cost: string;
	}[] = $state([]);

	function nameFromEmail(email: string) {
		return email.split('@')[0];
	}

	onMount(async () => {
		credits = (await getWalletBalance()).result;
		reports = (await get<Report[]>('v1/protected/reports?page=0&perPage=20')).result;
	});
</script>

{#if $user}
	<div class="flex flex-1 flex-col gap-4 p-4">
		<div class="grid auto-rows-min gap-4 md:grid-cols-3">
			<div class="flex aspect-video items-center justify-center rounded-xl bg-muted/50">
				<p class="text-center text-2xl">
					👋Hi,
					<span class="text-2xl font-bold">{nameFromEmail($user.email)}</span>
					!
				</p>
			</div>
			<div class="flex aspect-video items-center justify-center rounded-xl bg-muted/50">
				<p class="text-center text-2xl">
					You have
					<span class="text-2xl font-bold">{credits}</span>
					credits remaining.
				</p>
			</div>
			<div class="flex aspect-video items-center justify-center rounded-xl bg-muted/50">
				<a href="/reports">
					<p class="text-center text-2xl">
						<span class="text-2xl font-bold text-white transition-all hover:text-[#9333ea]"
							>View Reports</span
						>
					</p>
				</a>
			</div>
		</div>
		<div class="min-h-[100vh] flex-1 rounded-xl bg-muted/50 md:min-h-min">
			<Table.Root>
				<Table.Caption>A list of your recent reports.</Table.Caption>
				<Table.Header>
					<Table.Row>
						<Table.Head class="w-[100px]">Status</Table.Head>
						<Table.Head class="w-[100px]">Date</Table.Head>
						<Table.Head>Title</Table.Head>
						<Table.Head class="text-right">Cost</Table.Head>
					</Table.Row>
				</Table.Header>
				<Table.Body>
					{#each reports as report, i (i)}
						<Table.Row>
							<Table.Cell class="font-medium">{report.status}</Table.Cell>
							<Table.Cell>{report.created_at}</Table.Cell>
							<Table.Cell>
								<a href={`/reports/${report.id}`}>
									{report.user_input}
								</a></Table.Cell
							>
							<Table.Cell class="text-right">
								{#if report.cost}{report.cost} Credits{:else}Unavailable{/if}
							</Table.Cell>
						</Table.Row>
					{/each}
				</Table.Body>
			</Table.Root>
		</div>
	</div>
{:else}
	<div class="flex h-screen w-full items-center justify-center">
		<Spinner />
	</div>
{/if}
