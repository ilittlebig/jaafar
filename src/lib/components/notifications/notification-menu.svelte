<script lang="ts">
	import { cn } from "$lib/utils";
	import { Button } from "$lib/components/ui/button";
	import { Separator } from "$lib/components/ui/separator";
  import * as Popover from "$lib/components/ui/popover";
	import * as Tooltip from "$lib/components/ui/tooltip";
	import NotificationList from "$lib/components/notifications/notification-list.svelte";

	let open = $state(false);
</script>

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
				<Tooltip.Provider>
					<Tooltip.Root>
						<Tooltip.Trigger>
							<i class={cn(open ? "fa-solid" : "fa-regular", "fa-bell text-lg")}></i>
						</Tooltip.Trigger>
						<Tooltip.Content>Notifications</Tooltip.Content>
					</Tooltip.Root>
				</Tooltip.Provider>
			</Button>
		{/snippet}
	</Popover.Trigger>
	<Popover.Content class="w-80" align="end">
		<div class="flex flex-col gap-y-4">
			<h4 class="font-medium leading-none">Notifications</h4>
			<div class="-mx-4">
				<Separator />
			</div>
			<NotificationList />
		</div>
	</Popover.Content>
</Popover.Root>
