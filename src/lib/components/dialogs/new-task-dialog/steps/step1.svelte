<script>
  import * as Form from "$lib/components/ui/form";
  import * as Select from "$lib/components/ui/select";

  export let formData;
  export let form;
  export let disabled;

  $: selectedPage = $formData.page ? {
    label: $formData.page,
    value: $formData.page
  } : undefined;
</script>

<Form.Field name="page" class="space-y-1" {form}>
  <Form.Control let:attrs>
    <Form.Label>Page</Form.Label>
    <Select.Root
      selected={selectedPage}
      onSelectedChange={v => {
        v && ($formData.page = v.value);
      }}
      {disabled}
    >
      <Select.Trigger {...attrs}>
        <Select.Value placeholder="Select a page" />
      </Select.Trigger>
      <Select.Content>
        <Select.Item value="eventim.de" label="eventim.de" />
        <Select.Item value="uk-umg.com" label="uk-umg.com" />
        <Select.Item value="signup.umusic.com" label="signup.umusic.com" />
      </Select.Content>
    </Select.Root>
    <input hidden bind:value={$formData.page} name={attrs.name} />
  </Form.Control>
  <Form.FieldErrors />
</Form.Field>
