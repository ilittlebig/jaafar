<script>
  import NavbarButton from "./navbar-button.svelte";

  import House from "lucide-svelte/icons/house";
  import LayoutList from "lucide-svelte/icons/layout-list";
  import GlobeLock from "lucide-svelte/icons/globe-lock";
  import SquareUserRound from "lucide-svelte/icons/user";

  const navigations = [
    { label: "Dashboard", icon: House, href: "/" },
    { label: "Tasks", icon: LayoutList, href: "/tasks" },
    { label: "Proxies", icon: GlobeLock, href: "/proxies" },
    { label: "Accounts", icon: SquareUserRound, href: "/accounts" },
  ];
</script>

<nav class="grid gap-1">
  {#each navigations as navigation}
    <NavbarButton {navigation} />
  {/each}
</nav>
