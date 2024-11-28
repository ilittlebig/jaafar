<script lang="ts">
	import type { Component } from "svelte";
	import { Button } from "$lib/components/ui/button";
	import * as Tabs from "$lib/components/ui/tabs";

	interface Page {
		name: string;
		label: string;
		page: Component
	}

	interface Props {
		pages: Page[];
	}

	let { pages }: Props = $props();
</script>

<div class="flex flex-col w-48 h-full">
	{#each pages as page}
		<Tabs.Trigger value={page.name}>
			{#snippet child({ props })}
				<Button
					{...props}
					variant="ghost"
					class="w-fit data-[state=active]:bg-accent"
				>{page.label}</Button>
			{/snippet}
		</Tabs.Trigger>
	{/each}
</div>
