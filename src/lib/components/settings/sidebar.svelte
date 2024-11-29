<script lang="ts">
	import type { Component } from "svelte";
	import { Button } from "$lib/components/ui/button";
	import * as Tabs from "$lib/components/ui/tabs";
    import {cn} from "$lib/utils";

	interface Page {
		name: string;
		label: string;
		icon: string;
		page: Component
	}

	interface Props {
		pages: Page[];
	}

	let { pages }: Props = $props();
</script>

<div class="flex flex-col w-48 h-full fixed">
	{#each pages as page}
		<Tabs.Trigger value={page.name}>
			{#snippet child({ props })}
				<Button
					{...props}
					variant="ghost"
					class="justify-start data-[state=active]:bg-accent"
				>
					<i class={cn("fa-regular", page.icon)}></i>
					{page.label}
				</Button>
			{/snippet}
		</Tabs.Trigger>
	{/each}
</div>
