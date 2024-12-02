<script module>
	export let deleteDialog = $state({ open: false });
</script>

<script lang="ts">
	import { buttonVariants } from "$lib/components/ui/button";
	import * as AlertDialog from "$lib/components/ui/alert-dialog";

	interface Props {
		title: string;
		description: string;
		actionLabel: string;
	}

	let {
		title,
		description,
		actionLabel,
	}: Props = $props();
</script>

<AlertDialog.Root bind:open={deleteDialog.open}>
	<AlertDialog.Content class="max-w-[425px]">
		<AlertDialog.Header>
			<AlertDialog.Title>{title}</AlertDialog.Title>
			<AlertDialog.Description>{description}</AlertDialog.Description>
		</AlertDialog.Header>
		<AlertDialog.Footer>
			<AlertDialog.Cancel>Cancel</AlertDialog.Cancel>
				<AlertDialog.Action class={buttonVariants({ variant: "destructive" })}>
					{actionLabel}
				</AlertDialog.Action>
		</AlertDialog.Footer>
	</AlertDialog.Content>
</AlertDialog.Root>
