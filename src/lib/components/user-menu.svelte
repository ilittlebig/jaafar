<script>
  import { Button } from "$lib/components/ui/button";
  import * as DropdownMenu from "$lib/components/ui/dropdown-menu";
  import CircleUser from "lucide-svelte/icons/circle-user";
</script>

<DropdownMenu.Root>
  <DropdownMenu.Trigger asChild let:builder>
    <Button
      variant="ghost"
      size="icon"
      class="rounded-lg"
      builders={[builder]}
    >
      <CircleUser class="h-5 w-5" />
      <span class="sr-only">Toggle user menu</span>
    </Button>
  </DropdownMenu.Trigger>

  <DropdownMenu.Content class="w-56" align="end">
    <DropdownMenu.Label class="font-normal">
      <div class="flex flex-col space-y-1">
        <p class="text-sm font-medium leading-none">Elias</p>
        <p class="text-muted-foreground text-xs leading-none">elias@example.com</p>
      </div>
    </DropdownMenu.Label>
    <DropdownMenu.Separator />
    <DropdownMenu.Item>Profile</DropdownMenu.Item>
    <DropdownMenu.Item>
      Settings
      <DropdownMenu.Shortcut>⌘,</DropdownMenu.Shortcut>
    </DropdownMenu.Item>
    <DropdownMenu.Item>Billing</DropdownMenu.Item>
    <DropdownMenu.Separator />
    <DropdownMenu.Item>Sign Out</DropdownMenu.Item>
  </DropdownMenu.Content>
</DropdownMenu.Root>
