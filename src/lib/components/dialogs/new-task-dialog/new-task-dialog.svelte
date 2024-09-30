<script>
  import { writable } from "svelte/store";
  import { z } from "zod";

  import { Button, buttonVariants } from "$lib/components/ui/button";
  import { MultiStepForm } from "$lib/components/multi-step-form";
  import * as Dialog from "$lib/components/ui/dialog";

  import Plus from "lucide-svelte/icons/plus";

  import Step1 from "./steps/step1.svelte";
  import Step2 from "./steps/step2.svelte";

  let open = writable(false);
  let data = { page: undefined, url: undefined };

  const steps = [
    {
      component: Step1,
      schema: z.object({
        page: z.string({ required_error: "Please select a page" })
      })
    },
    {
      component: Step2,
      schema: z.object({
        url: z
          .string({ required_error: "Please enter a valid URL" })
          .refine(value => /^(https?):\/\/(?=.*\.[a-z]{2,})[^\s$.?#].[^\s]*$/i.test(value), {
            message: "Please enter a valid URL",
          }),
      })
    },
  ];

  const foo = () => {
    $open = false;
  }
</script>

<Dialog.Root bind:open={$open}>
  <Dialog.Trigger class={buttonVariants({ variant: "default" })}>
    <Plus class="mr-2 h-4 w-4" />
    Add New Task
  </Dialog.Trigger>

  <Dialog.Content class="max-w-[425px]">
    <Dialog.Header>
      <Dialog.Title>
        Add New Task
      </Dialog.Title>
      <Dialog.Description>
        Select a page, and enter an URL for that task to run.
      </Dialog.Description>
    </Dialog.Header>

    <MultiStepForm.Root {steps} {data}>
      <MultiStepForm.Content />
      <Dialog.Footer>
        <MultiStepForm.BackButton />
        <MultiStepForm.StepIndicator />
        <MultiStepForm.SubmitButton lastLabel="Add Task" />
      </Dialog.Footer>
    </MultiStepForm.Root>
  </Dialog.Content>
</Dialog.Root>
