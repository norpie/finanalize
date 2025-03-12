<script lang="ts">
    import * as Table from '$lib/components/ui/table/index.js';

    import Spinner from '$lib/components/spinner.svelte';
    import {user} from '$lib/store';

    // TODO: Fetch user data from the server
    const credits = 129847;
    const reports = [
        {
            id: 'laisknucrb09324',
            title: 'Apple Inc. During COVID-19',
            date: '2022-09-01',
            status: 'Paid',
            cost: 13241
        },
        {
            id: 'psNH8c7rb9w03',
            title: 'Effects of Territorial Disputes in the South China Sea on Semiconductor Industry',
            date: '2022-09-10',
            status: 'Payment Pending',
            cost: 15350
        },
        {
            id: 'nsa9pd8h796231',
            title: 'Tesla Inc. Q3 2022 Earnings Report',
            date: '2022-10-28',
            status: 'Generating',
            cost: null
        },
        {
            id: 'nc32764bcaw',
            title: 'The Future of Electric Vehicles and the companies leading the charge',
            date: '2022-11-08',
            status: 'Researching',
            cost: null
        },
        {
            id: 'n8902734cb0j',
            title: 'Performance of banks during periods of economic uncertainty',
            date: '2023-01-09',
            status: 'Validating',
            cost: null
        },
        {
            id: '890c74n2309asdh',
            title: "Credit Suisse's end and the UBS takeover",
            date: '2024-07-19',
            status: 'Pending',
            cost: null
        }
    ].toReversed();

    function nameFromEmail(email: string) {
        return email.split('@')[0];
    }
</script>

{#if $user}
    <div class="flex flex-1 flex-col gap-4 p-4">
        <div class="grid auto-rows-min gap-4 md:grid-cols-3">
            <div class="aspect-video rounded-xl bg-muted/50 flex items-center justify-center">
                <p class="text-2xl text-center">
                    👋Hi,
                    <span class="text-2xl font-bold">{nameFromEmail($user.email)}</span>
                    !
                </p>
            </div>
            <div class="aspect-video rounded-xl bg-muted/50 flex items-center justify-center">
                <p class="text-2xl text-center">
                    You have
                    <span class="text-2xl font-bold">{credits}</span>
                    credits remaining.
                </p>
            </div>
            <div class="aspect-video rounded-xl bg-muted/50 flex items-center justify-center">
                <a href="/reports">
                    <p class="text-2xl text-center">
                        <span class="text-2xl font-bold text-white hover:text-[#9333ea] transition-all">View Reports</span>
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
                            <Table.Cell>{report.date}</Table.Cell>
                            <Table.Cell>{report.title}</Table.Cell>
                            <Table.Cell class="text-right"
                            >
                                {#if report.cost}{report.cost} Credits{:else}Unavailable{/if}
                            </Table.Cell
                            >
                        </Table.Row>
                    {/each}
                </Table.Body>
            </Table.Root>
        </div>
    </div>
{:else}
    <div class="flex h-screen w-full items-center justify-center">
        <Spinner/>
    </div>
{/if}
