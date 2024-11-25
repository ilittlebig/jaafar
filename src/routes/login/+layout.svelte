<script lang="ts">
	import ResetPasswordVerificationDialog from "$lib/components/dialogs/auth/reset-password-verification-dialog.svelte";
	let { children } = $props();
</script>

<ResetPasswordVerificationDialog />

{@render children()}
