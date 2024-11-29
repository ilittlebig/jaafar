<script module>
	export let signOutDialog = $state({ open: false });
</script>

<script lang="ts">
	import { goto } from "$app/navigation";
	import { handleSignOut } from "$lib/services/auth-service";
	import { buttonVariants } from "$lib/components/ui/button";
	import { Button } from "$lib/components/ui/button";
	import * as Dialog from "$lib/components/ui/dialog";

	let signingOut = $state(false);

	const handleClick = async () => {
		signingOut = true;
		try {
			await handleSignOut();
			goto("/login");
			signOutDialog.open = false;
		} catch {
			// TODO: show error as an alert
		} finally {
			signingOut = false;
		}
	}
</script>

<Dialog.Root bind:open={signOutDialog.open}>
	<Dialog.Content class="max-w-[425px]">
		<Dialog.Header>
			<Dialog.Title>Confirm Sign Out</Dialog.Title>
			<Dialog.Description>
				Please confirm if you would like to sign out of your account.
			</Dialog.Description>
		</Dialog.Header>
		<Dialog.Footer>
			<Dialog.Close class={buttonVariants({ variant: "outline" })}>
				Cancel
			</Dialog.Close>
			<Button variant="destructive" onclick={handleClick} disabled={signingOut}>
				Sign Out
			</Button>
		</Dialog.Footer>
	</Dialog.Content>
</Dialog.Root>
