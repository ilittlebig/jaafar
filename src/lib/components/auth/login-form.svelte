<script lang="ts">
	import { loginFormSchema } from "$lib/schemas/auth";
	import { Button } from "$lib/components/ui/button";
	import * as Form from "$lib/components/ui/form";
	import FormComponent from "$lib/components/auth/form-component.svelte";
	import ResetPasswordDialog from "$lib/components/dialogs/auth/reset-password-dialog.svelte";

	interface Props {
		onsubmit: (formData: Record<string, unknown>) => Promise<void>;
	}

	let { onsubmit }: Props = $props();
	const data = { username: "", password: "" };

	const fields = [
		{
			name: "username",
			label: "Email",
			placeholder: "user@example.com",
		},
		{
			name: "password",
			label: "Password",
			type: "password",
			component: ResetPasswordDialog,
		},
	];
</script>

<FormComponent {data} {fields} {onsubmit} schema={loginFormSchema}>
	<Form.Button>Submit</Form.Button>
	<Button variant="outline">Login with Google</Button>
</FormComponent>
