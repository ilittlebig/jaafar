<script lang="ts">
  import { Badge } from "$lib/components/ui/badge";
  import * as Table from "$lib/components/ui/table";

	const invoices = [
		{
			"module": "oasis",
			"product": "sydney",
			"variant": null,
			"mode": "Auto Confirm",
			"email": "tobin_witting65106@web.de",
			"firstname": "Random",
			"lastname": "Random",
			"phone": "Random",
			"address1": "Random",
			"address2": "Random",
			"city": "Random",
			"postcode": "Random",
			"country": "AU",
			"state": null,
			"quantity": null,
		},
		{
			"module": "oasis",
			"product": "sydney",
			"variant": null,
			"mode": "Auto Confirm",
			"email": "tobin_witting65106@web.de",
			"firstname": "Random",
			"lastname": "Random",
			"phone": "Random",
			"address1": "Random",
			"address2": "Random",
			"city": "Random",
			"postcode": "Random",
			"country": "AU",
			"state": null,
			"quantity": null,
		},
		{
			"module": "oasis",
			"product": "sydney",
			"variant": null,
			"mode": "Auto Confirm",
			"email": "tobin_witting65106@web.de",
			"firstname": "Random",
			"lastname": "Random",
			"phone": "Random",
			"address1": "Random",
			"address2": "Random",
			"city": "Random",
			"postcode": "Random",
			"country": "AU",
			"state": null,
			"quantity": null,
		},
		{
			"module": "oasis",
			"product": "sydney",
			"variant": null,
			"mode": "Auto Confirm",
			"email": "tobin_witting65106@web.de",
			"firstname": "Random",
			"lastname": "Random",
			"phone": "Random",
			"address1": "Random",
			"address2": "Random",
			"city": "Random",
			"postcode": "Random",
			"country": "AU",
			"state": null,
			"quantity": null,
		},
		{
			"module": "oasis",
			"product": "sydney",
			"variant": null,
			"mode": "Auto Confirm",
			"email": "tobin_witting65106@web.de",
			"firstname": "Random",
			"lastname": "Random",
			"phone": "Random",
			"address1": "Random",
			"address2": "Random",
			"city": "Random",
			"postcode": "Random",
			"country": "AU",
			"state": null,
			"quantity": null,
		},
		{
			"module": "oasis",
			"product": "sydney",
			"variant": null,
			"mode": "Auto Confirm",
			"email": "tobin_witting65106@web.de",
			"firstname": "Random",
			"lastname": "Random",
			"phone": "Random",
			"address1": "Random",
			"address2": "Random",
			"city": "Random",
			"postcode": "Random",
			"country": "AU",
			"state": null,
			"quantity": null,
		},
		{
			"module": "oasis",
			"product": "sydney",
			"variant": null,
			"mode": "Auto Confirm",
			"email": "tobin_witting65106@web.de",
			"firstname": "Random",
			"lastname": "Random",
			"phone": "Random",
			"address1": "Random",
			"address2": "Random",
			"city": "Random",
			"postcode": "Random",
			"country": "AU",
			"state": null,
			"quantity": null,
		},
		{
			"module": "oasis",
			"product": "sydney",
			"variant": null,
			"mode": "Auto Confirm",
			"email": "tobin_witting65106@web.de",
			"firstname": "Random",
			"lastname": "Random",
			"phone": "Random",
			"address1": "Random",
			"address2": "Random",
			"city": "Random",
			"postcode": "Random",
			"country": "AU",
			"state": null,
			"quantity": null,
		},
		{
			"module": "oasis",
			"product": "sydney",
			"variant": null,
			"mode": "Auto Confirm",
			"email": "tobin_witting65106@web.de",
			"firstname": "Random",
			"lastname": "Random",
			"phone": "Random",
			"address1": "Random",
			"address2": "Random",
			"city": "Random",
			"postcode": "Random",
			"country": "AU",
			"state": null,
			"quantity": null,
		},
		{
			"module": "oasis",
			"product": "sydney",
			"variant": null,
			"mode": "Auto Confirm",
			"email": "tobin_witting65106@web.de",
			"firstname": "Random",
			"lastname": "Random",
			"phone": "Random",
			"address1": "Random",
			"address2": "Random",
			"city": "Random",
			"postcode": "Random",
			"country": "AU",
			"state": null,
			"quantity": null,
		},
		{
			"module": "oasis",
			"product": "sydney",
			"variant": null,
			"mode": "Auto Confirm",
			"email": "tobin_witting65106@web.de",
			"firstname": "Random",
			"lastname": "Random",
			"phone": "Random",
			"address1": "Random",
			"address2": "Random",
			"city": "Random",
			"postcode": "Random",
			"country": "AU",
			"state": null,
			"quantity": null,
		},
		{
			"module": "oasis",
			"product": "sydney",
			"variant": null,
			"mode": "Auto Confirm",
			"email": "tobin_witting65106@web.de",
			"firstname": "Random",
			"lastname": "Random",
			"phone": "Random",
			"address1": "Random",
			"address2": "Random",
			"city": "Random",
			"postcode": "Random",
			"country": "AU",
			"state": null,
			"quantity": null,
		},
		{
			"module": "oasis",
			"product": "sydney",
			"variant": null,
			"mode": "Auto Confirm",
			"email": "tobin_witting65106@web.de",
			"firstname": "Random",
			"lastname": "Random",
			"phone": "Random",
			"address1": "Random",
			"address2": "Random",
			"city": "Random",
			"postcode": "Random",
			"country": "AU",
			"state": null,
			"quantity": null,
		},
		{
			"module": "oasis",
			"product": "sydney",
			"variant": null,
			"mode": "Auto Confirm",
			"email": "maiya.kunde221@web.de",
			"firstname": "Random",
			"lastname": "Random",
			"phone": "Random",
			"address1": "Random",
			"address2": "Random",
			"city": "Random",
			"postcode": "Random",
			"country": "AU",
			"state": null,
			"quantity": null,
		},
		{
			"module": "oasis",
			"product": "sydney",
			"variant": null,
			"mode": "Auto Confirm",
			"email": "flossie_bahringer255@web.de",
			"firstname": "Random",
			"lastname": "Random",
			"phone": "Random",
			"address1": "Random",
			"address2": "Random",
			"city": "Random",
			"postcode": "Random",
			"country": "AU",
			"state": null,
			"quantity": null,
		},
		{
			"module": "oasis",
			"product": "sydney",
			"variant": null,
			"mode": "Auto Confirm",
			"email": "tracy62404@web.de",
			"firstname": "Random",
			"lastname": "Random",
			"phone": "Random",
			"address1": "Random",
			"address2": "Random",
			"city": "Random",
			"postcode": "Random",
			"country": "AU",
			"state": null,
			"quantity": null,
		}
	];
