<script lang="ts">
	import { onDestroy, type Snippet } from "svelte";
	import { Button } from "$lib/components/ui/button";

	let countdown = $state(0);
	let countdownInterval: ReturnType<typeof setInterval>;

	interface Props {
		initialDelay: number;
		callback: () => void;
		children: Snippet;
	}

	let {
		initialDelay,
		callback,
		children,
	}: Props = $props();

	const handleClick = () => {
		callback();
		startCountdown();
	}

	const startCountdown = async () => {
		countdown = initialDelay;
    countdownInterval = setInterval(() => {
      countdown -= 1;
      if (countdown <= 0) clearInterval(countdownInterval);
    }, 1000);
	}

	onDestroy(() => {
		clearInterval(countdownInterval);
	});
</script>

<Button
	disabled={countdown > 0}
	onclick={handleClick}
	variant="link"
	class="text-sm h-4 py-0 px-0 font-normal leading-none"
>
	{#if countdown > 0}
		{countdown}
	{:else}
		{@render children()}
	{/if}
</Button>
