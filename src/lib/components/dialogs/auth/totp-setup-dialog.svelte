<script module>
	export let totpSetupDialog = $state({ open: false });
</script>

<script lang="ts">
	import type { AuthTOTPSetupDetails } from "@ilittlebig/easy-auth";
	import QRCode from "qrcode";
	import { handleChallengeResponse } from "$lib/services/auth-service";
	import { totpSetupDetailsStore, usernameStore } from "$lib/stores/auth-store.svelte";
	import { totpCodeFormSchema } from "$lib/schemas/auth";
	import { buttonVariants } from "$lib/components/ui/button";
	import * as Dialog from "$lib/components/ui/dialog";
	import * as Form from "$lib/components/ui/form";
	import * as Tooltip from "$lib/components/ui/tooltip";
	import FormComponent from "$lib/components/auth/form-component.svelte";

  let qrCodeUrl = $state("");
  let secret = $state("");

	interface QRCodeProps {
		totpSetupDetails: AuthTOTPSetupDetails;
		username: string;
	}

  // Formats the secret into a 4-digit space-separated string
  const formatSecret = (secret: string) => secret.match(/.{1,4}/g)?.join(" ");

	const generateQRCode = async (uri: string): Promise<string> => {
		return QRCode.toDataURL(uri, {
			errorCorrectionLevel: "M",
			type: "image/png",
			margin: 4,
		});
	};

	const setupQRCode = async ({
		totpSetupDetails,
		username
	}: QRCodeProps): Promise<{ qrCodeUrl: string; sharedSecret: string }> => {
		try {
			const { sharedSecret, getSetupUri } = totpSetupDetails;
			const uri = getSetupUri("AM System - Studio", username);
			const qrCodeUrl = await generateQRCode(uri.toString());
			return { qrCodeUrl, sharedSecret };
		} catch (err) {
			console.error("Failed to generate QR code:", err);
			// TODO:
			throw err; // Re-throw the error for handling at the call site
		}
	};

	const fields = [
		{ name: "code", label: "Code" },
	];

	const data = { code: "" };

  $effect(() => {
    if (!totpSetupDetailsStore.value) return;
    setupQRCode({
      totpSetupDetails: totpSetupDetailsStore.value,
      username: usernameStore.value
		}).then(({ qrCodeUrl: url, sharedSecret }) => {
			[qrCodeUrl, secret] = [url, sharedSecret];
		});
  });
</script>

<Dialog.Root bind:open={totpSetupDialog.open}>
	<Dialog.Content class="max-w-[425px]">
		<Dialog.Header>
			<Dialog.Title>Setup MFA</Dialog.Title>
			<Dialog.Description>
        Scan the QR code with any authenticator app and enter the code below.
			</Dialog.Description>
		</Dialog.Header>

    <div class="flex justify-center">
      <img src={qrCodeUrl} width="228" height="228" alt="QR Code" />
    </div>

    <div class="flex justify-center">
      <Tooltip.Provider>
				<Tooltip.Root>
					<Tooltip.Trigger class={buttonVariants({ variant: "link" })}>
						Can't scan the QR code?
					</Tooltip.Trigger>
					<Tooltip.Content>
						<p>{formatSecret(secret)}</p>
					</Tooltip.Content>
				</Tooltip.Root>
		 </Tooltip.Provider>
    </div>

		<FormComponent
			{data} 
			{fields}
			onsubmit={handleChallengeResponse}
			onsuccess={() => totpSetupDialog.open = false}
			schema={totpCodeFormSchema}
		>
			{#snippet children(submitting)}
				<div class="flex justify-end">
					<Form.Button disabled={submitting}>Setup MFA</Form.Button>
				</div>
			{/snippet}
		</FormComponent>
	</Dialog.Content>
</Dialog.Root>
