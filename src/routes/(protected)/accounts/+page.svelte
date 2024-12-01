<script lang="ts">
	import { Button } from "$lib/components/ui/button";
	import { ScrollArea } from "$lib/components/ui/scroll-area";
	import AccountsTable from "$lib/components/accounts-table.svelte";
	import ImportAccountsDialog, { importAccountsDialog } from "$lib/components/dialogs/import-accounts-dialog.svelte";
</script>

<div class="flex flex-col gap-y-2 w-full">
	<div class="flex justify-between items-center">
		<h1 class="font-semibold text-2xl">Accounts</h1>
		<div class="flex gap-x-2 justify-end">
			{#if false}
				<Button variant="outline">
					<i class="fa-regular fa-cog"></i>
				</Button>
				<Button variant="outline">
					<i class="fa-regular fa-download"></i>
					Export
				</Button>
			{/if}
			<ImportAccountsDialog />
		</div>
	</div>
</div>

{#if false}
	<ScrollArea orientation="horizontal" class="w-full">
		<AccountsTable />
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
		<Button onclick={() => importAccountsDialog.open = true}>
			<i class="fa-regular fa-file-import"></i>
			Import Accounts
		</Button>
	</div>
{/if}
