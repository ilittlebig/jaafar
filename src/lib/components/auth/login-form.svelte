<script lang="ts">
	import { superForm, type Infer } from "sveltekit-superforms";
	import { zodClient } from "sveltekit-superforms/adapters";
	import { loginFormSchema, type LoginFormSchema } from "$lib/schemas/auth";
	import { Input } from "$lib/components/ui/input";
	import { Button } from "$lib/components/ui/button";
	import * as Form from "$lib/components/ui/form";
	import ResetPasswordDialog from "$lib/components/dialogs/auth/reset-password-dialog.svelte";

	interface Props {
		data: Infer<LoginFormSchema>;
	}
	let { data }: Props = $props();

	const form = superForm(data, {
    SPA: true,
    resetForm: false,
    clearOnSubmit: "errors",
    validators: zodClient(loginFormSchema),
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

	<Form.Field name="password" {form}>
		<Form.Control>
			{#snippet children({ props })}
				<div class="flex items-center">
					<Form.Label>Password</Form.Label>
					<ResetPasswordDialog />
				</div>
				<Input {...props} bind:value={$formData.password} />
			{/snippet}
		</Form.Control>
		<Form.FieldErrors />
	</Form.Field>

	<Form.Button>Submit</Form.Button>
	<Button variant="outline">Login with Google</Button>
</form>
