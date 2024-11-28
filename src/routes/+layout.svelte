<script lang="ts">
	import { onDestroy, onMount } from "svelte";
	import { checkForUpdates } from "$lib/services/updater-service";
	import { loadSettings } from "$lib/services/settings-service.svelte";
  import Titlebar from "$lib/components/titlebar.svelte";
  import "../app.css";

	let updateInterval: ReturnType<typeof setInterval>;
	let { children } = $props();

	onMount(async () => {
		await loadSettings();
		await checkForUpdates();
		updateInterval = setInterval(checkForUpdates, 12 * 60 * 60 * 1000);
	});

	onDestroy(() => {
		if (updateInterval) clearInterval(updateInterval);
	});
</script>

<svelte:head>
	<link rel="stylesheet" href="/fontawesome/css/all.css" />
</svelte:head>

<Titlebar />
{@render children()}
