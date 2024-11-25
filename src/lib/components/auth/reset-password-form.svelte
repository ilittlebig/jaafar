<script lang="ts">
	import { superForm, type Infer } from "sveltekit-superforms";
	import { zodClient } from "sveltekit-superforms/adapters";
	import { resetPasswordFormSchema, type ResetPasswordFormSchema } from "$lib/schemas/auth";
	import { Input } from "$lib/components/ui/input";
	import * as Form from "$lib/components/ui/form";

	interface Props {
		data: Infer<ResetPasswordFormSchema>;
	}
	let { data }: Props = $props();

	const form = superForm(data, {
    SPA: true,
    resetForm: false,
    clearOnSubmit: "errors",
    validators: zodClient(resetPasswordFormSchema),
  });

  const { form: formData, enhance } = form;
</script>

<form class="grid gap-4" method="POST" use:enhance>
	<Form.Field name="username" {form}>
		<Form.Control>
			{#snippet children({ props })}
				<Form.Label>Email</Form.Label>
				<Input
					{...props}
					placeholder="user@example.com"
					bind:value={$formData.username}
				/>
			{/snippet}
		</Form.Control>
		<Form.FieldErrors />
	</Form.Field>

	<div class="flex justify-end">
		<Form.Button>Submit</Form.Button>
	</div>
</form>
