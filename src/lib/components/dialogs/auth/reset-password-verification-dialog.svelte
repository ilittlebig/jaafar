<script module>
	export let resetPasswordVerificationDialog = $state({ open: false });
</script>

<script lang="ts">
	import * as Dialog from "$lib/components/ui/dialog";
</script>

<Dialog.Root bind:open={resetPasswordVerificationDialog.open}>
	<Dialog.Content class="max-w-[425px]">
		<Dialog.Header>
			<Dialog.Title>Enter Verification Code</Dialog.Title>
			<Dialog.Description>
        Enter your email below to recieve a verification code to reset your password.
			</Dialog.Description>
		</Dialog.Header>
	</Dialog.Content>
</Dialog.Root>
