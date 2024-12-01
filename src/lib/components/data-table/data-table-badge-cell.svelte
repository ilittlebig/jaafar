<script lang="ts">
	import { Badge } from "$lib/components/ui/badge";

	interface Props {
		value: string;
		variant: "default" | "secondary" | "destructive" | "outline";
	}

	let {
		value,
		variant = "default",
	}: Props = $props();
</script>

<Badge class="capitalize" {variant}>
	{value}
</Badge>
