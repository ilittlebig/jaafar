<script module>
	export let totpCodeDialog = $state({ open: false });
</script>

<script lang="ts">
	import { handleChallengeResponse } from "$lib/services/auth-service";
	import { totpCodeFormSchema } from "$lib/schemas/auth";
	import * as Dialog from "$lib/components/ui/dialog";
	import * as Form from "$lib/components/ui/form";
	import FormComponent from "$lib/components/auth/form-component.svelte";

	const fields = [
		{ name: "code", label: "Code" },
	];

	const data = { code: "" };
</script>

<Dialog.Root bind:open={totpCodeDialog.open}>
	<Dialog.Content class="max-w-[425px]">
		<Dialog.Header>
			<Dialog.Title>Enter MFA Code</Dialog.Title>
			<Dialog.Description>
        Enter the 6-digit code from your authenticator app to continue.
			</Dialog.Description>
		</Dialog.Header>
		<FormComponent
			{data} 
			{fields}
			onsubmit={handleChallengeResponse}
			onsuccess={() => totpCodeDialog.open = false}
			schema={totpCodeFormSchema}
		>
			{#snippet children(submitting)}
				<div class="flex justify-end">
					<Form.Button disabled={submitting}>Submit</Form.Button>
				</div>
			{/snippet}
		</FormComponent>
	</Dialog.Content>
</Dialog.Root>
