<script lang="ts">
	import { buttonVariants } from "$lib/components/ui/button";
	import * as AlertDialog from "$lib/components/ui/alert-dialog";
  import * as Alert from "$lib/components/ui/alert";

	let error: string | undefined = $state();

	interface Props {
		title: string;
		description: string;
		actionLabel: string;
		onconfirm: () => Promise<any>;
		open: boolean;
	}

	let {
		title,
		description,
		actionLabel,
		onconfirm,
		open = $bindable(),
	}: Props = $props();

	const handleConfirm = async () => {
		try {
			await onconfirm();
		} catch (err: any) {
			error = err.message;
		}
	}

	const onOpenChange = (newState: boolean) => {
		if (!newState) return;
		error = undefined;
	}
</script>

<AlertDialog.Root bind:open {onOpenChange}>
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
