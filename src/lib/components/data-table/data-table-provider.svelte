<script lang="ts" generics="TData, TValue">
	import { setContext, type Snippet } from "svelte";
	import {
		type ColumnDef,
		type PaginationState,
		type VisibilityState,
		getCoreRowModel,
    getPaginationRowModel,
	} from "@tanstack/table-core";
	import { createSvelteTable } from "$lib/components/ui/data-table";

	type DataTableProps<TData, TValue> = {
		columns: ColumnDef<TData, TValue>[];
		data: TData[];
		children: Snippet;
	};

  let pagination = $state<PaginationState>({ pageIndex: 0, pageSize: 10 });
	let columnVisibility = $state<VisibilityState>({});

	let {
		data,
		columns,
		children,
	}: DataTableProps<TData, TValue> = $props();

	const table = createSvelteTable({
		get data() {
      return data;
    },
		get columns() {
			return columns;
		},
    state: {
      get pagination() {
        return pagination;
      },
			get columnVisibility() {
				return columnVisibility;
			},
    },
		onColumnVisibilityChange: updater => {
      if (typeof updater === "function") {
        columnVisibility = updater(columnVisibility);
      } else {
        columnVisibility = updater;
      }
		},
    onPaginationChange: updater => {
      if (typeof updater === "function") {
        pagination = updater(pagination);
      } else {
        pagination = updater;
      }
    },
    getCoreRowModel: getCoreRowModel(),
    getPaginationRowModel: getPaginationRowModel(),
	});

	setContext("data-table", { table });
</script>

<div>
	{@render children()}
</div>
