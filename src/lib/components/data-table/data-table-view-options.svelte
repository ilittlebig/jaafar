<script lang="ts">
	import { getContext } from "svelte";
	import type { Table as TableType } from "@tanstack/table-core";
	import type { Account } from "$lib/stores/accounts-store.svelte";
  import { Button } from "$lib/components/ui/button";
  import * as DropdownMenu from "$lib/components/ui/dropdown-menu";

	interface Props {
		table: TableType<Account>;
	}

	const { table }: Props = getContext("data-table");
</script>

<DropdownMenu.Root>
  <DropdownMenu.Trigger>
		{#snippet child({ props })}
			<Button {...props} variant="outline" size="sm">
				<i class="fa-regular fa-filter-list"></i>
				View
			</Button>
		{/snippet}
  </DropdownMenu.Trigger>

  <DropdownMenu.Content>
    <DropdownMenu.Label>Toggle Columns</DropdownMenu.Label>
    <DropdownMenu.Separator />
		{#each table.getAllLeafColumns() as column}
			<DropdownMenu.CheckboxItem
				checked={column.getIsVisible()}
				disabled={!column.getCanHide()}
				onCheckedChange={checked => {
					column.toggleVisibility == null || column.toggleVisibility(checked);
				}}
				closeOnSelect={false}
			>
				{column.columnDef.header}
			</DropdownMenu.CheckboxItem>
		{/each}
  </DropdownMenu.Content>
</DropdownMenu.Root>
