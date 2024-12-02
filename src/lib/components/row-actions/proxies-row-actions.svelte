<script lang="ts">
	import { buttonVariants } from "$lib/components/ui/button";
	import * as DropdownMenu from "$lib/components/ui/dropdown-menu";
	import { deleteDialog } from "$lib/components/dialogs/delete-dialog.svelte";
</script>

<DropdownMenu.Root>
	<DropdownMenu.Trigger
		class={buttonVariants({
			variant: "ghost",
			class: "relative size-8",
		})}
	>
		<i class="fa-regular fa-ellipsis"></i>
		<span class="sr-only">Open menu</span>
	</DropdownMenu.Trigger>
	<DropdownMenu.Content align="end">
		<DropdownMenu.Label>Actions</DropdownMenu.Label>
		<DropdownMenu.Item>View proxies</DropdownMenu.Item>
		<DropdownMenu.Item onclick={() => deleteDialog.open = true}>
			Delete
		</DropdownMenu.Item>
	</DropdownMenu.Content>
</DropdownMenu.Root>
