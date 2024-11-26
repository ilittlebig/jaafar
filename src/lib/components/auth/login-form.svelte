<script module>
	import { handleSignIn } from "$lib/services/auth-service";
	import { loginFormSchema } from "$lib/schemas/auth";
</script>

<script lang="ts">
	import { Button } from "$lib/components/ui/button";
	import * as Form from "$lib/components/ui/form";
	import FormComponent from "$lib/components/auth/form-component.svelte";
	import ResetPasswordDialog from "$lib/components/dialogs/auth/reset-password-dialog.svelte";

	const fields = [
		{ name: "username", label: "Email", placeholder: "user@example.com" },
		{ name: "password", label: "Password", type: "password", component: ResetPasswordDialog },
	];

	const data = { username: "", password: "" };
</script>

<FormComponent {data} {fields} onsubmit={handleSignIn} schema={loginFormSchema}>
	{#snippet children(submitting)}
		<Form.Button disabled={submitting}>Submit</Form.Button>
		<Button variant="outline">Login with Google</Button>
	{/snippet}
</FormComponent>
