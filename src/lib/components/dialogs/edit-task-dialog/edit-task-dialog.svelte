<script>
  import { z } from "zod";
  import { superForm } from "sveltekit-superforms";
  import { zodClient } from "sveltekit-superforms/adapters";
  import SquarePen from "lucide-svelte/icons/square-pen";
  import { Button } from "$lib/components/ui/button";
  import { Input } from "$lib/components/ui/input";
  import * as Form from "$lib/components/ui/form";
  import * as Dialog from "$lib/components/ui/dialog";
  import * as Select from "$lib/components/ui/select";

  const data = { page: "eventim.de", url: "/event/blah-blah-blah-12233", amount: undefined };

  const schema = z.object({
    page: z.string({ required_error: "Please select a page" }),
    url: z.string({ required_error: "Please select a task" }),
  })

  const form = superForm(data, {
    SPA: true,
    resetForm: false,
    validators: zodClient(schema),
  });

  const { form: formData, enhance } = form;
</script>

<Dialog.Root>
  <Dialog.Trigger asChild let:builder>
    <Button
      variant="ghost"
      builders={[builder]}
      size="icon"
      class="relative h-8 w-8 p-0"
    >
      <span class="sr-only">Open menu</span>
      <SquarePen class="h-4 w-4" />
    </Button>
  </Dialog.Trigger>

  <Dialog.Content class="max-w-[425px]">
    <Dialog.Header>
      <Dialog.Title>
        Edit Task
      </Dialog.Title>
      <Dialog.Description>
        Select a task, and enter an URL for that task to run.
      </Dialog.Description>
    </Dialog.Header>

    <div class="grid gap-2">
      <Form.Field name="task" class="space-y-1" {form}>
        <Form.Control let:attrs>
          <Form.Label>Page</Form.Label>
          <Input
            id="page"
            disabled={true}
            bind:value={$formData.page}
            {...attrs}
          />
        </Form.Control>
        <Form.FieldErrors />
      </Form.Field>

      <Form.Field name="url" class="space-y-1" {form}>
        <Form.Control let:attrs>
          <Form.Label>Url</Form.Label>
          <Input
            id="url"
            disabled={true}
            bind:value={$formData.url}
            {...attrs}
          />
        </Form.Control>
        <Form.FieldErrors />
      </Form.Field>

      <Form.Field name="proxies" class="space-y-1" {form}>
        <Form.Control let:attrs>
          <Form.Label>Proxies</Form.Label>
          <Select.Root>
            <Select.Trigger {...attrs}>
              <Select.Value placeholder="Select proxies" />
            </Select.Trigger>
            <Select.Content>
              <Select.Item value="eventim.de" label="eventim.de" />
              <Select.Item value="uk-umg.com" label="uk-umg.com" />
              <Select.Item value="signup.umusic.com" label="signup.umusic.com" />
            </Select.Content>
          </Select.Root>
        </Form.Control>
        <Form.FieldErrors />
      </Form.Field>

      <Form.Field name="amount" class="space-y-1" {form}>
        <Form.Control let:attrs>
          <Form.Label>Amount</Form.Label>
          <Input
            id="amount"
            placeholder="Enter an amount"
            bind:value={$formData.amount}
            {...attrs}
          />
        </Form.Control>
        <Form.FieldErrors />
      </Form.Field>
    </div>

    <Dialog.Footer>
      <Button variant="outline">
        Cancel
      </Button>
      <Button type="submit">
        Save
      </Button>
    </Dialog.Footer>
  </Dialog.Content>
</Dialog.Root>
