/**
 *
 *
 * Author: Elias Sjödin
 * Created: 2024-12-01
 */

import type { ColumnDef } from "@tanstack/table-core";

export const columns: ColumnDef<string>[] = [
  {
    accessorFn: (row: string) => row,
    header: "Proxy",
  },
];
