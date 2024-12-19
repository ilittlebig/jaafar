<script lang="ts">
	import * as Tooltip from "$lib/components/ui/tooltip";
</script>

<div class="flex gap-x-2">
	<Tooltip.Provider disableHoverableContent={true}>
		<Tooltip.Root>
			<Tooltip.Trigger>
				<button aria-label="Mark as read" class="text-xs text-muted-foreground hover:text-foreground">
					<i class="fa-regular fa-eye"></i>
				</button>
			</Tooltip.Trigger>
			<Tooltip.Content side="bottom" align="end">
				Mark as read
			</Tooltip.Content>
		</Tooltip.Root>
		<Tooltip.Root>
			<Tooltip.Trigger>
				<button aria-label="Remove notification" class="text-xs text-muted-foreground hover:text-foreground">
					<i class="fa-regular fa-trash-can"></i>
				</button>
			</Tooltip.Trigger>
			<Tooltip.Content side="bottom" align="end">
				Remove notification
			</Tooltip.Content>
		</Tooltip.Root>
	</Tooltip.Provider>
</div>
