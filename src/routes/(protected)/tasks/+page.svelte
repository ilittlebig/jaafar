<script>
  import { Button } from "$lib/components/ui/button";
  import { NewTaskDialog } from "$lib/components/dialogs/new-task-dialog";
  import { DataTable } from "$lib/components/data-table";
</script>

<div class="flex justify-between w-full">
  <h1 class="font-semibold text-2xl">Tasks</h1>
  <NewTaskDialog />
</div>
<DataTable />
