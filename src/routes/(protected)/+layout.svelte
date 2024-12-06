<script lang="ts">
	import { onMount } from "svelte";
	import { loadAccounts } from "$lib/services/accounts-service";
	import { loadProxyGroups } from "$lib/services/proxies-service";
	import { loadSignups } from "$lib/services/signups-service";
	import { ScrollArea } from "$lib/components/ui/scroll-area";
	import Header from "$lib/components/header.svelte";
	import Sidebar from "$lib/components/sidebar.svelte";
	import SignOutDialog from "$lib/components/dialogs/auth/sign-out-dialog.svelte";
	import SettingsDialog from "$lib/components/dialogs/settings-dialog.svelte";

	let { children } = $props();

	onMount(async () => {
		await loadAccounts();
		await loadProxyGroups();
		await loadSignups();
	});
</script>

<SignOutDialog />
<SettingsDialog />

<div class="grid h-screen w-full grid-cols-[85px_1fr]">
  <Sidebar />
  <div class="flex flex-col w-full h-screen min-w-0">
    <Header />
		<div class="flex bg-muted/40 w-full h-screen overflow-y-scroll">
			<ScrollArea class="w-full">
				<div class="flex flex-col gap-y-6 h-full p-6">
					{@render children()}
				</div>
			</ScrollArea>
		</div>
  </div>
</div>
