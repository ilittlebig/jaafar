<script module>
	export let importValidationErrorsDialog = $state({ open: true });
</script>

<script lang="ts">
	import { Button } from "$lib/components/ui/button";
	import { ScrollArea } from "$lib/components/ui/scroll-area";
	import * as Dialog from "$lib/components/ui/dialog";
	import * as Table from "$lib/components/ui/table";

	interface Props {
		validationErrors: Record<number, string[]>;
	}

	let {
		validationErrors
	}: Props = $props();

	const maxErrors = Math.max(
		...Object.values(validationErrors).map(rowErrors => rowErrors.length)
	);
</script>

<Dialog.Root bind:open={importValidationErrorsDialog.open}>
	<Dialog.Trigger>
		{#snippet child({ props })}
			<Button {...props} variant="link" class="inline-block text-sm h-4 py-0 px-0 font-normal leading-none">
				See More
			</Button>
		{/snippet}
	</Dialog.Trigger>
	<Dialog.Content class="max-w-[1000px] max-h-[720px] flex flex-col pb-0 h-full">
		<Dialog.Header>
			<Dialog.Title>Import Validation Errors</Dialog.Title>
			<Dialog.Description>You need to fix these errors before you can import this file.</Dialog.Description>
		</Dialog.Header>
		<ScrollArea orientation="both" scrollHideDelay={0}>
			<Table.Root class="pb-6">
				<Table.Header class="bg-accent">
					<Table.Row>
						<Table.Head>Row</Table.Head>
						{#each { length: maxErrors } as _, i}
							<Table.Head>Error {i + 1}</Table.Head>
						{/each}
					</Table.Row>
				</Table.Header>
				<Table.Body>
					{#each Object.entries(validationErrors) as [row, errors]}
						<Table.Row>
							<Table.Cell>{row}</Table.Cell>
							{#each errors as error}
								<Table.Cell class="text-nowrap">{error}</Table.Cell>
							{/each}
						</Table.Row>
					{/each}
				</Table.Body>
			</Table.Root>
		</ScrollArea>
	</Dialog.Content>
</Dialog.Root>
