<script module>
	export let deleteDialog = $state({ open: false });
</script>

<script lang="ts">
	import type { Snippet } from "svelte";
	import { buttonVariants } from "$lib/components/ui/button";
	import * as AlertDialog from "$lib/components/ui/alert-dialog";
  import * as Alert from "$lib/components/ui/alert";

	let error: string | undefined = $state();

	interface Props {
		title: string;
		description: string;
		actionLabel: string;
		children: Snippet<[{ props: Record<string, unknown> }]>,
		onconfirm: () => Promise<any>;
	}

	let {
		title,
		description,
		actionLabel,
		children,
		onconfirm,
	}: Props = $props();

	const handleConfirm = async () => {
		try {
			await onconfirm();
		} catch (err: any) {
			error = err.message;
		}
	}

	const onOpenChange = (open: boolean) => {
		if (!open) return;
		error = undefined;
	}
</script>

<AlertDialog.Root bind:open={deleteDialog.open} {onOpenChange}>
	<AlertDialog.Trigger>
		{#snippet child({ props })}
			{@render children({ props })}
		{/snippet}
  </AlertDialog.Trigger>
	<AlertDialog.Content class="max-w-[425px]">
		<AlertDialog.Header>
			<AlertDialog.Title>{title}</AlertDialog.Title>
			<AlertDialog.Description>{description}</AlertDialog.Description>
		</AlertDialog.Header>

		{#if error}
			<Alert.Root variant="destructive">
				<i class="fa-regular fa-circle-exclamation text-lg absolute left-4 top-4"></i>
				<Alert.Title>Error</Alert.Title>
				<Alert.Description>{error}</Alert.Description>
			</Alert.Root>
		{/if}

		<AlertDialog.Footer>
			<AlertDialog.Cancel>Cancel</AlertDialog.Cancel>
				<AlertDialog.Action onclick={handleConfirm} class={buttonVariants({ variant: "destructive" })}>
					{actionLabel}
				</AlertDialog.Action>
		</AlertDialog.Footer>
	</AlertDialog.Content>
</AlertDialog.Root>
