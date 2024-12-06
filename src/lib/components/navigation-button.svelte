<script lang="ts">
	import { page } from "$app/stores";
	import { cn } from "$lib/utils";
	import { Button } from "$lib/components/ui/button";
	import * as Tooltip from "$lib/components/ui/tooltip";

	interface Navigation {
		label: string;
		icon: string;
		href: string;
	}

	interface Props {
		navigation: Navigation;
	}

	let {
		navigation,
	}: Props = $props();

	const isActive = $derived($page.url.pathname === navigation.href);
</script>

<Tooltip.Provider delayDuration={0}>
	<Tooltip.Root>
		<Tooltip.Trigger>
			{#snippet child({ props })}
				<Button
					{...props}
					variant="ghost"
					size="icon"
					class={cn(isActive ?? "bg-muted", "rounded-lg")}
					href={navigation.href}
					aria-label={navigation.label}
				>
					<i class={cn("text-lg", isActive ? "fa-solid" : "fa-regular", navigation.icon)}></i>
				</Button>
			{/snippet}
		</Tooltip.Trigger>

		<Tooltip.Content side="right" sideOffset={5}>
			{navigation.label}
		</Tooltip.Content>
	</Tooltip.Root>
</Tooltip.Provider>
