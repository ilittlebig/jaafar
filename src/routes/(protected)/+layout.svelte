<script lang="ts">
	import ScrollArea from "$lib/components/ui/scroll-area/scroll-area.svelte";
	import Header from "$lib/components/header.svelte";
	import Sidebar from "$lib/components/sidebar.svelte";
	import SignOutDialog from "$lib/components/dialogs/auth/sign-out-dialog.svelte";
	import SettingsDialog from "$lib/components/dialogs/settings-dialog.svelte";

	let { children } = $props();
</script>

<SignOutDialog />
<SettingsDialog />

<div class="grid h-screen w-full grid-cols-[85px_1fr]">
  <Sidebar />
  <div class="flex flex-col w-full h-screen min-w-0">
    <Header />
		<div class="flex bg-muted/40 w-full h-screen overflow-y-scroll">
			<ScrollArea>
				<div class="flex flex-col gap-y-6 h-full p-6">
					{@render children()}
				</div>
			</ScrollArea>
		</div>
  </div>
</div>
