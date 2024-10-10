<script context="module">
  import { writable } from "svelte/store";
  export let open = writable(false);
</script>

<script>
  import { superForm, setError } from "sveltekit-superforms";
  import { zodClient } from "sveltekit-superforms/adapters";
  import { newPasswordRequiredSchema } from "$lib/schemas/auth-schemas";
  import { handleChallengeResponse } from "$lib/services/auth-service";
  import * as Dialog from "$lib/components/ui/dialog";
  import PasswordResetForm from "$lib/components/password-reset-form.svelte";

  const handleValidForm = async form => {
    const formData = form.data;
    try {
      await handleChallengeResponse({ response: formData.newPassword });
    } catch (err) {
      setError(form, "newPassword", err.message);
    }
  };

  const form = superForm({}, {
    SPA: true,
    resetForm: false,
    clearOnSubmit: "errors",
    validators: zodClient(newPasswordRequiredSchema),
    async onUpdate({ form }) {
      if (!form.valid) return;
      await handleValidForm(form);
    }
  });

  const { form: formData, enhance, submitting } = form;
</script>

<Dialog.Root bind:open={$open} onOpenChange={() => form.reset()}>
  <Dialog.Content class="max-w-[425px]">
    <Dialog.Header>
      <Dialog.Title>
        Password Reset Required
      </Dialog.Title>
      <Dialog.Description>
        Enter the code sent to your email and your new password to continue.
      </Dialog.Description>
    </Dialog.Header>
    <PasswordResetForm codeRequired={false} {enhance} {form} {formData} {submitting} {open} />
  </Dialog.Content>
</Dialog.Root>
