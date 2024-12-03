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
import SignupsRowActions from "$lib/components/row-actions/signups-row-actions.svelte";
import { DataTable } from "$lib/components/data-table";

export const columns: ColumnDef<Signup>[] = [
  {
		id: "status",
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
		id: "mode",
    accessorFn: (row: Signup) => row.mode,
    header: "Mode",
		cell: ({ row }) => {
			const value: string = row.getValue("mode");
      return renderComponent(DataTable.BadgeCell, {
				value,
				variant: "default"
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
		id: "actions",
    header: "",
		cell: ({ row }) => {
			const rowActionsCellSnippet = createRawSnippet<[Signup]>(getOriginal => {
				const signup = getOriginal();
				return {
					render: () => `<div class="flex justify-end"></div>`,
					setup: target => {
						const comp = mount(SignupsRowActions, {
							target,
							props: { signup },
						});
						return () => unmount(comp);
					}
				};
			});
      return renderSnippet(rowActionsCellSnippet, row.original);
    },
  },
];
