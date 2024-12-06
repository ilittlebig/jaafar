<script lang="ts">
	import { signupsStore } from "$lib/stores/signups-store.svelte";
	import { Button } from "$lib/components/ui/button";
	import { ScrollArea } from "$lib/components/ui/scroll-area";
	import { DataTable } from "$lib/components/data-table";
	import NewSignupDialog, { newSignupDialog } from "$lib/components/dialogs/new-signup-dialog.svelte";
	import { columns } from "./columns";
</script>

<div class="flex flex-col gap-y-2 w-full">
	<div class="flex justify-between">
		<h1 class="font-semibold text-2xl">Signups</h1>
		<div class="flex gap-x-2 justify-end">
			<NewSignupDialog />
		</div>
	</div>
</div>

{#if signupsStore.signups.length > 0}
	<DataTable.Provider data={signupsStore.signups} {columns}>
		<ScrollArea orientation="horizontal" class="w-full">
			<DataTable.Table />
		</ScrollArea>
		<DataTable.Pagination />
	</DataTable.Provider>
{:else}
	<div class="flex flex-col gap-y-4 items-center justify-center h-full">
		<img src="/images/sign_up.svg" width={200} height={200} alt="Signups Illustration" />
		<div class="flex flex-col gap-y-1 items-center">
			<p class="font-semibold">No Signups Found</p>
			<p class="max-w-[350px] text-center text-sm text-muted-foreground">
				You have not done any signups yet. Click the
				<span class="font-medium">New Signup</span>
				button to add your first signup.
			</p>
		</div>
		<Button onclick={() => newSignupDialog.open = true}>
			<i class="fa-regular fa-plus"></i>
			New Signup
		</Button>
	</div>
{/if}
