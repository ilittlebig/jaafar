<script module>
	export let importProxiesDialog = $state({ open: false });
</script>

<script lang="ts">
	import { selectFile } from "$lib/services/files-service";
	import { importAccounts } from "$lib/services/accounts-service";
	import { Button, buttonVariants } from "$lib/components/ui/button";
	import * as Dialog from "$lib/components/ui/dialog";
  import * as Alert from "$lib/components/ui/alert";
	import ImportValidationErrorsDialog from "$lib/components/dialogs/import-validation-errors-dialog.svelte";

	let error: string | undefined = $state();
	let validationErrors: Record<number, string[]> = $state([]);

	const handleTxtImport = async () => {
		error = undefined;
		validationErrors = [];

		const filePath = await selectFile({
			multiple: false,
			directory: false,
			filters: [{ name: "extension", extensions: ["txt"] }],
		});
		if (!filePath) return;

		try {
			validationErrors = await importAccounts(filePath);
			if (Object.keys(validationErrors).length > 0) return;
			importProxiesDialog.open = false;
		} catch (err: any) {
			error = err.message;
		}
	}

	const onOpenChange = () => {
		error = undefined;
		validationErrors = [];
	}
</script>

<Dialog.Root bind:open={importProxiesDialog.open} {onOpenChange}>
	<Dialog.Trigger class={buttonVariants({ variant: "default" })}>
		<i class="fa-regular fa-file-import"></i>
		Import
	</Dialog.Trigger>
	<Dialog.Content class="max-w-[500px]">
		<Dialog.Header>
			<Dialog.Title>Import Proxies</Dialog.Title>
		</Dialog.Header>

		{#if error}
			<Alert.Root variant="destructive">
				<i class="fa-regular fa-circle-exclamation text-lg absolute left-4 top-4"></i>
				<Alert.Title>Error</Alert.Title>
				<Alert.Description>{error}</Alert.Description>
			</Alert.Root>
		{/if}

		{#if Object.keys(validationErrors).length > 0}
			<Alert.Root variant="destructive">
				<i class="fa-regular fa-circle-exclamation text-lg absolute left-4 top-4"></i>
				<Alert.Title>Error</Alert.Title>
				<Alert.Description class="flex gap-x-2 items-center">
					The following issues were found in your CSV file:
					<ImportValidationErrorsDialog {validationErrors} />
				</Alert.Description>
			</Alert.Root>
		{/if}

		<div class="flex gap-x-2">
			<Button onclick={handleTxtImport} class="w-full h-48" variant="outline">
				<div class="flex flex-col h-full w-full text-left text-wrap">
					<p class="text-base font-semibold">Text File</p>
					<p class="font-normal text-muted-foreground">
						Generate a table from a list of proxies automatically.
					</p>
				</div>
			</Button>
			<Button class="w-full h-48" variant="outline" disabled>
				<div class="flex flex-col justify-between h-full w-full text-left text-wrap">
					<div class="flex flex-col">
						<p class="text-base font-semibold">CSV File</p>
						<p class="font-normal text-muted-foreground">
							Import a CSV file with the correct format.
						</p>
					</div>
					<div>
						<p class="font-semibold">Useful if some proxies need unique values.</p>
					</div>
				</div>
			</Button>
		</div>
	</Dialog.Content>
</Dialog.Root>
