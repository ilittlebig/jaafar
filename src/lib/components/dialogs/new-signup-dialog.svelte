<script module>
	export let newSignupDialog = $state({ open: false });
</script>

<script lang="ts">
	import { proxiesStore, type ProxyGroup } from "$lib/stores/proxies-store.svelte";
	import { addProxyGroupFormSchema } from "$lib/schemas/proxies";
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
			values: [
				{
					group: "Sabrina Carpenter",
					options: [
						{ value: "sabrina-hallenstadion", label: "Sabrina Carpenter - 3/27 - Hallenstadion"},
						{ value: "sabrina-unity-arena", label: "Sabrina Carpenter - 3/30 - Unity Arena" },
						{ value: "sabrina-royal-arena", label: "Sabrina Carpenter - 3/31 - Royal Arena" },
						{ value: "sabrina-avicii-arena", label: "Sabrina Carpenter - 4/3 - Avicii Arena" },
					],
				},
				{ value: "random-unicorn-arena", label: "Random Artist - 24/24 - Unicorn Arena" },
			]
		},
		{
			name: "proxyGroup",
			label: "Proxy Group",
			type: "select",
			placeholder: "Select a proxy group",
			values: [...proxyGroupOptions]
		},
		{
			name: "mode",
			label: "Mode",
			type: "select",
			description: "Note: This is only used for signups that require email confirmation.",
			placeholder: "Select a mode",
			allowDeselect: false,
			values: [
				{ value: "manual_confirm", label: "Manual Confirmation"},
				{ value: "auto_confirm", label: "Auto Confirmation" },
			]
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
			onsubmit={async (formData: any) => console.log(formData)}
			schema={addProxyGroupFormSchema}
		>
			{#snippet children(submitting)}
				<div class="flex justify-end">
					<Form.Button disabled={submitting}>Add Signup</Form.Button>
				</div>
			{/snippet}
		</FormComponent>
	</Dialog.Content>
</Dialog.Root>
