<script lang="ts">
	import { handleSignUp } from "$lib/services/auth-service";
	import { signUpFormSchema } from "$lib/schemas/auth";
	import { Button } from "$lib/components/ui/button";
	import * as Card from "$lib/components/ui/card";
	import * as Form from "$lib/components/ui/form";
	import FormComponent from "$lib/components/auth/form-component.svelte";

	const fields = [
		{ name: "username", label: "Email", placeholder: "user@example.com" },
		{ name: "password", label: "Password", type: "password" },
		{ name: "confirmPassword", label: "Confirm Password", type: "password" },
	];

	const data = { username: "", password: "" };
</script>

<Card.Root class="mx-auto w-full max-w-sm">
	<Card.Header>
		<Card.Title class="text-2xl">Sign Up</Card.Title>
		<Card.Description>Create your account by filling in the details below to get started.</Card.Description>
	</Card.Header>
	<Card.Content>
		<FormComponent {data} {fields} onsubmit={handleSignUp} schema={signUpFormSchema}>
			{#snippet children(submitting)}
				<p class="text-sm text-center">
					By creating an account, I agree to
					<button type="button" class="hover:underline text-primary">Terms & Conditions</button>
				</p>
				<Form.Button disabled={submitting}>Create Account</Form.Button>
			{/snippet}
		</FormComponent>
		<div class="mt-4 text-center text-sm">
			Already have an account?
			<Button href="/login" variant="link" class="text-sm h-4 py-0 px-0 font-normal leading-none">
				Sign in
			</Button>
		</div>
	</Card.Content>
</Card.Root>
