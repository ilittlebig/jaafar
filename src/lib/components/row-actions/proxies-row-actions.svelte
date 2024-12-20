<script lang="ts">
	import type { ProxyGroup } from "$lib/stores/proxies-store.svelte";
	import { deleteProxyGroup } from "$lib/services/proxies-service";
	import { buttonVariants } from "$lib/components/ui/button";
	import * as DropdownMenu from "$lib/components/ui/dropdown-menu";
	import DeleteDialog from "$lib/components/dialogs/delete-dialog.svelte";

	let { proxyGroup }: { proxyGroup: ProxyGroup } = $props();
	let isOpen = $state(false);

	const handleDeleteConfirm = async () => {
		await deleteProxyGroup(proxyGroup.name);
		isOpen = false;
	}
</script>

<DeleteDialog
	title={`Delete ${proxyGroup.name}`}
	description="Are you sure you want to delete this proxy group? This action is irreversible."
	actionLabel="Delete Group"
	onconfirm={handleDeleteConfirm}
	bind:open={isOpen}
/>

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
		<DropdownMenu.Item>Edit</DropdownMenu.Item>
		<DropdownMenu.Item>View proxies</DropdownMenu.Item>
		<DropdownMenu.Separator />
		<DropdownMenu.Item onclick={() => isOpen = true}>Delete</DropdownMenu.Item>
	</DropdownMenu.Content>
</DropdownMenu.Root>
