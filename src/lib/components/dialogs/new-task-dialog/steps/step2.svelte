<script>
  import * as Form from "$lib/components/ui/form";
  import { Input } from "$lib/components/ui/input";

  export let disabled;
  export let form;
  export let formData;
</script>

<Form.Field name="url" class="space-y-1" {form}>
  <Form.Control let:attrs>
    <Form.Label>Url</Form.Label>
    <Input
      id="url"
      placeholder="Enter an URL"
      bind:value={$formData.url}
      {...attrs}
      {disabled}
    />
  </Form.Control>
  <Form.Description>Must match the page domain.</Form.Description>
  <Form.FieldErrors />
</Form.Field>
