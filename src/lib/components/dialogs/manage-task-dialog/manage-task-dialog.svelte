<script>
  import Play from "lucide-svelte/icons/play";
  import Clock from "lucide-svelte/icons/clock";
  import { Button } from "$lib/components/ui/button";
  import * as Dialog from "$lib/components/ui/dialog";
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
