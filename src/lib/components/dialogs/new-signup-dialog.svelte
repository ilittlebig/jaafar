<script module>
	export let newSignupDialog = $state({ open: false });
</script>

<script lang="ts">
	import { proxiesStore, type ProxyGroup } from "$lib/stores/proxies-store.svelte";
	import { addNewSignup } from "$lib/services/signups-service";
	import { signupModes, signupProducts } from "$lib/data/signups";
	import { newSignupFormSchema } from "$lib/schemas/signups";
	import { buttonVariants } from "$lib/components/ui/button";
	import * as Dialog from "$lib/components/ui/dialog";
	import * as Form from "$lib/components/ui/form";
	import FormComponent from "$lib/components/auth/form-component.svelte";

	let proxyGroupOptions = proxiesStore.groups.map((group: ProxyGroup) => ({
		value: group.name,
		label: `${group.name} (${group.amount})`,
	}));

	const fields = [
		{
			name: "product",
			label: "Product",
			type: "select",
			placeholder: "Select a product",
			allowDeselect: false,
			values: signupProducts,
		},
		{
			name: "proxyGroup",
			label: "Proxy Group",
			type: "select",
			placeholder: "Select a proxy group",
			allowDeselect: false,
			values: proxyGroupOptions,
		},
		{
			name: "mode",
			label: "Mode",
			type: "select",
			description: "Note: This is only used for signups that require email confirmation.",
			placeholder: "Select a mode",
			allowDeselect: false,
			values: signupModes,
		},
	];

	const data = {
		product: undefined,
		proxyGroup: undefined,
		mode: "auto_confirm",
	};

	const onsuccess = () => {
		newSignupDialog.open = false;
	}
</script>

<Dialog.Root bind:open={newSignupDialog.open}>
	<Dialog.Trigger class={buttonVariants({ variant: "default" })}>
		<i class="fa-regular fa-plus"></i>
		New Signup
	</Dialog.Trigger>
	<Dialog.Content class="max-w-[425px]">
		<Dialog.Header>
			<Dialog.Title>New Signup</Dialog.Title>
			<Dialog.Description>
				Select a product, choose a proxy group, and set the mode for the signup process.
			</Dialog.Description>
		</Dialog.Header>
		<FormComponent
			{data} 
			{fields}
			{onsuccess}
			onsubmit={addNewSignup}
			schema={newSignupFormSchema}
		>
			{#snippet children(submitting)}
				<div class="flex justify-end">
					<Form.Button disabled={submitting}>Add Signup</Form.Button>
				</div>
			{/snippet}
		</FormComponent>
	</Dialog.Content>
</Dialog.Root>
