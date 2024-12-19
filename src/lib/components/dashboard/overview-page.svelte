<script lang="ts">
	import * as Card from "$lib/components/ui/card";
</script>

<div class="grid gap-4 md:grid-cols-2 lg:grid-cols-4">
	<Card.Root>
		<Card.Header class="flex flex-row items-center justify-between space-y-0 pb-2" >
			<Card.Title class="text-sm font-medium">Product Carting Success</Card.Title>
			<i class="fa-regular fa-shopping-cart text-muted-foreground text-sm"></i>
		</Card.Header>
		<Card.Content>
			<div class="text-2xl font-bold">73%</div>
			<p class="text-muted-foreground text-xs">+20.1% from last month</p>
		</Card.Content>
	</Card.Root>
	<Card.Root>
		<Card.Header class="flex flex-row items-center justify-between space-y-0 pb-2" >
			<Card.Title class="text-sm font-medium">Missed Opportunities</Card.Title>
			<i class="fa-regular fa-arrow-trend-down text-muted-foreground text-sm"></i>
		</Card.Header>
		<Card.Content>
			<div class="text-2xl font-bold">6</div>
			<p class="text-muted-foreground text-xs">+20.1% from last month</p>
		</Card.Content>
	</Card.Root>
	<Card.Root>
		<Card.Header class="flex flex-row items-center justify-between space-y-0 pb-2" >
			<Card.Title class="text-sm font-medium">Total Estimated Cart Value</Card.Title>
			<i class="fa-regular fa-sack-dollar text-muted-foreground text-sm"></i>
		</Card.Header>
		<Card.Content>
			<div class="text-2xl font-bold">$5,230.00</div>
			<p class="text-muted-foreground text-xs">+20.1% from last month</p>
		</Card.Content>
	</Card.Root>
	<Card.Root>
		<Card.Header class="flex flex-row items-center justify-between space-y-0 pb-2" >
			<Card.Title class="text-sm font-medium">Cost per Successful Task</Card.Title>
			<i class="fa-regular fa-tag text-muted-foreground text-sm"></i>
		</Card.Header>
		<Card.Content>
			<div class="text-2xl font-bold">$0.25</div>
			<p class="text-muted-foreground text-xs">+20.1% from last month</p>
		</Card.Content>
	</Card.Root>
</div>
