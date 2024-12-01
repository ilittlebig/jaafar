<script lang="ts">
	import { accountsStore } from "$lib/stores/accounts-store.svelte";
	import { importAccounts } from "$lib/services/accounts-service";
	import { Button } from "$lib/components/ui/button";
	import { ScrollArea } from "$lib/components/ui/scroll-area";
	import { DataTable } from "$lib/components/data-table";
	import ImportDialog, { importDialog } from "$lib/components/dialogs/import-dialog.svelte";
	import { columns } from "./columns";
</script>

<div class="flex flex-col gap-y-2 w-full">
	<div class="flex justify-between">
		<h1 class="font-semibold text-2xl">Accounts</h1>
		<div class="flex gap-x-2 justify-end">
			{#if accountsStore.length > 0}
				<Button variant="outline">
					<i class="fa-regular fa-cog"></i>
				</Button>
				<Button variant="outline" class="hidden">
					<i class="fa-regular fa-download"></i>
					Export
				</Button>
			{/if}
			<ImportDialog extension="csv" onImport={importAccounts}>
				Import Accounts
			</ImportDialog>
		</div>
	</div>
</div>

{#if accountsStore.length > 0}
	<ScrollArea orientation="horizontal" class="w-full">
		<DataTable.Provider data={accountsStore} {columns}>
			<DataTable.Table />
			<DataTable.Pagination />
		</DataTable.Provider>
	</ScrollArea>
{:else}
	<div class="flex flex-col gap-y-4 items-center justify-center h-full">
		<img src="/images/add_document.svg" width={200} height={200} alt="Import Accounts Illustration" />
		<div class="flex flex-col gap-y-1 items-center">
			<p class="font-semibold">No Accounts Found</p>
			<p class="max-w-[350px] text-center text-sm text-muted-foreground">
				You do not have any accounts yet. Click the import button to add accounts.
			</p>
		</div>
		<Button onclick={() => importDialog.open = true}>
			<i class="fa-regular fa-file-import"></i>
			Import Accounts
		</Button>
	</div>
{/if}
