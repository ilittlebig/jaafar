<script lang="ts">
	import ResetPasswordVerificationDialog from "$lib/components/dialogs/auth/reset-password-verification-dialog.svelte";
	import TotpCodeDialog from "$lib/components/dialogs/auth/totp-code-dialog.svelte";
	let { children } = $props();
</script>

<ResetPasswordVerificationDialog />
<TotpCodeDialog />

{@render children()}
