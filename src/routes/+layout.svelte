<script lang="ts">
  import "../app.css";
  import Titlebar from "$lib/components/titlebar.svelte";
	let { children } = $props();
</script>

<svelte:head>
	<link rel="stylesheet" href="/fontawesome/css/all.css" />
</svelte:head>

<Titlebar />
{@render children()}
