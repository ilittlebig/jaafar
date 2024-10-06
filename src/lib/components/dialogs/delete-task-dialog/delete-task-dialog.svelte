<script>
  import { Button } from "$lib/components/ui/button";
  import * as AlertDialog from "$lib/components/ui/alert-dialog";
  import TrashCan from "lucide-svelte/icons/trash-2";
</script>

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
      <AlertDialog.Title>Delete Task</AlertDialog.Title>
      <AlertDialog.Description>
        This action cannot be undone. This will permanently delete this task.
      </AlertDialog.Description>
    </AlertDialog.Header>
    <AlertDialog.Footer>
      <AlertDialog.Cancel>Cancel</AlertDialog.Cancel>
      <AlertDialog.Action>Delete</AlertDialog.Action>
    </AlertDialog.Footer>
  </AlertDialog.Content>
</AlertDialog.Root>
