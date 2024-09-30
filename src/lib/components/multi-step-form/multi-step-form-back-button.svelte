<script>
  import { getContext } from "svelte";
  import { Button } from "$lib/components/ui/button";
  const { currentStep, previousPage } = getContext("form");
</script>

<Button variant="outline" class="mr-2" disabled={currentStep <= 0} on:click={previousPage}>
  Back
</Button>
