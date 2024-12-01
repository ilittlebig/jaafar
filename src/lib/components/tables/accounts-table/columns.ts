/**
 *
 *
 * Author: Elias Sjödin
 * Created: 2024-12-01
 */

import type { ColumnDef } from "@tanstack/table-core";
import type { Account } from "$lib/stores/accounts-store.svelte";

export const columns: ColumnDef<Account>[] = [
  {
    accessorKey: "email",
    header: "Email",
  },
  {
    accessorKey: "firstname",
    header: "First Name",
  },
  {
    accessorKey: "lastname",
    header: "Last Name",
  },
  {
    accessorKey: "phone",
    header: "Phone",
  },
  {
    accessorKey: "address1",
    header: "Address 1",
  },
  {
    accessorKey: "address2",
    header: "Address 2",
  },
  {
    accessorKey: "city",
    header: "City",
  },
  {
    accessorKey: "postcode",
    header: "Postcode",
  },
  {
    accessorKey: "country",
    header: "Country",
  },
];
