/**
 *
 *
 * Author: Elias Sjödin
 * Created: 2024-12-01
 */

import { createRawSnippet, mount, unmount } from "svelte";
import type { ColumnDef } from "@tanstack/table-core";
import type { Account } from "$lib/stores/accounts-store.svelte";
import { renderSnippet } from "$lib/components/ui/data-table";
import { DataTable } from "$lib/components/data-table";

const badgeCellSnippet = createRawSnippet<[string]>(getValue => {
	const value = getValue();
	return {
		render: () => "<div></div>",
		setup: (target) => {
			const comp = mount(DataTable.BadgeCell, { target, props: {
				value, variant: "outline"
			}});
			return () => unmount(comp);
		}
	};
});

export const columns: ColumnDef<Account>[] = [
  {
    accessorKey: "email",
    header: "Email",
  },
  {
    accessorKey: "firstname",
    header: "First Name",
		cell: ({ row }) => {
			const value: string = row.getValue("firstname");
      return renderSnippet(badgeCellSnippet, value);
    },
  },
  {
    accessorKey: "lastname",
    header: "Last Name",
		cell: ({ row }) => {
			const value: string = row.getValue("lastname");
      return renderSnippet(badgeCellSnippet, value);
    },
  },
  {
    accessorKey: "phone",
    header: "Phone",
		cell: ({ row }) => {
			const value: string = row.getValue("phone");
      return renderSnippet(badgeCellSnippet, value);
    },
  },
  {
    accessorKey: "address1",
    header: "Address 1",
		cell: ({ row }) => {
			const value: string = row.getValue("address1");
      return renderSnippet(badgeCellSnippet, value);
    },
  },
  {
    accessorKey: "address2",
    header: "Address 2",
		cell: ({ row }) => {
			const value: string = row.getValue("address2");
      return renderSnippet(badgeCellSnippet, value);
    },
  },
  {
    accessorKey: "city",
    header: "City",
		cell: ({ row }) => {
			const value: string = row.getValue("city");
      return renderSnippet(badgeCellSnippet, value);
    },
  },
  {
    accessorKey: "postcode",
    header: "Postcode",
		cell: ({ row }) => {
			const value: string = row.getValue("postcode");
      return renderSnippet(badgeCellSnippet, value);
    },
  },
  {
    accessorKey: "country",
    header: "Country",
  },
];
