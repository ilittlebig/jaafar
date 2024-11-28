<script lang="ts">
	import Header from "$lib/components/header.svelte";
	import Sidebar from "$lib/components/sidebar.svelte";
	import SignOutDialog from "$lib/components/dialogs/auth/sign-out-dialog.svelte";

	let { children } = $props();
</script>

<SignOutDialog />

<div class="grid min-h-screen w-full grid-cols-[85px_1fr]">
	<Sidebar />
	<div class="flex flex-col">
		<Header />
		<div class="flex flex-1 flex-col gap-6 p-6 bg-muted/40">
			{@render children()}
		</div>
	</div>
</div>
