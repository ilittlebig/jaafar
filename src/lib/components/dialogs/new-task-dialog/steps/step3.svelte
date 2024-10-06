<script>
  import * as Form from "$lib/components/ui/form";
  import * as Select from "$lib/components/ui/select";

  export let disabled;
  export let form;
  export let formData;

  $: selectedProxies = $formData.proxies ? {
    label: $formData.proxies,
    value: $formData.proxies
  } : undefined;
</script>

<Form.Field name="proxies" class="space-y-1" {form}>
  <Form.Control let:attrs>
    <Form.Label>Proxies</Form.Label>
    <Select.Root
      selected={selectedProxies}
      onSelectedChange={v => {
        v && ($formData.proxies = v.value);
      }}
      {disabled}
    >
      <Select.Trigger {...attrs}>
        <Select.Value placeholder="Select proxies" />
      </Select.Trigger>
      <Select.Content>
        <Select.Item value="proxies1" label="proxies1" />
        <Select.Item value="proxies2" label="proxies2" />
        <Select.Item value="proxies3" label="proxies3" />
      </Select.Content>
    </Select.Root>
  </Form.Control>
  <Form.FieldErrors />
</Form.Field>
