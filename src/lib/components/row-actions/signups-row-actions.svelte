<script lang="ts">
	import { invoke } from "@tauri-apps/api/core";
	import type { Signup } from "$lib/stores/signups-store.svelte";
	import { deleteSignup } from "$lib/services/signups-service";
	import { signupCommands } from "$lib/data/signups";
	import { buttonVariants } from "$lib/components/ui/button";
	import * as DropdownMenu from "$lib/components/ui/dropdown-menu";
	import DeleteDialog from "$lib/components/dialogs/delete-dialog.svelte";

	let { signup }: { signup: Signup } = $props();
	let isOpen = $state(false);

	const handleDeleteConfirm = async () => {
		await deleteSignup(signup.id);
		isOpen = false;
	}

	const startSignup = async () => {
		const signup_command = signupCommands[signup.product];
		await invoke(signup_command, {
			proxyGroup: signup.proxyGroup,
			mode: signup.mode,
		});
	}
</script>

<DeleteDialog
	title="Delete Signup"
	description="Are you sure you want to delete this signup? This action is irreversible."
	actionLabel="Delete Signup"
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
		{#if signup.status === "not_started"}
			<DropdownMenu.Item onclick={startSignup}>Start signup</DropdownMenu.Item>
		{/if}
		<DropdownMenu.Item>Edit</DropdownMenu.Item>
		{#if signup.status !== "not_started"}
			<DropdownMenu.Item>View info</DropdownMenu.Item>
		{/if}
		<DropdownMenu.Separator />
		<DropdownMenu.Item onclick={() => isOpen = true}>Delete</DropdownMenu.Item>
	</DropdownMenu.Content>
</DropdownMenu.Root>
