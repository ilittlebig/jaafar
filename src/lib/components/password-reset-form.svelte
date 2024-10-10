<script>
  import { Button } from "$lib/components/ui/button";
  import { Input } from "$lib/components/ui/input";
  import * as Dialog from "$lib/components/ui/dialog";
  import * as Form from "$lib/components/ui/form";

  export let codeRequired = true;
  export let enhance;
  export let form;
  export let formData;
  export let submitting;
  export let open;
</script>

<form class="grid gap-4" method="POST" use:enhance>
  {#if codeRequired}
    <Form.Field name="confirmationCode" class="space-y-1" {form}>
      <Form.Control let:attrs>
        <Form.Label>Confirmation Code</Form.Label>
        <Input
          id="confirmation-code"
          bind:value={$formData.confirmationCode}
          {...attrs}
        />
      </Form.Control>
      <Form.FieldErrors />
    </Form.Field>
  {/if}

  <Form.Field name="newPassword" class="space-y-1" {form}>
    <Form.Control let:attrs>
      <Form.Label>New Password</Form.Label>
      <Input
        id="new-password"
        type="password"
        bind:value={$formData.newPassword}
        {...attrs}
      />
    </Form.Control>
    <Form.FieldErrors />
  </Form.Field>

  <Form.Field name="confirmPassword" class="space-y-1" {form}>
    <Form.Control let:attrs>
      <Form.Label>Confirm Password</Form.Label>
      <Input
        id="confirm-password"
        type="password"
        bind:value={$formData.confirmPassword}
        {...attrs}
      />
    </Form.Control>
    <Form.FieldErrors />
  </Form.Field>

  <Dialog.Footer>
    <Button variant="outline" on:click={() => $open = false}>
      Cancel
    </Button>
    <Button type="submit" disabled={$submitting}>
      Change Password
    </Button>
  </Dialog.Footer>
</form>
