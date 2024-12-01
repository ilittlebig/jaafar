<script lang="ts">
	import { getContext } from "svelte";
	import type { Table as TableType } from "@tanstack/table-core";
	import type { Account } from "$lib/stores/accounts-store.svelte";
	import { FlexRender } from "$lib/components/ui/data-table";
	import * as Table from "$lib/components/ui/table";

	interface Props {
		table: TableType<Account>;
	}

	const { table }: Props = getContext("data-table");
</script>

<div class="rounded-md border">
	<Table.Root>
		<Table.Header class="bg-accent">
			{#each table.getHeaderGroups() as headerGroup (headerGroup.id)}
				<Table.Row>
					{#each headerGroup.headers as header (header.id)}
						<Table.Head>
							{#if !header.isPlaceholder}
								<FlexRender
									content={header.column.columnDef.header}
									context={header.getContext()}
								/>
							{/if}
						</Table.Head>
					{/each}
				</Table.Row>
			{/each}
		</Table.Header>
		<Table.Body>
			{#each table.getRowModel().rows as row (row.id)}
				<Table.Row data-state={row.getIsSelected() && "selected"}>
					{#each row.getVisibleCells() as cell (cell.id)}
						<Table.Cell>
							<FlexRender
								content={cell.column.columnDef.cell}
								context={cell.getContext()}
							/>
						</Table.Cell>
					{/each}
				</Table.Row>
			{:else}
				<Table.Row>
					<Table.Cell colspan={table.getAllColumns().length} class="h-24 text-center">
						No results.
					</Table.Cell>
				</Table.Row>
			{/each}
		</Table.Body>
	</Table.Root>
</div>
