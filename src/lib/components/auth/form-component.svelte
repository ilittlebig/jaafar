<script lang="ts">
	import type { Component, Snippet } from "svelte";
	import type { z } from "zod";
	import {
		superForm,
		setError,
		type SuperValidated,
		type Infer,
	} from "sveltekit-superforms";
	import { zodClient } from "sveltekit-superforms/adapters";
	import { selectFile } from "$lib/services/files-service";
	import { Input } from "$lib/components/ui/input";
	import { Button } from "$lib/components/ui/button";
	import * as Form from "$lib/components/ui/form";
	import * as Select from "$lib/components/ui/select";

	// Helper function for determining of a select value is a group
	function isValueGroup(value: ValueObj | ValueGroup): value is ValueGroup {
		return (value as ValueGroup).group !== undefined;
	}

	type ValueObj = {
		value: string;
		label: string;
	}

	type ValueGroup = {
		group: string;
		options: ValueObj[];
	};

	interface Field {
		name: string;
		label: string;
		placeholder?: string;
		description?: string;
		extensions?: string[];
		type?: string;
		component?: Component;
		values?: (ValueObj | ValueGroup)[];
		allowDeselect?: boolean;
	}

	interface Props {
		onsubmit: (formData: any) => Promise<void>;
		onsuccess?: () => void;
		fields: Field[];
		schema: z.ZodTypeAny;
		data: Record<string, string | undefined>;
		children: Snippet<[boolean]>;
	}

	let {
		onsubmit,
		onsuccess,
		fields,
		schema,
		data,
		children,
	}: Props = $props();

	const handleSelectFile = async (field: Field) => {
		const filePath = await selectFile({
			multiple: false,
			directory: false,
			filters: field.extensions ? [{ name: "extension", extensions: field.extensions }] : [],
		});
		if (!filePath) return;
		$formData[field.name] = filePath;
	}

  const handleValidForm = async (form: SuperValidated<Infer<typeof schema>>) => {
    const formData = form.data;
    try {
      await onsubmit(formData);
			onsuccess?.();
    } catch (err: any) {
      setError(form, fields[0].name, err.message);
    }
  };

	const form = superForm(data, {
    SPA: true,
    resetForm: false,
    clearOnSubmit: "errors",
    validators: zodClient(schema),
    async onUpdate({ form }) {
      if (form.valid) await handleValidForm(form);
    }
  });

  const { form: formData, enhance, submitting } = form;
</script>

<form class="grid gap-4" method="POST" use:enhance>
	{#each fields as field}
		<Form.Field name={field.name} {form}>
			<Form.Control>
				{#snippet children({ props })}
					{#if field.component}
						<div class="flex items-center">
							<Form.Label>{field.label}</Form.Label>
							<field.component />
						</div>
					{:else}
						<Form.Label>{field.label}</Form.Label>
					{/if}

					{#if field.type === "file"}
						<Button variant="outline" class="w-full" onclick={() => handleSelectFile(field)}>
							{$formData[field.name] || "Select File"}
						</Button>
					{:else if field.type === "select" && field.values}
						<Select.Root type="single" bind:value={$formData[field.name]} allowDeselect={field.allowDeselect}>
							<Select.Trigger {...props}>
								{field.values
									.flatMap(obj => isValueGroup(obj) ? obj.options : [obj])
									.find((obj: ValueObj) => obj.value === $formData[field.name])?.label ||
									field.placeholder || "Select"}
							</Select.Trigger>
							<Select.Content>
								{#each field.values as obj}
									{#if isValueGroup(obj)}
										<Select.Group>
											<Select.GroupHeading>{obj.group}</Select.GroupHeading>
											{#each obj.options as option}
												<Select.Item value={option.value} class="pl-12">
													{option.label}
												</Select.Item>
											{/each}
										</Select.Group>
									{:else}
										<Select.Item value={obj.value}>{obj.label}</Select.Item>
									{/if}
								{/each}
							</Select.Content>
						</Select.Root>
					{:else}
						<Input
							{...props}
							type={field.type}
							placeholder={field.placeholder}
							bind:value={$formData[field.name]}
						/>
					{/if}
				{/snippet}
			</Form.Control>
			{#if field.description}
				<Form.Description>{field.description}</Form.Description>
			{/if}
			<Form.FieldErrors />
		</Form.Field>
	{/each}
	{@render children?.($submitting)}
</form>
