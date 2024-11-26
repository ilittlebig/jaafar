<script module>
	export let newPasswordRequiredDialog = $state({ open: false });
</script>

<script lang="ts">
	import { handleChallengeResponse } from "$lib/services/auth-service";
	import { newPasswordRequiredFormSchema } from "$lib/schemas/auth";
	import * as Dialog from "$lib/components/ui/dialog";
	import * as Form from "$lib/components/ui/form";
	import FormComponent from "$lib/components/auth/form-component.svelte";

	const fields = [
		{ name: "newPassword", label: "New Password", type: "password" },
		{ name: "confirmPassword", label: "Confirm Password", type: "password" },
	];

	const data = { newPassword: "", confirmPassword: "" };
</script>

<Dialog.Root bind:open={newPasswordRequiredDialog.open}>
	<Dialog.Content class="max-w-[425px]">
		<Dialog.Header>
			<Dialog.Title>Password Reset Required</Dialog.Title>
			<Dialog.Description>
				For your security, you need to reset your password before you can continue.
			</Dialog.Description>
		</Dialog.Header>
		<FormComponent
			{data} 
			{fields}
			onsubmit={handleChallengeResponse}
			onsuccess={() => newPasswordRequiredDialog.open = false}
			schema={newPasswordRequiredFormSchema}
		>
			{#snippet children(submitting)}
				<div class="flex justify-end">
					<Form.Button disabled={submitting}>Change Password</Form.Button>
				</div>
			{/snippet}
		</FormComponent>
	</Dialog.Content>
</Dialog.Root>
