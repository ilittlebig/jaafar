<script>
  import { superForm, setError } from "sveltekit-superforms";
  import { zodClient } from "sveltekit-superforms/adapters";
  import { usernameSchema } from "$lib/schemas/forgot-password-schemas";
  import { handleResetPassword } from "$lib/services/auth-service";
  import { Button } from "$lib/components/ui/button";
  import { Input } from "$lib/components/ui/input";
  import * as Form from "$lib/components/ui/form";
  import * as Dialog from "$lib/components/ui/dialog";

  const handleValidForm = async form => {
    const formData = form.data;
    try {
      await handleResetPassword({ username: formData.email });
    } catch (err) {
      setError(form, "email", err.message);
    }
  }

  const form = superForm({}, {
    SPA: true,
    resetForm: false,
    clearOnSubmit: "errors",
    validators: zodClient(usernameSchema),
    async onUpdate({ form }) {
      if (!form.valid) return;
      await handleValidForm(form);
    }
  });

  const { form: formData, enhance, submitting } = form;
</script>

<Dialog.Root>
  <Dialog.Trigger asChild let:builder>
    <Button
      variant="link"
      builders={[builder]}
      class="ml-auto inline-block text-sm font-normal hover:underline p-0 h-auto"
    >
      Forgot your password?
    </Button>
  </Dialog.Trigger>

  <Dialog.Content class="max-w-[425px]">
    <Dialog.Header>
      <Dialog.Title>
        Reset Password
      </Dialog.Title>
      <Dialog.Description>
        Enter your email below to recieve a verification code to reset your password.
      </Dialog.Description>
    </Dialog.Header>

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

      <Dialog.Footer>
        <Button variant="outline" disabled={$submitting}>
          Cancel
        </Button>
        <Button type="submit">
          Send Code
        </Button>
      </Dialog.Footer>
    </form>
  </Dialog.Content>
</Dialog.Root>
