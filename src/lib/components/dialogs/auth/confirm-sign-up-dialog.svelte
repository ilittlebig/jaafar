<script module>
	export let confirmSignUpDialog = $state({ open: false });
</script>

<script lang="ts">
	import { goto } from "$app/navigation";
	import { handleConfirmSignUp, handleResendSignUpCode } from "$lib/services/auth-service";
	import { confirmSignUpCodeFormSchema } from "$lib/schemas/auth";
	import * as Dialog from "$lib/components/ui/dialog";
	import * as Form from "$lib/components/ui/form";
	import FormComponent from "$lib/components/auth/form-component.svelte";
	import CountdownButton from "$lib/components/countdown-button.svelte";

	const fields = [
		{ name: "confirmationCode", label: "Confirmation Code" },
	];

	const data = { confirmationCode: "" };

	const onsuccess = () => {
		confirmSignUpDialog.open = false;
		goto("/login");
	}
</script>

<Dialog.Root bind:open={confirmSignUpDialog.open}>
	<Dialog.Content class="max-w-[425px]">
		<Dialog.Header>
			<Dialog.Title>Confirm Sign Up</Dialog.Title>
			<Dialog.Description>
        Enter the 6-digit code sent to your email to continue.
			</Dialog.Description>
		</Dialog.Header>
		<FormComponent
			{data} 
			{fields}
			onsubmit={handleConfirmSignUp}
			{onsuccess}
			schema={confirmSignUpCodeFormSchema}
		>
			{#snippet children(submitting)}
				<div class="flex justify-end">
					<Form.Button disabled={submitting}>Sign Up</Form.Button>
				</div>
				<p class="text-sm text-center">
					Didn't receive a code?
					<CountdownButton callback={handleResendSignUpCode} initialDelay={5}>
						Send code
					</CountdownButton>
				</p>
			{/snippet}
		</FormComponent>
	</Dialog.Content>
</Dialog.Root>
