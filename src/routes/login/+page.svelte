<script>
  import { Button } from "$lib/components/ui/button";
  import { Input } from "$lib/components/ui/input";
  import { Label } from "$lib/components/ui/label";
  import LightSwitch from "$lib/components/light-switch.svelte";
</script>

<div class="flex w-full h-screen items-center justify-center">
  <div class="absolute top-0 right-0 pr-3 pt-3 z-20">
    <LightSwitch />
  </div>

  <div class="flex items-center justify-center py-12">
    <div class="mx-auto grid w-[350px] gap-6">
      <div class="grid gap-2 text-center">
        <h1 class="text-3xl font-bold">Login</h1>
        <p class="text-muted-foreground text-balance">
          Enter your email below to login to your account
        </p>
      </div>

      <div class="grid gap-4">
        <div class="grid gap-2">
          <Label for="email">Email</Label>
          <Input id="email" type="email" placeholder="user@example.com" required />
        </div>
        <div class="grid gap-2">
          <div class="flex items-center">
            <Label for="password">Password</Label>
            <a href="##" class="ml-auto inline-block text-sm underline">
              Forgot your password?
            </a>
          </div>
          <Input id="password" type="password" required />
        </div>
        <Button type="submit" class="w-full">Login</Button>
      </div>

      <div class="mt-4 text-center text-sm">
        Don't have an account?
        <a href="##" class="underline"> Sign up </a>
      </div>
    </div>
  </div>
</div>