</script>

<Table.Root>
	<Table.Header class="bg-accent">
		<Table.Row>
			<Table.Head>Email</Table.Head>
			<Table.Head>Mode</Table.Head>
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
		{#each invoices as invoice, i (i)}
			<Table.Row class="bg-background">
				<Table.Cell>{invoice.email}</Table.Cell>
				<Table.Cell>
					<Badge class="text-nowrap">{invoice.mode}</Badge>
				</Table.Cell>
				<Table.Cell>
					<Badge variant="outline">{invoice.firstname}</Badge>
				</Table.Cell>
				<Table.Cell>
					<Badge variant="outline">{invoice.lastname}</Badge>
				</Table.Cell>
				<Table.Cell>
					<Badge variant="outline">{invoice.phone}</Badge>
				</Table.Cell>
				<Table.Cell>
					<Badge variant="outline">{invoice.address1}</Badge>
				</Table.Cell>
				<Table.Cell>
					<Badge variant="outline">{invoice.address2}</Badge>
				</Table.Cell>
				<Table.Cell>
					<Badge variant="outline">{invoice.city}</Badge>
				</Table.Cell>
				<Table.Cell>
					<Badge variant="outline">{invoice.postcode}</Badge>
				</Table.Cell>
				<Table.Cell>{invoice.country}</Table.Cell>
				<Table.Cell></Table.Cell>
			</Table.Row>
		{/each}
	</Table.Body>
</Table.Root>
