<script>
  import { getContext } from "svelte";
  const { currentStep, steps } = getContext("form");
</script>

<div class="flex items-center gap-x-1 w-full">
  {#each { length: steps.length } as _, index}
    <div class="{index === currentStep ? 'bg-primary' : 'bg-muted'} w-2 h-2 rounded-full" />
  {/each}
</div>
