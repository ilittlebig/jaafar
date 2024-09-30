<script>
  import { getContext } from "svelte";
  import { Button } from "$lib/components/ui/button";

  export let lastLabel = "Finish";

  const { currentStep, steps } = getContext("form");
</script>

<Button type="submit">
  {#if currentStep < steps.length - 1}
    Continue
  {:else}
    {lastLabel}
  {/if}
</Button>
