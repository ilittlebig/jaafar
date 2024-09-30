<script>
  import { Button } from "$lib/components/ui/button";
  import Bell from "lucide-svelte/icons/bell";
  import CircleHelp from "lucide-svelte/icons/circle-help";
</script>

<header class="bg-background flex justify-end items-center gap-1 border-b h-[44px] px-2">
  <Button
    variant="ghost"
    size="icon"
    class="rounded-lg z-20"
    aria-label="Notifications"
  >
    <Bell class="h-5 w-5" />
  </Button>
  <Button
    variant="ghost"
    size="icon"
    class="rounded-lg z-20"
    aria-label="Help"
  >
    <CircleHelp class="h-5 w-5" />
  </Button>
</header>

