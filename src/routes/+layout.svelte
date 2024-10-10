<script>
  import { ModeWatcher } from "mode-watcher";
  import { Toaster } from "$lib/components/ui/sonner";
  import Titlebar from "$lib/components/titlebar.svelte";
</script>

<Titlebar />
<ModeWatcher />
<Toaster />

<slot />
