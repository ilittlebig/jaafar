<script lang="ts">
	import { Input } from "$lib/components/ui/input";
  import { Label } from "$lib/components/ui/label";
  import { Separator } from "$lib/components/ui/separator";
	import * as Select from "$lib/components/ui/select";

	interface SelectType {
		value: string;
		label: string;
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
	<div class="flex w-full flex-col gap-1.5">
		<Label for="request-delay">Request Delay</Label>
		<Input id="request-delay" placeholder="3000" />
		<p class="text-muted-foreground text-sm">Configure the delay (in milliseconds) between automated actions.</p>
	</div>
	<div class="flex w-full flex-col gap-1.5">
		<Label for="entry-limit">Entry Limit</Label>
		<Input id="entry-limit" placeholder="10" />
		<p class="text-muted-foreground text-sm">Sets the maximum number of entries/tasks to process per request.</p>
	</div>
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
