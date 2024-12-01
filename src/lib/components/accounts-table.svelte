<script lang="ts">
	import { accountsStore } from "$lib/stores/accounts-store.svelte";
  import { Badge } from "$lib/components/ui/badge";
  import * as Table from "$lib/components/ui/table";
</script>

<Table.Root>
	<Table.Header class="bg-accent">
		<Table.Row>
			<Table.Head>Email</Table.Head>
			<Table.Head>First Name</Table.Head>
			<Table.Head>Last Name</Table.Head>
			<Table.Head>Phone</Table.Head>
			<Table.Head>Address 1</Table.Head>
			<Table.Head>Address 2</Table.Head>
			<Table.Head>City</Table.Head>
			<Table.Head>Postcode</Table.Head>
			<Table.Head>Country</Table.Head>
			<Table.Head>Actions</Table.Head>
		</Table.Row>
	</Table.Header>
	<Table.Body>
		{#each accountsStore as account, i (i)}
			<Table.Row class="bg-background">
				<Table.Cell>{account.email}</Table.Cell>
				<Table.Cell>
					<Badge variant="outline" class="capitalize">{account.firstname}</Badge>
				</Table.Cell>
				<Table.Cell>
					<Badge variant="outline" class="capitalize">{account.lastname}</Badge>
				</Table.Cell>
				<Table.Cell>
					<Badge variant="outline" class="capitalize">{account.phone}</Badge>
				</Table.Cell>
				<Table.Cell>
					<Badge variant="outline" class="capitalize">{account.address1}</Badge>
				</Table.Cell>
				<Table.Cell>
					<Badge variant="outline" class="capitalize">{account.address2}</Badge>
				</Table.Cell>
				<Table.Cell>
					<Badge variant="outline" class="capitalize">{account.city}</Badge>
				</Table.Cell>
				<Table.Cell>
					<Badge variant="outline" class="capitalize">{account.postcode}</Badge>
				</Table.Cell>
				<Table.Cell>{account.country}</Table.Cell>
				<Table.Cell></Table.Cell>
			</Table.Row>
		{/each}
	</Table.Body>
</Table.Root>
