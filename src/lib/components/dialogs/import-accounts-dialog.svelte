<script module>
	export let importAccountsDialog = $state({ open: true });
</script>

<script lang="ts">
	import { open } from "@tauri-apps/plugin-dialog";
	import { Button, buttonVariants } from "$lib/components/ui/button";
	import * as Dialog from "$lib/components/ui/dialog";

	const handleClick = async () => {
		const file = await open({
			multiple: false,
			directory: false,
			filters: [
				{ name: "a", extensions: ["csv"] },
			],
		});
		console.log(file);
	}
</script>

<Dialog.Root bind:open={importAccountsDialog.open}>
	<Dialog.Trigger class={buttonVariants({ variant: "default" })}>
		<i class="fa-regular fa-file-import"></i>
		Import
	</Dialog.Trigger>
	<Dialog.Content class="max-w-[500px]">
		<Dialog.Header>
			<Dialog.Title>Import Accounts</Dialog.Title>
		</Dialog.Header>
		<div class="flex gap-x-2">
			<Button class="w-full h-48" variant="outline">
				<div class="flex flex-col h-full w-full text-left text-wrap">
					<p class="text-base font-semibold">Text File</p>
					<p class="font-normal text-muted-foreground">
						Generate a CSV file from a list of emails automatically.
					</p>
				</div>
			</Button>
			<Button onclick={handleClick} class="w-full h-48" variant="outline">
				<div class="flex flex-col justify-between h-full w-full text-left text-wrap">
					<div class="flex flex-col">
						<p class="text-base font-semibold">CSV File</p>
						<p class="font-normal text-muted-foreground">
							Import a CSV file with the correct format.
						</p>
					</div>
					<div>
						<p class="font-semibold">Useful if some accounts need unique values.</p>
					</div>
				</div>
			</Button>
		</div>
	</Dialog.Content>
</Dialog.Root>
