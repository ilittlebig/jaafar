<script lang="ts">
	import {
		superForm,
		setError,
		type SuperValidated,
		type Infer,
	} from "sveltekit-superforms";
	import { zodClient } from "sveltekit-superforms/adapters";
	import { resetPasswordFormSchema, type ResetPasswordFormSchema } from "$lib/schemas/auth";
	import { Input } from "$lib/components/ui/input";
	import * as Form from "$lib/components/ui/form";

	interface Props {
		onsubmit: (username: string) => Promise<void>;
	}

	let { onsubmit }: Props = $props();
	const data = { username: "" };

  const handleValidForm = async (form: SuperValidated<Infer<ResetPasswordFormSchema>>) => {
    const formData = form.data;
    try {
      await onsubmit(formData.username);
    } catch (err: any) {
      setError(form, "username", err.message);
    }
  };

	const form = superForm(data, {
    SPA: true,
    resetForm: false,
    clearOnSubmit: "errors",
    validators: zodClient(resetPasswordFormSchema),
    async onUpdate({ form }) {
      if (form.valid) await handleValidForm(form);
    }
  });

  const { form: formData, enhance, submitting } = form;
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
		<Form.Button disabled={$submitting}>Submit</Form.Button>
	</div>
</form>
