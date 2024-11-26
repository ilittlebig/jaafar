<script lang="ts">
	import NewPasswordRequiredDialog from "$lib/components/dialogs/auth/new-password-required-dialog.svelte";
	import ResetPasswordVerificationDialog from "$lib/components/dialogs/auth/reset-password-verification-dialog.svelte";
	import TotpCodeDialog from "$lib/components/dialogs/auth/totp-code-dialog.svelte";
	import TotpSetupDialog from "$lib/components/dialogs/auth/totp-setup-dialog.svelte";

	let { children } = $props();
</script>

<ResetPasswordVerificationDialog />
<TotpCodeDialog />
<NewPasswordRequiredDialog />
<TotpSetupDialog />

{@render children()}
