<script module>
	export let settingsDialog = $state({ open: true });
</script>

<script lang="ts">
	import type { Component } from "svelte";
	import { Separator } from "$lib/components/ui/separator";
	import { ScrollArea } from "$lib/components/ui/scroll-area";
	import * as Dialog from "$lib/components/ui/dialog";
	import * as Tabs from "$lib/components/ui/tabs";
	import Sidebar from "$lib/components/settings/sidebar.svelte";
	import AccountSettingsPage from "$lib/components/settings/account-settings-page.svelte";
	import NotificationsPage from "$lib/components/settings/notifications-page.svelte";
	import PrivacyPage from "$lib/components/settings/privacy-page.svelte";
	import PreferencesPage from "$lib/components/settings/preferences-page.svelte";
	import IntegrationPage from "$lib/components/settings/integration-page.svelte";
	import SecurityPage from "$lib/components/settings/security-page.svelte";
	import BillingPage from "$lib/components/settings/billing-page.svelte";

	interface Page {
		name: string;
		label: string;
		icon: string;
		page: Component
	}

	const pages: Page[] = [
		{ name: "account-settings", label: "Account Settings", icon: "fa-user", page: AccountSettingsPage },
		{ name: "notifications", label: "Notifications", icon: "fa-bell", page: NotificationsPage },
		{ name: "privacy", label: "Privacy", icon: "fa-lock", page: PrivacyPage },
		{ name: "preferences", label: "Preferences", icon: "fa-cog", page: PreferencesPage },
		{ name: "integration", label: "Integration", icon: "fa-link", page: IntegrationPage },
		{ name: "security", label: "Security", icon: "fa-shield", page: SecurityPage },
		{ name: "billing", label: "Billing", icon: "fa-money-bill", page: BillingPage },
	];
</script>

<Dialog.Root bind:open={settingsDialog.open}>
	<Dialog.Content class="max-w-[800px] max-h-[650px] h-full flex flex-col px-0 pt-6 pb-0 gap-0">
		<Dialog.Header class="h-fit px-6">
			<Dialog.Title>Settings</Dialog.Title>
			<div class="-mx-6">
				<Separator class="mt-4" />
			</div>
		</Dialog.Header>
		<ScrollArea class="h-full" scrollHideDelay={0}>
			<Tabs.Root value={pages[0].name} class="flex gap-x-4 h-full pb-6 px-6 pt-4">
				<Sidebar {pages} />
				<div class="ml-52 w-full">
					{#each pages as { name, page: Page }}
						<Tabs.Content value={name} class="w-full">
							<Page />
						</Tabs.Content>
					{/each}
				</div>
			</Tabs.Root>
		</ScrollArea>
	</Dialog.Content>
</Dialog.Root>
