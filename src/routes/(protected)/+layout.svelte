<script>
  import { Navbar } from "$lib/components/navbar";
  import UserMenu from "$lib/components/user-menu.svelte";
  import Header from "$lib/components/header.svelte";
</script>

<div class="grid min-h-screen w-full grid-cols-[85px_1fr]">
  <div class="bg-background border-r h-full w-full">
    <div class="flex h-full max-h-screen flex-col justify-between items-center pb-4">
      <div class="flex w-full h-full flex-col items-center gap-4">
        <div class="flex items-center border-b w-full h-[44px] px-6" />
        <Navbar />
      </div>
      <UserMenu />
    </div>
  </div>
  <div class="flex flex-col">
    <Header />
    <main class="flex flex-1 flex-col gap-6 p-6 bg-muted/40">
      <slot />
    </main>
  </div>
</div>
