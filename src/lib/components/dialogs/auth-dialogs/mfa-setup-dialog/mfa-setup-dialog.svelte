<script context="module">
  import { writable } from "svelte/store";
  export let open = writable(false);
</script>

<script>
  import QRCode from "qrcode";
  import { superForm, setError } from "sveltekit-superforms";
  import { zodClient } from "sveltekit-superforms/adapters";
  import { mfaCodeSchema } from "$lib/schemas/auth-schemas";
  import { handleChallengeResponse } from "$lib/services/auth-service";
  import { username, totpSetupDetails } from "$lib/stores/auth-store";
  import { Button } from "$lib/components/ui/button";
  import { Input } from "$lib/components/ui/input";
  import * as Dialog from "$lib/components/ui/dialog";
  import * as Form from "$lib/components/ui/form";
  import * as Tooltip from "$lib/components/ui/tooltip";

  let qrCodeUrl = "";
  let secret = "";

  // Formats the secret into a 4-digit space-separated string
  const formatSecret = secret => secret.match(/.{1,4}/g)?.join(" ") ?? "";

  const generateQRCode = async uri => {
    const url = await QRCode.toDataURL(uri, {
      errorCorrectionLevel: "M",
      type: "image/png",
      margin: 4,
    });
    return url;
  }

  const setupQRCode = async () => {
    try {
      const { sharedSecret, getSetupUri } = $totpSetupDetails;
      const uri = getSetupUri("AM System - Studio", $username);
      qrCodeUrl = await generateQRCode(uri);
      secret = sharedSecret;
    } catch (err) {
      console.error("Failed to generate QR code:", err);
    }
  };

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
  $: if ($totpSetupDetails) setupQRCode();
</script>

<Dialog.Root bind:open={$open} onOpenChange={() => form.reset()}>
  <Dialog.Content class="max-w-[425px]">
    <Dialog.Header>
      <Dialog.Title>
        Setup MFA
      </Dialog.Title>
      <Dialog.Description>
        Scan the QR code with any authenticator app and enter the code below.
      </Dialog.Description>
    </Dialog.Header>

    <div class="flex justify-center">
      <img
        src={qrCodeUrl}
        width="228"
        height="228"
        alt="QR Code"
      />
    </div>

    <div class="flex justify-center">
      <Tooltip.Root>
        <Tooltip.Trigger asChild let:builder>
          <Button builders={[builder]} variant="link" class="w-fit" tabindex="-1">
            Can't scan the QR code?
          </Button>
        </Tooltip.Trigger>
        <Tooltip.Content>
          <p>{formatSecret(secret)}</p>
        </Tooltip.Content>
      </Tooltip.Root>
    </div>

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
          Setup MFA
        </Button>
      </Dialog.Footer>
    </form>
  </Dialog.Content>
</Dialog.Root>
