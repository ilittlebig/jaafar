<script module>
	export let totpCodeDialog = $state({ open: false });
</script>

<script lang="ts">
	import * as Dialog from "$lib/components/ui/dialog";
	import TotpCodeForm from "$lib/components/auth/totp-code-form.svelte";
</script>

<Dialog.Root bind:open={totpCodeDialog.open}>
	<Dialog.Content class="max-w-[425px]">
		<Dialog.Header>
			<Dialog.Title>Enter MFA Code</Dialog.Title>
			<Dialog.Description>
        Enter the 6-digit code from your authenticator app to continue.
			</Dialog.Description>
		</Dialog.Header>
		<TotpCodeForm />
	</Dialog.Content>
</Dialog.Root>
