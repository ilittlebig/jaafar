<script>
  import * as Form from "$lib/components/ui/form";
  import { Input } from "$lib/components/ui/input";

  export let disabled;
  export let form;
  export let formData;
</script>

<Form.Field name="tasks" class="space-y-1" {form}>
  <Form.Control let:attrs>
    <Form.Label>Tasks</Form.Label>
    <Input
      id="tasks"
      placeholder="Enter an amount"
      bind:value={$formData.tasks}
      {...attrs}
      {disabled}
    />
  </Form.Control>
  <Form.Description>Must be a value between 1-250.</Form.Description>
  <Form.FieldErrors />
</Form.Field>
