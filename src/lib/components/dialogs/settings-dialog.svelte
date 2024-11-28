<script module>
	export let settingsDialog = $state({ open: true });
</script>

<script lang="ts">
	import type { Component } from "svelte";
	import { Separator } from "$lib/components/ui/separator";
	import * as Dialog from "$lib/components/ui/dialog";
	import * as Tabs from "$lib/components/ui/tabs";
	import Sidebar from "$lib/components/settings/sidebar.svelte";
	import ProfilePage from "$lib/components/settings/profile-page.svelte";

	interface Page {
		name: string;
		label: string;
		icon: string;
		page: Component
	}

	const pages: Page[] = [
		{ name: "account-settings", label: "Account Settings", icon: "fa-user", page: ProfilePage },
		{ name: "notifications", label: "Notifications", icon: "fa-bell", page: Separator },
		{ name: "privacy", label: "Privacy", icon: "fa-lock", page: Separator },
		{ name: "preferences", label: "Preferences", icon: "fa-cog", page: Separator },
		{ name: "integration", label: "Integration", icon: "fa-link", page: Separator },
		{ name: "security", label: "Security", icon: "fa-shield", page: Separator },
		{ name: "billing", label: "Billing", icon: "fa-money-bill", page: Separator },
	];
</script>

<Dialog.Root bind:open={settingsDialog.open}>
	<Dialog.Content class="max-w-[800px] max-h-[650px] h-full flex flex-col">
		<Dialog.Header class="h-fit">
			<Dialog.Title>Settings</Dialog.Title>
			<div class="-mx-6">
				<Separator class="my-4" />
			</div>
		</Dialog.Header>
		<Tabs.Root value={pages[0].name} class="flex gap-x-4 h-full">
			<Sidebar {pages} />
			{#each pages as { name, page: Page }}
				<Tabs.Content value={name} class="w-full">
					<Page />
				</Tabs.Content>
			{/each}
		</Tabs.Root>
	</Dialog.Content>
</Dialog.Root>
