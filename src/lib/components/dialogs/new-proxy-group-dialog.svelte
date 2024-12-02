<script module>
	export let newProxyGroupDialog = $state({ open: false });
</script>

<script lang="ts">
	import { addProxyGroup } from "$lib/services/proxies-service";
	import { addProxyGroupFormSchema } from "$lib/schemas/proxies";
	import { buttonVariants } from "$lib/components/ui/button";
	import * as Dialog from "$lib/components/ui/dialog";
	import * as Form from "$lib/components/ui/form";
	import FormComponent from "$lib/components/auth/form-component.svelte";

	const fields = [
		{ name: "name", label: "Name", description: "Shows up when you need to select proxies." },
		{ name: "file", label: "File", type: "file", extensions: ["txt"] },
	];

	const data = { name: "", file: undefined };

	const onsubmit = async (formData: any) => {
		await addProxyGroup(formData);
	}

	const onsuccess = () => {
		newProxyGroupDialog.open = false;
	}
</script>

<Dialog.Root bind:open={newProxyGroupDialog.open}>
	<Dialog.Trigger class={buttonVariants({ variant: "default" })}>
		<i class="fa-regular fa-pen"></i>
		New Proxy Group
	</Dialog.Trigger>
	<Dialog.Content class="max-w-[425px]">
		<Dialog.Header>
			<Dialog.Title>New Proxy Group</Dialog.Title>
			<Dialog.Description>
				Provide a name for your new group and select a text file containing the proxies.
			</Dialog.Description>
		</Dialog.Header>
		<FormComponent
			{data} 
			{fields}
			{onsubmit}
			{onsuccess}
			schema={addProxyGroupFormSchema}
		>
			{#snippet children(submitting)}
				<div class="flex justify-end">
					<Form.Button disabled={submitting}>Create Group</Form.Button>
				</div>
			{/snippet}
		</FormComponent>
	</Dialog.Content>
</Dialog.Root>
