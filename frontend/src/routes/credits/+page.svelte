<script lang="ts">
    import * as Table from '$lib/components/ui/table/index.js';
    import Spinner from '$lib/components/spinner.svelte';
    import { Button } from '$lib/components/ui/button/index.js';
    import { getWalletBalance, addCredits, getWalletTransactions } from '$lib/request.ts';
    import { toast } from 'svelte-sonner';

    let walletId = "your-wallet-id-here"; // Replace with actual wallet ID
    let balance = $state(0);
    let transactions = $state<LedgerEntry[]>([]); // Ensure this is an array of LedgerEntry type
    let loading = $state(false);

    // Ledger Entry Interface
    interface LedgerEntry {
        date: string;
        description: string;
        amount: number;
        incoming: boolean;
    }

    // Ledger Interface
    interface Ledger {
        balance: number;
        entries: LedgerEntry[];
    }

    async function loadWallet() {
    try {
        const walletData = await getWalletBalance(walletId);
        balance = walletData.result;

        const transactionData = await getWalletTransactions(walletId);
        if (transactionData.error) throw new Error(transactionData.error);


        transactions = transactionData.result.map((entry: any) => {
            const credit = entry.Credit || entry;  


            return {
                date: credit.date || new Date().toLocaleDateString(),  // Use current date if no date is found
                description: credit.description || 'No Description',  
                amount: credit.amount || 0,  // Ensure a valid amount
                incoming: parseFloat(credit.amount || 0) >= 0,  
            };
        });
    } catch (err) {
        toast.error(err.message);
    }
}




    async function handleAddCredits(amount: number) {
        console.log(`Adding ${amount} credits to wallet ${walletId}`);
        try {
            const result = await addCredits(walletId, amount);
            if (result.error) throw new Error(result.error);
            await loadWallet(); // Refresh wallet data after adding credits
        } catch (err) {
            toast.error(err.message);
        }
    }

    loadWallet();
</script>

<div class="flex flex-col gap-4 p-4 text-white">
    {#if loading}
        <Spinner />
    {:else}
        <h2 class="text-xl">You have <span class="text-green-400">{balance}</span> credits</h2>

        <Button onclick={() => handleAddCredits(500)} class="bg-purple-700 hover:bg-purple-600">
            Add 500 Credits
        </Button>

        <h3 class="mt-6 text-lg">Transaction History</h3>
        <div class="min-h-[100vh] flex-1 rounded-xl bg-muted/50 md:min-h-min">
            <Table.Root>
                <Table.Caption>A list of your recent reports and transactions</Table.Caption>
                <Table.Header>
                    <Table.Row>
                        <Table.Head class="w-[100px]">Date</Table.Head>
                        <Table.Head>Description</Table.Head>
                        <Table.Head class="text-right">Amount</Table.Head>
                    </Table.Row>
                </Table.Header>
                <Table.Body>
                    {#each transactions as entry, i (i)}
                        <Table.Row>
                            <Table.Cell class="font-medium">{entry.date}</Table.Cell>
                            <Table.Cell>{entry.description}</Table.Cell>
                            <Table.Cell class={entry.incoming ? 'text-right text-green-500' : 'text-right text-red-500'}>
                                {entry.amount}
                            </Table.Cell>
                        </Table.Row>
                    {/each}
                </Table.Body>
            </Table.Root>
        </div>
    {/if}
</div>

