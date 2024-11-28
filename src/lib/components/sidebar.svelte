<script lang="ts">
	import NavigationButton from "$lib/components/navigation-button.svelte";
	import UserMenu from "$lib/components/user-menu.svelte";

	const navigations = [
		{ label: "Dashboard", icon: "fa-house", href: "/" },
		{ label: "Tasks", icon: "fa-table-layout", href: "/tasks" },
		{ label: "Signups", icon: "fa-user-tie-hair", href: "/signups" },
		{ label: "Proxies", icon: "fa-globe", href: "/proxies" },
	];
</script>

<div class="bg-background border-r h-full w-full">
	<div class="flex h-full max-h-screen flex-col justify-between items-center pb-4">
		<div class="flex w-full h-full flex-col items-center gap-4">
			<div class="flex items-center border-b w-full h-[44px] px-6"></div>
			<nav class="grid gap-1">
				{#each navigations as navigation}
					<NavigationButton {navigation} />
				{/each}
			</nav>
		</div>
		<UserMenu />
	</div>
</div>
