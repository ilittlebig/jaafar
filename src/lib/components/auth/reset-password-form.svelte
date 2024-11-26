<script module>
	import { handleResetPassword } from "$lib/services/auth-service";
	import { resetPasswordFormSchema } from "$lib/schemas/auth";
</script>

<script lang="ts">
	import * as Form from "$lib/components/ui/form";
	import FormComponent from "$lib/components/auth/form-component.svelte";

	const fields = [
		{ name: "username", label: "Email", placeholder: "user@example.com" }
	];

	const data = { username: "" };
</script>

<FormComponent {data} {fields} onsubmit={handleResetPassword} schema={resetPasswordFormSchema}>
	<div class="flex justify-end">
		<Form.Button>Send Code</Form.Button>
	</div>
</FormComponent>
