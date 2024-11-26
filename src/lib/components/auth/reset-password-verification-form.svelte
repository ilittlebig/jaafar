<script module>
	import { handleConfirmResetPassword } from "$lib/services/auth-service";
	import { resetPasswordVerificationFormSchema } from "$lib/schemas/auth";
</script>

<script lang="ts">
	import * as Form from "$lib/components/ui/form";
	import FormComponent from "$lib/components/auth/form-component.svelte";

	const fields = [
		{ name: "confirmationCode", label: "Confirmation Code" },
		{ name: "newPassword", label: "New Password", type: "password" },
		{ name: "confirmPassword", label: "Confirm Password", type: "password" },
	];

	const data = { confirmationCode: "", newPassword: "", confirmPassword: "" };
</script>

<FormComponent {data} {fields} onsubmit={handleConfirmResetPassword} schema={resetPasswordVerificationFormSchema}>
	<div class="flex justify-end">
		<Form.Button>Change Password</Form.Button>
	</div>
</FormComponent>
