<script lang="ts">
	import type { SuperForm } from "sveltekit-superforms";
	import type { SettingsSchema } from "$lib/schemas/settings";
	import { Input } from "$lib/components/ui/input";
  import { Label } from "$lib/components/ui/label";
  import { Separator } from "$lib/components/ui/separator";
	import * as Select from "$lib/components/ui/select";
	import * as Form from "$lib/components/ui/form";

	interface SelectType {
		value: string;
		label: string;
	}

	interface Props {
		form: SuperForm<SettingsSchema>;
		formData: SvelteStore<SettingsSchema>;
	}

	const captchaSolvers = [
		{ value: "cap-solver", label: "CapSolver" },
		{ value: "anti-captcha", label: "Anti-Captcha" },
		{ value: "2-captcha", label: "2Captcha" },
		{ value: "death-by-captcha", label: "Death by Captcha" },
		{ value: "zennolab-captcha", label: "ZennoLab Captcha" },
		{ value: "image-typerz", label: "ImageTyperz" },
		{ value: "az-captcha", label: "AZcaptcha" },
		{ value: "best-captcha-solver", label: "Best Captcha Solver" },
		{ value: "resolve-captcha", label: "ResolveCaptcha" },
		{ value: "captcha-solutions", label: "CaptchaSolutions" },
	];

  let captchaValue = $state<string>("");
	const selectedCaptcha = $derived(
    captchaValue
      ? captchaSolvers.find((captcha: SelectType) => captcha.value === captchaValue)?.label
      : "Select a captcha solver"
  );

	let { form, formData }: Props = $props();
</script>

<div class="flex flex-col gap-y-6 h-full">
	<div class="flex w-full flex-col gap-1.5">
		<Label>Captcha Solver</Label>
		<Select.Root type="single" onValueChange={v => (captchaValue = v)}>
			<Select.Trigger>{selectedCaptcha}</Select.Trigger>
			<Select.Content>
				{#each captchaSolvers as captchaSolver}
					<Select.Item value={captchaSolver.value}>{captchaSolver.label}</Select.Item>
				{/each}
			</Select.Content>
		</Select.Root>
		<p class="text-muted-foreground text-sm">Required for solving captchas on supported websites.</p>
	</div>
	{#if !!captchaValue}
		<div class="flex w-full flex-col gap-1.5">
			<Label for="captcha-api-key">Captcha API Key</Label>
			<Input id="captcha-api-key" />
		</div>
	{/if}
	<Separator />
	<Form.Field name="integration.request_delay" {form}>
		<Form.Control>
			{#snippet children({ props })}
				<div class="flex w-full flex-col gap-1.5">
					<Form.Label>Request Delay</Form.Label>
					<Input {...props} placeholder="3000" bind:value={$formData.integration.request_delay} />
					<p class="text-muted-foreground text-sm">Configure the delay (in milliseconds) between automated actions.</p>
				</div>
			{/snippet}
		</Form.Control>
		<Form.FieldErrors />
	</Form.Field>
	<Form.Field name="integration.entry_limit" {form}>
		<Form.Control>
			{#snippet children({ props })}
				<div class="flex w-full flex-col gap-1.5">
					<Form.Label>Entry Limit</Form.Label>
					<Input {...props} placeholder="10" bind:value={$formData.integration.entry_limit} />
					<p class="text-muted-foreground text-sm">Sets the maximum number of entries/tasks to process per request.</p>
				</div>
			{/snippet}
		</Form.Control>
		<Form.FieldErrors />
	</Form.Field>
	<Separator />
	<div class="flex w-full flex-col gap-1.5">
		<Label for="imap-email">IMAP Email</Label>
		<Input type="email" id="imap-email" />
	</div>
	<div class="flex w-full flex-col gap-1.5">
		<Label for="imap-password">IMAP Password</Label>
		<Input type="password" id="imap-password" />
	</div>
	<Separator />
	<div class="flex w-full flex-col gap-1.5">
		<Label for="webhook">Webhook</Label>
		<Input id="webhook" />
		<p class="text-muted-foreground text-sm">Configure a URL for handling webhook events.</p>
	</div>
</div>
