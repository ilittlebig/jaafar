<script lang="ts">
	import NavbarButton from "$lib/components/navbar-button.svelte";

	const navigations = [
		{ label: "Dashboard", icon: "fa-house", href: "/" },
		{ label: "Tasks", icon: "fa-table-layout", href: "/tasks" },
		{ label: "Proxies", icon: "fa-globe", href: "/proxies" },
	];
</script>

<nav class="grid gap-1">
	{#each navigations as navigation}
		<NavbarButton {navigation} />
	{/each}
</nav>
