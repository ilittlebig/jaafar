<script lang="ts">
	import { resetPasswordFormSchema } from "$lib/schemas/auth";
	import * as Form from "$lib/components/ui/form";
	import FormComponent from "$lib/components/auth/form-component.svelte";

	interface Props {
		onsubmit: (formData: Record<string, unknown>) => Promise<void>;
	}

	let { onsubmit }: Props = $props();
	const data = { username: "" };

	const fields = [{
		name: "username",
		label: "Email",
		placeholder: "user@example.com",
	}];
</script>

<FormComponent {data} {fields} {onsubmit} schema={resetPasswordFormSchema}>
	<div class="flex justify-end">
		<Form.Button>Submit</Form.Button>
	</div>
</FormComponent>
