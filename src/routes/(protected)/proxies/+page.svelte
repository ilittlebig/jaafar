<script lang="ts">
	import { proxiesStore } from "$lib/stores/proxies-store.svelte";
	import { Button } from "$lib/components/ui/button";
	import { ScrollArea } from "$lib/components/ui/scroll-area";
	import { DataTable } from "$lib/components/data-table";
	import NewProxyGroupDialog, { newProxyGroupDialog } from "$lib/components/dialogs/new-proxy-group-dialog.svelte";
	import DeleteDialog from "$lib/components/dialogs/delete-dialog.svelte";
	import { columns } from "./columns";
</script>

<DeleteDialog
	title="Delete Proxy Group"
	description="Are you sure you want to delete this proxy group? This action is irreversible."
	actionLabel="Delete Group"
/>

<div class="flex flex-col gap-y-2 w-full">
	<div class="flex justify-between">
		<h1 class="font-semibold text-2xl">Proxies</h1>
		<div class="flex gap-x-2 justify-end">
			<NewProxyGroupDialog />
		</div>
	</div>
</div>

{#if proxiesStore.groups.length > 0}
	<ScrollArea orientation="horizontal" class="w-full">
		<DataTable.Provider data={proxiesStore.groups} {columns}>
			<DataTable.Table />
			<DataTable.Pagination />
		</DataTable.Provider>
	</ScrollArea>
{:else}
	<div class="flex flex-col gap-y-4 items-center justify-center h-full">
		<img src="/images/add_proxy.svg" width={200} height={200} alt="Import Proxies Illustration" />
		<div class="flex flex-col gap-y-1 items-center">
			<p class="font-semibold">No Proxies Found</p>
			<p class="max-w-[350px] text-center text-sm text-muted-foreground">
				You have not added any proxies yet. Click the button to add proxies.
			</p>
		</div>
		<Button onclick={() => newProxyGroupDialog.open = true}>
			<i class="fa-regular fa-pen"></i>
			Create Proxy Group
		</Button>
	</div>
{/if}
