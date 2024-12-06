<script lang="ts">
	import type { SuperForm } from "sveltekit-superforms";
	import type { SettingsSchema } from "$lib/schemas/settings";
	import { Input } from "$lib/components/ui/input";
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

	let {
		form,
		formData
	}: Props = $props();

	const captchaSolvers = [
		{ value: "cap-solver", label: "CapSolver" },
	];

	const smsVerifiers = [
		{ value: "sms-activate", label: "SmsActivate" },
	];

	const selectedCaptcha = $derived(
			$formData.integration.captcha_solver
      ? captchaSolvers.find((captcha: SelectType) => captcha.value === $formData.integration.captcha_solver)?.label
      : "Select a captcha solver"
  );

	const selectedSmsVerifier = $derived(
			$formData.integration.sms_verifier
      ? smsVerifiers.find((sms_verifier: SelectType) => sms_verifier.value === $formData.integration.sms_verifier)?.label
      : "Select an SMS verifier"
  );
</script>

<div class="flex flex-col gap-y-6 h-full">
	<Form.Field name="integration.captcha_solver" {form}>
		<Form.Control>
			{#snippet children({ props })}
				<div class="flex w-full flex-col gap-1.5">
					<Form.Label>Captcha Solver</Form.Label>
					<Select.Root type="single" bind:value={$formData.integration.captcha_solver}>
						<Select.Trigger {...props}>{selectedCaptcha}</Select.Trigger>
						<Select.Content>
							{#each captchaSolvers as captchaSolver}
								<Select.Item value={captchaSolver.value}>{captchaSolver.label}</Select.Item>
							{/each}
						</Select.Content>
					</Select.Root>
					<p class="text-muted-foreground text-sm">Required for solving captchas on supported websites.</p>
				</div>
			{/snippet}
		</Form.Control>
		<Form.FieldErrors />
	</Form.Field>
	{#if !!$formData.integration.captcha_solver}
		<Form.Field name="integration.captcha_solver_api_key" {form}>
			<Form.Control>
				{#snippet children({ props })}
					<div class="flex w-full flex-col gap-1.5">
						<Form.Label>Captcha API Key</Form.Label>
						<Input {...props} bind:value={$formData.integration.captcha_solver_api_key} />
					</div>
				{/snippet}
			</Form.Control>
			<Form.FieldErrors />
		</Form.Field>
	{/if}
	<Separator />
	<Form.Field name="integration.sms_verifier" {form}>
		<Form.Control>
			{#snippet children({ props })}
				<div class="flex w-full flex-col gap-1.5">
					<Form.Label>SMS Verifier</Form.Label>
					<Select.Root type="single" bind:value={$formData.integration.sms_verifier}>
						<Select.Trigger {...props}>{selectedSmsVerifier}</Select.Trigger>
						<Select.Content>
							{#each smsVerifiers as smsVerifier}
								<Select.Item value={smsVerifier.value}>{smsVerifier.label}</Select.Item>
							{/each}
						</Select.Content>
					</Select.Root>
					<p class="text-muted-foreground text-sm">Required for solving captchas on supported websites.</p>
				</div>
			{/snippet}
		</Form.Control>
		<Form.FieldErrors />
	</Form.Field>
	{#if !!$formData.integration.sms_verifier}
		<Form.Field name="integration.sms_verifier_api_key" {form}>
			<Form.Control>
				{#snippet children({ props })}
					<div class="flex w-full flex-col gap-1.5">
						<Form.Label>SMS Verifier API Key</Form.Label>
						<Input {...props} bind:value={$formData.integration.sms_verifier_api_key} />
					</div>
				{/snippet}
			</Form.Control>
			<Form.FieldErrors />
		</Form.Field>
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
	<Form.Field name="integration.max_request_retries" {form}>
		<Form.Control>
			{#snippet children({ props })}
				<div class="flex w-full flex-col gap-1.5">
					<Form.Label>Max Request Retries</Form.Label>
					<Input {...props} placeholder="3" bind:value={$formData.integration.max_request_retries} />
					<p class="text-muted-foreground text-sm">Set the maximum number of retry attempts for failed requests before skipping to the next task.</p>
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
	<Form.Field name="integration.imap_email" {form}>
		<Form.Control>
			{#snippet children({ props })}
				<div class="flex w-full flex-col gap-1.5">
					<Form.Label>IMAP Email</Form.Label>
					<Input {...props} type="email" bind:value={$formData.integration.imap_email} />
				</div>
			{/snippet}
		</Form.Control>
		<Form.FieldErrors />
	</Form.Field>
	<Form.Field name="integration.imap_password" {form}>
		<Form.Control>
			{#snippet children({ props })}
				<div class="flex w-full flex-col gap-1.5">
					<Form.Label>IMAP Password</Form.Label>
					<Input {...props} type="password" bind:value={$formData.integration.imap_password} />
				</div>
			{/snippet}
		</Form.Control>
		<Form.FieldErrors />
	</Form.Field>
	<Separator />
	<Form.Field name="integration.webhook" {form}>
		<Form.Control>
			{#snippet children({ props })}
				<div class="flex w-full flex-col gap-1.5">
					<Form.Label>Webhook</Form.Label>
					<Input {...props} bind:value={$formData.integration.webhook} />
					<p class="text-muted-foreground text-sm">Configure a URL for handling webhook events.</p>
				</div>
			{/snippet}
		</Form.Control>
		<Form.FieldErrors />
	</Form.Field>
</div>
