/**
 *
 *
 * Author: Elias Sjödin
 * Created: 2024-12-01
 */

import type { ProxyGroup } from "$lib/stores/proxies-store.svelte";
import type { ColumnDef } from "@tanstack/table-core";

export const columns: ColumnDef<ProxyGroup>[] = [
  {
    accessorFn: (row: ProxyGroup) => row.name,
    header: "Group Name",
  },
  {
    accessorFn: (row: ProxyGroup) => row.originalFilePath,
    header: "Original File Path",
  },
];
