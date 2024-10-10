<script context="module">
  import { writable } from "svelte/store";
  export let open = writable(false);
</script>

<script>
  import { superForm, setError } from "sveltekit-superforms";
  import { zodClient } from "sveltekit-superforms/adapters";
  import { mfaCodeSchema } from "$lib/schemas/auth-schemas";
  import { handleChallengeResponse } from "$lib/services/auth-service";
  import { Button } from "$lib/components/ui/button";
  import { Input } from "$lib/components/ui/input";
  import * as Dialog from "$lib/components/ui/dialog";
  import * as Form from "$lib/components/ui/form";

  const handleValidForm = async form => {
    const formData = form.data;
    try {
      await handleChallengeResponse({ response: formData.code });
    } catch (err) {
      setError(form, "code", err.message);
    }
  }

  const form = superForm({}, {
    SPA: true,
    resetForm: false,
    clearOnSubmit: "errors",
    validators: zodClient(mfaCodeSchema),
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
        Enter MFA Code
      </Dialog.Title>
      <Dialog.Description>
        Enter the 6-digit code from your authenticator app to continue.
      </Dialog.Description>
    </Dialog.Header>

    <form class="grid gap-4" method="POST" use:enhance>
      <Form.Field name="code" class="space-y-1" {form}>
        <Form.Control let:attrs>
          <Form.Label>Code</Form.Label>
          <Input
            id="code"
            bind:value={$formData.code}
            {...attrs}
          />
        </Form.Control>
        <Form.FieldErrors />
      </Form.Field>

      <Dialog.Footer>
        <Button variant="outline" on:click={() => $open = false}>
          Cancel
        </Button>
        <Button type="submit" disabled={$submitting}>
          Submit Code
        </Button>
      </Dialog.Footer>
    </form>
  </Dialog.Content>
</Dialog.Root>
