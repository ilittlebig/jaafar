<script lang="ts">
	let { children } = $props();
</script>

<div class="flex items-center justify-center h-screen w-full relative">
	{@render children()}
	<div class="flex gap-x-8 absolute bottom-0 p-4">
		<p class="text-sm text-muted-foreground">
			Made with
			<i class="fa-solid fa-heart text-destructive"></i>
			by Jaafar.
		</p>
	</div>
</div>
