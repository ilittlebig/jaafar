<script lang="ts">
	import ConfirmSignUpDialog from "$lib/components/dialogs/auth/confirm-sign-up-dialog.svelte";
	let { children } = $props();
</script>

<ConfirmSignUpDialog />

{@render children()}
