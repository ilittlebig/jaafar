<script lang="ts">
	import { Button } from "$lib/components/ui/button";
	import * as Tabs from "$lib/components/ui/tabs";
	import OverviewPage from "$lib/components/dashboard/overview-page.svelte";
</script>

<div class="flex items-center justify-between">
  <h1 class="font-semibold text-2xl">Dashboard</h1>
	<Button>
		<i class="fa-regular fa-download"></i>
		Download
	</Button>
</div>

<Tabs.Root value="overview" class="space-y-4">
	<Tabs.List>
		<Tabs.Trigger value="overview">Overview</Tabs.Trigger>
		<Tabs.Trigger value="analytics">Analytics</Tabs.Trigger>
		<Tabs.Trigger value="reports">Reports</Tabs.Trigger>
		<Tabs.Trigger value="notifications">Notifications</Tabs.Trigger>
	</Tabs.List>
	<Tabs.Content value="overview" class="space-y-4">
		<OverviewPage />
	</Tabs.Content>
</Tabs.Root>
