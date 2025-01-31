<script lang="ts">
    import * as Table from '$lib/components/ui/table/index.js';
    import * as Pagination from '$lib/components/ui/pagination/index.js';
    import ChevronLeft from 'lucide-svelte/icons/chevron-left';
    import ChevronRight from 'lucide-svelte/icons/chevron-right';
    import { get } from '$lib/request';
    import { onMount } from 'svelte';
    import Spinner from '$lib/components/spinner.svelte';

    const defaultSize = 8;

    interface ReportPage {
        page: number;
        size: number;
        total: number;
        reports: Report[];
    }

    interface Report {
        id: string;
        title: string;
        date: string;
        status: string;
        cost: number | null;
    }

    let currentPage: ReportPage | undefined = {
        page: 1,
        size: 8,
        total: 6,
        reports: [
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
        ].toReversed()
    };

    async function getPage(page: number) {
        let size = currentPage?.size ?? defaultSize;
        const query = `v1/reports/?page=${page}&size=${size}`;
        currentPage = (await get<ReportPage>(query)).result;
    }

    onMount(async () => {
        // await getPage(1);
    });
</script>

<div class="flex flex-1 flex-col gap-4 p-4">
    <div class="min-h-[100vh] flex-1 rounded-xl bg-muted/50 md:min-h-min">
        {#if currentPage}
            <Table.Root>
                <Table.Header>
                    <Table.Row>
                        <Table.Head class="w-[100px]">Status</Table.Head>
                        <Table.Head class="w-[100px]">Date</Table.Head>
                        <Table.Head>Title</Table.Head>
                        <Table.Head class="text-right">Cost</Table.Head>
                    </Table.Row>
                </Table.Header>
                <Table.Body>
                    {#each currentPage.reports as report, i (i)}
                        <Table.Row>
                            <Table.Cell class="font-medium">{report.status}</Table.Cell>
                            <Table.Cell>{report.date}</Table.Cell>
                            <Table.Cell>{report.title}</Table.Cell>
                            <Table.Cell class="text-right"
                                >{#if report.cost}{report.cost} Credits{:else}Unavailable{/if}</Table.Cell
                            >
                        </Table.Row>
                    {/each}
                </Table.Body>
            </Table.Root>
            <Pagination.Root
                count={currentPage.total}
                perPage={currentPage.size}
                onPageChange={(page) => getPage(page)}
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
