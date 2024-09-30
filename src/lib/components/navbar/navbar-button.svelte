<script>
  import { page } from "$app/stores";
  import { Button } from "$lib/components/ui/button";
  import * as Tooltip from "$lib/components/ui/tooltip";

  export let navigation;

  $: isActive = $page.url.pathname === navigation.href;
</script>

<Tooltip.Root openDelay={0}>
  <Tooltip.Trigger asChild let:builder>
    <Button
      variant="ghost"
      size="icon"
      class="{isActive ? "bg-muted" : ""} rounded-lg"
      href={navigation.href}
      aria-label={navigation.label}
      builders={[builder]}
    >
      <svelte:component
        this={navigation.icon}
        class="size-5"
        fill={isActive ? "black" : "transparent"}
      />
    </Button>
  </Tooltip.Trigger>

  <Tooltip.Content side="right" sideOffset={5}>
    {navigation.label}
  </Tooltip.Content>
</Tooltip.Root>
