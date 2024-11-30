<script lang="ts">
	import { Button } from "$lib/components/ui/button";
	import { ScrollArea } from "$lib/components/ui/scroll-area";
	import AccountsTable from "$lib/components/accounts-table.svelte";
	import ImportAccountsDialog from "$lib/components/dialogs/import-accounts-dialog.svelte";
</script>

<div class="flex flex-col gap-y-2 w-full">
	<div class="flex justify-between items-center">
		<h1 class="font-semibold text-2xl">Accounts</h1>
		<div class="flex gap-x-2 justify-end">
			<Button variant="outline">
				<i class="fa-regular fa-cog"></i>
			</Button>
			<Button variant="outline">
				<i class="fa-regular fa-download"></i>
				Export
			</Button>
			<ImportAccountsDialog />
		</div>
	</div>
</div>

<ScrollArea orientation="horizontal" class="w-full">
	<AccountsTable />
</ScrollArea>
