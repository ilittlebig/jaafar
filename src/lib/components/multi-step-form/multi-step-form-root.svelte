<script>
  import { setContext } from "svelte";
  import { superForm } from "sveltekit-superforms";
  import { zodClient } from "sveltekit-superforms/adapters";

  export let steps;
  export let data;

  let form, formData, enhance;
  let currentStep = 0;

  const nextPage = () => {
    if (currentStep < steps.length - 1) {
      currentStep += 1;
    } else {
    }
  }

  const previousPage = () => {
    if (currentStep <= 0) return;
    currentStep -= 1;
  }

  const onUpdated = ({ form: f }) => {
    if (!f.valid) return;
    $data = $formData;
    nextPage();
  };

  const createForm = step => {
    const schema = steps[step].schema;
    const newForm = superForm($data, {
      SPA: true,
      resetForm: false,
      validators: zodClient(schema),
      onUpdated,
    });
    return newForm;
  };

  $: {
    form = createForm(currentStep);
    formData = form.form;
    enhance = form.enhance;

    setContext("form", {
      form,
      formData,
      enhance,
      currentStep,
      previousPage,
      steps,
    });
  }
</script>

{#key form}
  <form class="grid gap-4" method="POST" use:enhance>
    <slot />
  </form>
{/key}
