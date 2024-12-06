<script lang="ts">
	let { children } = $props();
</script>

<div class="flex items-center justify-between">
	{@render children()}
</div>
