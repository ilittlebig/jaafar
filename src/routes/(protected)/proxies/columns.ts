/**
 *
 *
 * Author: Elias Sjödin
 * Created: 2024-12-01
 */

import { createRawSnippet, mount, unmount } from "svelte";
import type { ProxyGroup } from "$lib/stores/proxies-store.svelte";
import type { ColumnDef } from "@tanstack/table-core";
import { renderSnippet } from "$lib/components/ui/data-table";
import ProxiesRowActions from "$lib/components/row-actions/proxies-row-actions.svelte";

export const columns: ColumnDef<ProxyGroup>[] = [
  {
    accessorFn: (row: ProxyGroup) => row.name,
    header: "Group Name",
  },
  {
    accessorFn: (row: ProxyGroup) => row.originalFilePath,
    header: "File Path",
  },
  {
		id: "amount",
    accessorFn: (row: ProxyGroup) => row.amount,
    header: "Amount of Proxies",
		cell: ({ row }) => {
			const formatter = new Intl.NumberFormat("en-US");
			const amountCellSnippet = createRawSnippet<[string]>(getAmount => {
				const amount = getAmount();
				return {
					render: () => `<div class="font-medium">${amount}</div>`,
				};
			});

			return renderSnippet(
				amountCellSnippet,
				formatter.format(parseFloat(row.getValue("amount")))
			);
    },
  },
  {
		id: "actions",
    header: "",
		cell: ({ row }) => {
			const rowActionsCellSnippet = createRawSnippet<[ProxyGroup]>(getOriginal => {
				const proxyGroup = getOriginal();
				return {
					render: () => `<div class="flex justify-end"></div>`,
					setup: target => {
						const comp = mount(ProxiesRowActions, {
							target,
							props: { proxyGroup },
						});
						return () => unmount(comp);
					}
				};
			});
      return renderSnippet(rowActionsCellSnippet, row.original);
    },
  },
];
