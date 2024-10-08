<script context="module">
  import { writable } from "svelte/store";
  export let open = writable(false);
</script>

<script>
  import { superForm } from "sveltekit-superforms";
  import { zodClient } from "sveltekit-superforms/adapters";
  import { confirmPasswordSchema } from "$lib/schemas/forgot-password-schemas";
  import { Button } from "$lib/components/ui/button";
  import { Input } from "$lib/components/ui/input";
  import * as Form from "$lib/components/ui/form";
  import * as Dialog from "$lib/components/ui/dialog";

  export let onSubmit;

  const form = superForm({}, {
    SPA: true,
    resetForm: false,
    clearOnSubmit: "errors",
    validators: zodClient(confirmPasswordSchema),
    async onUpdate({ form }) {
      if (!form.valid) return;
      await onSubmit(form);
    }
  });

  const { form: formData, enhance, submitting } = form;
</script>

<Dialog.Root bind:open={$open}>
  <Dialog.Content class="max-w-[425px]">
    <Dialog.Header>
      <Dialog.Title>
        Enter Verification Code
      </Dialog.Title>
      <Dialog.Description>
        We have sent a verification code to your email. Please enter it below.
      </Dialog.Description>
    </Dialog.Header>

    <form class="grid gap-4" method="POST" use:enhance>
      <Form.Field name="confirmationCode" class="space-y-1" {form}>
        <Form.Control let:attrs>
          <Form.Label>Confirmation Code</Form.Label>
          <Input
            id="confirmation-code"
            bind:value={$formData.confirmationCode}
            {...attrs}
          />
        </Form.Control>
        <Form.FieldErrors />
      </Form.Field>

      <Form.Field name="newPassword" class="space-y-1" {form}>
        <Form.Control let:attrs>
          <Form.Label>New Password</Form.Label>
          <Input
            id="new-password"
            type="password"
            bind:value={$formData.newPassword}
            {...attrs}
          />
        </Form.Control>
        <Form.FieldErrors />
      </Form.Field>

      <Form.Field name="confirmPassword" class="space-y-1" {form}>
        <Form.Control let:attrs>
          <Form.Label>Confirm Password</Form.Label>
          <Input
            id="confirm-password"
            type="password"
            bind:value={$formData.confirmPassword}
            {...attrs}
          />
        </Form.Control>
        <Form.FieldErrors />
      </Form.Field>

      <Dialog.Footer>
        <Button variant="outline">
          Cancel
        </Button>
        <Button type="submit" disabled={$submitting}>
          Change Password
        </Button>
      </Dialog.Footer>
    </form>
  </Dialog.Content>
</Dialog.Root>
