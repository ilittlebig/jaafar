<script lang="ts">
	import { cn } from "$lib/utils";
	import { Button } from "$lib/components/ui/button";
	import { Separator } from "$lib/components/ui/separator";
  import * as Popover from "$lib/components/ui/popover";

	let open = $state(false);
</script>

<header class="bg-background flex justify-end items-center gap-1 border-b h-[44px] px-2 shrink-0">
	<Popover.Root bind:open>
		<Popover.Trigger>
			{#snippet child({ props })}
				<Button
					variant="ghost"
					size="icon"
					class="rounded-lg z-20"
					aria-label="Notifications"
					{...props}
				>
					<i class={cn(open ? "fa-solid" : "fa-regular", "fa-bell text-lg")}></i>
				</Button>
			{/snippet}
		</Popover.Trigger>
		<Popover.Content class="w-80" align="end">
			<div class="flex flex-col gap-y-4">
				<h4 class="font-medium leading-none">Notifications</h4>
				<div class="-mx-4">
					<Separator />
				</div>
				<div class="flex flex-col">
					<div class="flex gap-x-2 items-center">
						<p class="text-sm">Jaafar</p>
						<p class="text-xs text-muted-foreground">Today @ 10:15am</p>
					</div>
					<p class="text-sm text-muted-foreground">A new update is available.</p>
				</div>
				<div class="-mx-4">
					<Separator />
				</div>
				<div class="flex flex-col">
					<div class="flex gap-x-2 items-center">
						<p class="text-sm">Jaafar</p>
						<p class="text-xs text-muted-foreground">Today @ 10:15am</p>
					</div>
					<p class="text-sm text-muted-foreground">A new update is available.</p>
				</div>
				<div class="-mx-4">
					<Separator />
				</div>
				<div class="flex flex-col">
					<div class="flex gap-x-2 items-center">
						<p class="text-sm">Jaafar</p>
						<p class="text-xs text-muted-foreground">Today @ 10:15am</p>
					</div>
					<p class="text-sm text-muted-foreground">A new update is available.</p>
				</div>
			</div>
		</Popover.Content>
	</Popover.Root>
</header>
