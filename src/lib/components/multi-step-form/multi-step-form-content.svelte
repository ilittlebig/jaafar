<script>
  import { getContext } from "svelte";
  const { form, formData, currentStep, steps } = getContext("form");
</script>

<div class="grid gap-2">
  {#each steps as step, index}
    {#if index <= currentStep}
      <svelte:component
        this={step.component}
        {formData}
        {form}
        disabled={index < currentStep}
      />
    {/if}
  {/each}
</div>
