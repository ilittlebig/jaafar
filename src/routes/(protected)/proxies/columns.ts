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

const rowActionsCellSnippet = createRawSnippet(() => {
	return {
		render: () => `<div class="flex justify-end"></div>`,
		setup: (target) => {
			const comp = mount(ProxiesRowActions, { target });
			return () => unmount(comp);
		}
	};
});

export const columns: ColumnDef<ProxyGroup>[] = [
  {
    accessorFn: (row: ProxyGroup) => row.name,
    header: "Group Name",
  },
  {
    accessorFn: (row: ProxyGroup) => row.originalFilePath,
    header: "Original File Path",
  },
  {
		id: "actions",
    header: "",
		cell: ({ row }) => {
      return renderSnippet(rowActionsCellSnippet, row.original);
    },
  },
];
