<script module>
	export let resetPasswordDialog = $state({ open: false });
</script>

<script lang="ts">
	import { handleResetPassword } from "$lib/services/auth-service";
	import { resetPasswordFormSchema } from "$lib/schemas/auth";
	import { Button } from "$lib/components/ui/button";
	import * as Dialog from "$lib/components/ui/dialog";
	import * as Form from "$lib/components/ui/form";
	import FormComponent from "$lib/components/auth/form-component.svelte";

	const fields = [
		{ name: "username", label: "Email", placeholder: "user@example.com" }
	];

	const data = { username: "" };
</script>

<Dialog.Root bind:open={resetPasswordDialog.open}>
	<Dialog.Trigger>
		{#snippet child({ props })}
			<Button {...props} variant="link" class="ml-auto inline-block text-sm h-4 py-0 px-0 font-normal leading-none">
				Forgot your password?
			</Button>
		{/snippet}
	</Dialog.Trigger>
	<Dialog.Content class="max-w-[425px]">
		<Dialog.Header>
			<Dialog.Title>Reset Password</Dialog.Title>
			<Dialog.Description>
        Enter your email below to recieve a verification code to reset your password.
			</Dialog.Description>
		</Dialog.Header>
		<FormComponent
			{data} 
			{fields}
			onsubmit={handleResetPassword}
			onsuccess={() => resetPasswordDialog.open = false}
			schema={resetPasswordFormSchema}
		>
			{#snippet children(submitting)}
				<div class="flex justify-end">
					<Form.Button disabled={submitting}>Send Code</Form.Button>
				</div>
			{/snippet}
		</FormComponent>
	</Dialog.Content>
</Dialog.Root>
