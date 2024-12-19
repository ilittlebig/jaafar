<script lang="ts">
	import NotificationActionButtons from "$lib/components/notifications/notification-action-buttons.svelte";
</script>

<div class="flex flex-col">
	<div class="flex justify-between items-center">
		<div class="flex gap-x-2 items-center">
			<p class="text-sm">Jaafar</p>
			<p class="text-xs text-muted-foreground">Today @ 10:15am</p>
		</div>
		<NotificationActionButtons />
	</div>
	<p class="text-sm text-muted-foreground">A new update is available.</p>
</div>
