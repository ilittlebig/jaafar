<script lang="ts">
	import { cn } from "$lib/utils";
	import NotificationActionButtons from "$lib/components/notifications/notification-action-buttons.svelte";
	let { isRead = false }: { isRead?: boolean } = $props();
</script>

<div class="flex flex-col">
	<div class="flex justify-between items-center">
		<div class={cn("flex gap-x-2 items-center", isRead && "line-through opacity-50")}>
			<p class="text-sm">Jaafar</p>
			<p class="text-xs text-muted-foreground">Today @ 10:15am</p>
		</div>
		<NotificationActionButtons {isRead} />
	</div>
	<p class={cn("text-sm text-muted-foreground", isRead && "line-through opacity-50")}>
		A new update is available.
	</p>
</div>
