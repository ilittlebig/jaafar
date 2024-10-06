<script>
  import { Button } from "$lib/components/ui/button";
  import * as Dialog from "$lib/components/ui/dialog";
</script>

<Dialog.Root>
  <Dialog.Trigger asChild let:builder>
    <Button
      variant="link"
      builders={[builder]}
      class="text-sm font-normal hover:underline p-0 h-auto"
    >
      Sign up
    </Button>
  </Dialog.Trigger>

  <Dialog.Content class="max-w-[425px]">
    <Dialog.Header>
      <Dialog.Title>
        Sign Up
      </Dialog.Title>
      <Dialog.Description>
        Select a task, and enter an URL for that task to run.
      </Dialog.Description>
    </Dialog.Header>

    <Dialog.Footer>
      <Button variant="outline">
        Cancel
      </Button>
      <Button>
        Sign Up
      </Button>
    </Dialog.Footer>
  </Dialog.Content>
</Dialog.Root>
