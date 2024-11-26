<script module>
	export let resetPasswordVerificationDialog = $state({ open: false });
</script>

<script lang="ts">
	import { handleConfirmResetPassword } from "$lib/services/auth-service";
	import { resetPasswordVerificationFormSchema } from "$lib/schemas/auth";
	import * as Dialog from "$lib/components/ui/dialog";
	import * as Form from "$lib/components/ui/form";
	import FormComponent from "$lib/components/auth/form-component.svelte";

	const fields = [
		{ name: "confirmationCode", label: "Confirmation Code" },
		{ name: "newPassword", label: "New Password", type: "password" },
		{ name: "confirmPassword", label: "Confirm Password", type: "password" },
	];

	const data = { confirmationCode: "", newPassword: "", confirmPassword: "" };
</script>

<Dialog.Root bind:open={resetPasswordVerificationDialog.open}>
	<Dialog.Content class="max-w-[425px]">
		<Dialog.Header>
			<Dialog.Title>Enter Verification Code</Dialog.Title>
			<Dialog.Description>
        Enter your email below to recieve a verification code to reset your password.
			</Dialog.Description>
		</Dialog.Header>
		<FormComponent
			{data} 
			{fields}
			onsubmit={handleConfirmResetPassword}
			onsuccess={() => resetPasswordVerificationDialog.open = false}
			schema={resetPasswordVerificationFormSchema}
		>
			{#snippet children(submitting)}
				<div class="flex justify-end">
					<Form.Button disabled={submitting}>Change Password</Form.Button>
				</div>
			{/snippet}
		</FormComponent>
	</Dialog.Content>
</Dialog.Root>
