<script lang="ts">
	import * as Card from "$lib/components/ui/card";
	import LoginForm from "$lib/components/auth/login-form.svelte";
	import SignUpDialog from "$lib/components/dialogs/auth/sign-up-dialog.svelte";
</script>

<div class="flex items-center justify-center h-screen w-full relative">
	<Card.Root class="mx-auto w-full max-w-sm">
		<Card.Header>
			<Card.Title class="text-2xl">Login</Card.Title>
			<Card.Description>Enter your email below to login to your account</Card.Description>
		</Card.Header>
		<Card.Content>
			<LoginForm />
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
