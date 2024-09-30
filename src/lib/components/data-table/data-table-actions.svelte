<script>
  import { z } from "zod";
  import { superForm } from "sveltekit-superforms";
  import { zodClient } from "sveltekit-superforms/adapters";
  import TrashCan from "lucide-svelte/icons/trash-2";
  import SquarePen from "lucide-svelte/icons/square-pen";
  import Play from "lucide-svelte/icons/play";
  import Clock from "lucide-svelte/icons/clock";
  import { Button } from "$lib/components/ui/button";
  import { Input } from "$lib/components/ui/input";
  import { Separator } from "$lib/components/ui/separator";
  import * as Form from "$lib/components/ui/form";
  import * as Dialog from "$lib/components/ui/dialog";
  import * as AlertDialog from "$lib/components/ui/alert-dialog";
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

  const formData = form.form;
  const enhance = form.enhance;
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

<Dialog.Root>
  <Dialog.Trigger asChild let:builder>
    <Button
      variant="ghost"
      builders={[builder]}
      size="icon"
      class="relative h-8 w-8 p-0"
    >
      <span class="sr-only">Open menu</span>
      <Play class="h-4 w-4" />
    </Button>
  </Dialog.Trigger>

  <Dialog.Content class="max-w-[525px]">
    <Dialog.Header>
      <Dialog.Title>
        Manage Task
      </Dialog.Title>
      <Dialog.Description>
        Select a task, and enter an URL for that task to run.
      </Dialog.Description>
    </Dialog.Header>

    <div class="grid grid-cols-4">
      <div class="flex flex-col">
        <h3 class="text-lg font-semibold">100</h3>
        <p class="text-sm">Tasks</p>
      </div>
      <div class="flex flex-col">
        <h3 class="text-lg font-semibold">0</h3>
        <p class="text-sm">Attempts</p>
      </div>
      <div class="flex flex-col">
        <h3 class="text-lg font-semibold">0</h3>
        <p class="text-sm">Carts</p>
      </div>
      <div class="flex flex-col">
        <h3 class="text-lg font-semibold">0</h3>
        <p class="text-sm">Errors</p>
      </div>
    </div>

    <div class="flex flex-col gap-y-2 rounded-md bg-muted h-96 w-full p-4 overflow-y-scroll">
      {#each { length: 15 } as _}
        <div class="flex gap-x-2">
          <p class="text-sm opacity-60">2024-01-01 04:03</p>
          <p class="text-sm">Waiting for restock</p>
        </div>
      {/each}
    </div>

    <Dialog.Footer>
      <Button variant="outline">
        <Clock class="mr-2 h-4 w-4" />
        Schedule Start
      </Button>
      <Button>
        <Play class="mr-2 h-4 w-4" />
        Start Task
      </Button>
    </Dialog.Footer>
  </Dialog.Content>
</Dialog.Root>

<AlertDialog.Root>
  <AlertDialog.Trigger asChild let:builder>
    <Button
      variant="ghost"
      builders={[builder]}
      size="icon"
      class="relative h-8 w-8 p-0"
    >
      <span class="sr-only">Open menu</span>
      <TrashCan class="h-4 w-4" />
    </Button>
  </AlertDialog.Trigger>

  <AlertDialog.Content>
    <AlertDialog.Header>
      <AlertDialog.Title>Are you absolutely sure?</AlertDialog.Title>
      <AlertDialog.Description>
        This action cannot be undone. This will permanently delete your account
        and remove your data from our servers.
      </AlertDialog.Description>
    </AlertDialog.Header>
    <AlertDialog.Footer>
      <AlertDialog.Cancel>Cancel</AlertDialog.Cancel>
      <AlertDialog.Action>Continue</AlertDialog.Action>
    </AlertDialog.Footer>
  </AlertDialog.Content>
</AlertDialog.Root>
