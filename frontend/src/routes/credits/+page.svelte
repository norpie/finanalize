<script lang="ts">
    import * as Table from '$lib/components/ui/table/index.js';
    import Spinner from '$lib/components/spinner.svelte';

    interface LedgerEntry {
        id: string;
        description: string;
        date: string;
        incoming: boolean;
        amount: number;
    }

    interface Ledger {
        balance: number;
        entries: LedgerEntry[];
    }

    let ledger: Ledger = $state({
        balance: 2039487,
        entries: [
            {
                id: 'laisknucrb09324',
                description: 'Added credits to account',
                date: '2022-09-01',
                incoming: true,
                amount: 13241
            },
            {
                id: 'psNH8c7rb9w03',
                description: 'Purchased report: "Apple Inc. During COVID-19"',
                date: '2022-09-10',
                incoming: false,
                amount: 13241
            },
            {
                id: 'nsa9pd8h796231',
                description: 'Added credits to account',
                date: '2022-10-28',
                incoming: true,
                amount: 15350
            },
            {
                id: 'nc32764bcaw',
                description:
                    'Purchased report: "Effects of Territorial Disputes in the South China Sea on Semiconductor Industry"',
                date: '2022-11-08',
                incoming: false,
                amount: 15350
            },
            {
                id: 'n8902734cb0j',
                description: 'Added credits to account',
                date: '2023-01-09',
                incoming: true,
                amount: 15350
            },
            {
                id: '890c74n2309asdh',
                description: 'Purchased report: "Tesla Inc. Q3 2022 Earnings Report"',
                date: '2024-07-19',
                incoming: false,
                amount: 15350
            }
        ]
    });

    function entryCostClass(entry: LedgerEntry) {
        return entry.incoming ? 'text-right text-green-500' : 'text-right text-red-500';
    }
</script>

<div class="flex flex-1 flex-col gap-4 p-4">
    <div class="grid auto-rows-min gap-4 md:grid-cols-3">
        <h2>You have {ledger.balance} credits</h2>
    </div>
    <div class="min-h-[100vh] flex-1 rounded-xl bg-muted/50 md:min-h-min">
        <Table.Root>
            <Table.Caption>A list of your recent reports.</Table.Caption>
            <Table.Header>
                <Table.Row>
                    <Table.Head class="w-[100px]">Date</Table.Head>
                    <Table.Head>Description</Table.Head>
                    <Table.Head class="text-right">Amount</Table.Head>
                </Table.Row>
            </Table.Header>
            <Table.Body>
                {#each ledger.entries as entry, i (i)}
                    <Table.Row>
                        <Table.Cell class="font-medium">{entry.date}</Table.Cell>
                        <Table.Cell>{entry.description}</Table.Cell>
                        <Table.Cell class={entryCostClass(entry)}>{entry.amount}</Table.Cell>
                    </Table.Row>
                {/each}
            </Table.Body>
        </Table.Root>
    </div>
</div>

<!-- <div class="flex h-screen w-full items-center justify-center"> -->
<!--     <Spinner /> -->
<!-- </div> -->
