/**
 *
 *
 * Author: Elias Sjödin
 * Created: 2024-12-01
 */

import { createRawSnippet, mount, unmount } from "svelte";
import type { Signup } from "$lib/stores/signups-store.svelte";
import type { ColumnDef } from "@tanstack/table-core";
import { renderSnippet, renderComponent } from "$lib/components/ui/data-table";
import ProxiesRowActions from "$lib/components/row-actions/proxies-row-actions.svelte";
import { DataTable } from "$lib/components/data-table";

const rowActionsCellSnippet = createRawSnippet(() => {
	return {
		render: () => `<div class="flex justify-end"></div>`,
		setup: (target) => {
			const comp = mount(ProxiesRowActions, { target });
			return () => unmount(comp);
		}
	};
});

const amountCellSnippet = createRawSnippet<[string]>((getAmount) => {
	const amount = getAmount();
	return {
		render: () => `<div class="font-medium">${amount}</div>`,
	};
});

export const columns: ColumnDef<Signup>[] = [
  {
    accessorFn: (row: Signup) => row.status,
    header: "Status",
		cell: ({ row }) => {
			const value: string = row.getValue("status");
      return renderComponent(DataTable.BadgeCell, {
				value,
				variant: "outline"
			});
    },
  },
  {
    accessorFn: (row: Signup) => row.product,
    header: "Product",
  },
  {
    accessorFn: (row: Signup) => row.proxyGroup,
    header: "Proxy Group",
  },
  {
		id: "amount",
    accessorFn: (row: Signup) => row.amountOfAccounts,
    header: "Amount of Accounts",
		cell: ({ row }) => {
			const formatter = new Intl.NumberFormat("en-US");
			return renderSnippet(
				amountCellSnippet,
				formatter.format(parseFloat(row.getValue("amount")))
			);
    },
  },
  {
    accessorFn: (row: Signup) => row.mode,
    header: "Mode",
  },
  {
		id: "actions",
    header: "",
		cell: ({ row }) => {
      return renderSnippet(rowActionsCellSnippet, row.original);
    },
  },
];
