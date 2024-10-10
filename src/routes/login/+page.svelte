<script>
  import { superForm, setError } from "sveltekit-superforms";
  import { zodClient } from "sveltekit-superforms/adapters";
  import { loginSchema } from "$lib/schemas/auth-schemas";
  import { handleSignIn } from "$lib/services/auth-service";
  import { username } from "$lib/stores/auth-store";
  import { Button } from "$lib/components/ui/button";
  import { Input } from "$lib/components/ui/input";
  import { ForgotPasswordDialog } from "$lib/components/dialogs/auth-dialogs/forgot-password-dialog";
  import { SignUpDialog } from "$lib/components/dialogs/auth-dialogs/sign-up-dialog";
  import { NewPasswordRequiredDialog } from "$lib/components/dialogs/auth-dialogs/new-password-required-dialog";
  import { MfaCodeDialog } from "$lib/components/dialogs/auth-dialogs/mfa-code-dialog";
  import { MfaSetupDialog } from "$lib/components/dialogs/auth-dialogs/mfa-setup-dialog";
  import LightSwitch from "$lib/components/light-switch.svelte";
  import * as Form from "$lib/components/ui/form";

  const handleValidForm = async form => {
    const formData = form.data;
    try {
      username.set(formData.email);
      await handleSignIn({
        username: formData.email,
        password: formData.password
      });
    } catch (err) {
      setError(form, "email", err.message);
    }
  };

  const form = superForm({}, {
    SPA: true,
    resetForm: false,
    clearOnSubmit: "errors",
    validators: zodClient(loginSchema),
    async onUpdate({ form }) {
      if (!form.valid) return;
      await handleValidForm(form);
    }
  });

  const { form: formData, enhance, submitting } = form;
</script>

<NewPasswordRequiredDialog />
<MfaCodeDialog />
<MfaSetupDialog />

<div class="flex w-full h-screen items-center justify-center overflow-hidden">
  <div class="absolute top-0 right-0 pr-3 pt-3 z-20">
    <LightSwitch />
  </div>

  <div class="w-full grid lg:grid-cols-2 min-h-[720px]">
    <div class="flex items-center justify-center py-12">
      <div class="mx-auto grid w-[350px] gap-6">
        <div class="grid gap-2 text-center">
          <h1 class="text-3xl font-bold">Login</h1>
          <p class="text-muted-foreground text-balance">
            Enter your email below to login to your account
          </p>
        </div>

        <form class="grid gap-4" method="POST" use:enhance>
          <Form.Field name="email" class="space-y-1" {form}>
            <Form.Control let:attrs>
              <Form.Label>Email</Form.Label>
              <Input
                id="email"
                placeholder="user@example.com"
                bind:value={$formData.email}
                {...attrs}
              />
            </Form.Control>
            <Form.FieldErrors />
          </Form.Field>

          <Form.Field name="password" class="space-y-1" {form}>
            <Form.Control let:attrs>
              <div class="flex items-center">
                <Form.Label>Password</Form.Label>
                <ForgotPasswordDialog />
              </div>

              <Input
                id="password"
                type="password"
                bind:value={$formData.password}
                {...attrs}
              />
            </Form.Control>
            <Form.FieldErrors />
          </Form.Field>

          <Button type="submit" class="w-full" disabled={$submitting}>
            Login
          </Button>
        </form>

        <div class="mt-4 text-center text-sm">
          Don't have an account?
          <SignUpDialog />
        </div>
      </div>
    </div>

    <div class="bg-muted hidden lg:block">
      <img
        src="/images/placeholder.svg"
        alt="placeholder"
        width="1920"
        height="1080"
        class="h-full w-full object-cover dark:brightness-[0.2] dark:grayscale"
      />
    </div>
  </div>
</div>
