<script lang="ts">
  import { Button } from "$lib/components/ui/button";
  import * as DropdownMenu from "$lib/components/ui/dropdown-menu";
	import { signOutDialog } from "$lib/components/dialogs/auth/sign-out-dialog.svelte";
	import { settingsDialog } from "$lib/components/dialogs/settings-dialog.svelte";
</script>

<DropdownMenu.Root>
  <DropdownMenu.Trigger>
		{#snippet child({ props })}
			<Button
				{...props}
				variant="ghost"
				size="icon"
				class="rounded-lg "
			>
				<i class="fa-regular fa-circle-user text-lg"></i>
				<span class="sr-only">Toggle user menu</span>
			</Button>
		{/snippet}
  </DropdownMenu.Trigger>

  <DropdownMenu.Content class="w-56" align="end" side="right">
    <DropdownMenu.Label class="font-normal">
      <div class="flex flex-col space-y-1">
        <p class="text-sm font-medium leading-none">Elias</p>
        <p class="text-muted-foreground text-xs leading-none">elias@example.com</p>
      </div>
    </DropdownMenu.Label>
    <DropdownMenu.Separator />
    <DropdownMenu.Item onSelect={() => settingsDialog.open = true}>
      Settings
      <DropdownMenu.Shortcut>⌘,</DropdownMenu.Shortcut>
    </DropdownMenu.Item>
    <DropdownMenu.Separator />
		<DropdownMenu.Item onSelect={() => signOutDialog.open = true}>
			Sign Out
		</DropdownMenu.Item>
  </DropdownMenu.Content>
</DropdownMenu.Root>
