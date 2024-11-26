<script module>
	import { handleSignIn } from "$lib/services/auth-service";
	import { loginFormSchema } from "$lib/schemas/auth";
</script>

<script lang="ts">
	import { Button } from "$lib/components/ui/button";
	import * as Card from "$lib/components/ui/card";
	import * as Form from "$lib/components/ui/form";
	import FormComponent from "$lib/components/auth/form-component.svelte";
	import ResetPasswordDialog from "$lib/components/dialogs/auth/reset-password-dialog.svelte";
	import SignUpDialog from "$lib/components/dialogs/auth/sign-up-dialog.svelte";

	const fields = [
		{ name: "username", label: "Email", placeholder: "user@example.com" },
		{ name: "password", label: "Password", type: "password", component: ResetPasswordDialog },
	];

	const data = { username: "", password: "" };
</script>

<div class="flex items-center justify-center h-screen w-full relative">
	<Card.Root class="mx-auto w-full max-w-sm">
		<Card.Header>
			<Card.Title class="text-2xl">Login</Card.Title>
			<Card.Description>Enter your email below to login to your account</Card.Description>
		</Card.Header>
		<Card.Content>
			<FormComponent {data} {fields} onsubmit={handleSignIn} schema={loginFormSchema}>
				{#snippet children(submitting)}
					<Form.Button disabled={submitting}>Submit</Form.Button>
					<Button variant="outline">Login with Google</Button>
				{/snippet}
			</FormComponent>
			<div class="mt-4 text-center text-sm">
				Don't have an account?
				<SignUpDialog />
			</div>
		</Card.Content>
	</Card.Root>

	<div class="flex gap-x-8 absolute bottom-0 p-4">
		<p class="text-sm text-muted-foreground">
			Made with
			<i class="fa-solid fa-heart text-destructive"></i>
			by Jaafar.
		</p>
	</div>
</div>
