/**
 *
 * Author: Elias Sjödin
 * Created: 2024-12-01
 */

import DataTablePagination from "./data-table-pagination.svelte";
import DataTableProvider from "./data-table-provider.svelte";
import DataTableTable from "./data-table-table.svelte";
import DataTableBadgeCell from "./data-table-badge-cell.svelte";
import DataTableToolbar from "./data-table-toolbar.svelte";
import DataTableViewOptions from "./data-table-view-options.svelte";

export const DataTable =  {
	Pagination: DataTablePagination,
	Provider: DataTableProvider,
	Table: DataTableTable,
	BadgeCell: DataTableBadgeCell,
	Toolbar: DataTableToolbar,
	ViewOptions: DataTableViewOptions,
}
