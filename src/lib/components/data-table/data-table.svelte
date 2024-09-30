<script>
  import { readable } from "svelte/store";
  import {
    createTable,
    Render,
    Subscribe,
    createRender,
  } from "svelte-headless-table";
  import { addSelectedRows } from "svelte-headless-table/plugins";
  import { Badge } from "$lib/components/ui/badge";
  import * as Table from "$lib/components/ui/table";
  import TableActions from "./data-table-actions.svelte";
  import TableCheckbox from "./data-table-checkbox.svelte";

  const data = [
    {
      id: "m5gr84i9",
      url: "/event/bob-dylan-rough-and-rowdy-ways-messehalle-erfurt-18219523",
      page: "eventim.de",
      type: "signup",
      status: "stopped",
      proxies: "proxies1",
      tasks: 100,
    },
    {
      id: "m5gr84i9",
      url: "/event/bob-dylan-rough-and-rowdy-ways-messehalle-erfurt-18219523",
      page: "eventim.de",
      type: "signup",
      status: "running",
      proxies: "proxies2",
      tasks: 50,
    },
    {
      id: "m5gr84i9",
      url: "/event/bob-dylan-rough-and-rowdy-ways-messehalle-erfurt-18219523",
      page: "eventim.de",
      type: "signup",
      status: "stopped",
      proxies: "proxies3",
      tasks: 75,
    },
  ];

  const table = createTable(readable(data), {
    select: addSelectedRows(),
  });

  const columns = table.createColumns([
    table.column({
      accessor: "id",
      header: (_, { pluginStates}) => {
        const { allPageRowsSelected } = pluginStates.select;
        return createRender(TableCheckbox, {
          checked: allPageRowsSelected
        });
      },
      cell: ({ row }, { pluginStates }) => {
        const { getRowState } = pluginStates.select;
        const { isSelected } = getRowState(row);

        return createRender(TableCheckbox, {
          checked: isSelected,
        });
      },
    }),
    table.column({
      accessor: "status",
      header: "Status",
    }),
    table.column({
      accessor: "page",
      header: "Page",
    }),
    table.column({
      accessor: "url",
      header: "URL",
    }),
    table.column({
      accessor: "type",
      header: "Type",
    }),
    table.column({
      accessor: "proxies",
      header: "Proxies",
    }),
    table.column({
      accessor: "tasks",
      header: "Tasks",
    }),
    table.column({
      accessor: ({ id }) => id,
      header: "",
      cell: ({ value }) => {
        return createRender(TableActions, { id: value });
      }
    }),
  ]);

  const {
    headerRows,
    pageRows,
    tableAttrs,
    tableBodyAttrs,
    pluginStates,
  } = table.createViewModel(columns);
</script>

<div class="rounded-md border bg-background">
  <Table.Root {...$tableAttrs}>
    <Table.Header>
      {#each $headerRows as headerRow}
        <Subscribe rowAttrs={headerRow.attrs()}>
          <Table.Row>
            {#each headerRow.cells as cell (cell.id)}
              <Subscribe attrs={cell.attrs()} let:attrs props={cell.props()}>
                <Table.Head {...attrs}>
                  <Render of={cell.render()} />
                </Table.Head>
              </Subscribe>
            {/each}
          </Table.Row>
        </Subscribe>
      {/each}
    </Table.Header>

    <Table.Body {...$tableBodyAttrs}>
      {#each $pageRows as row (row.id)}
        <Subscribe rowAttrs={row.attrs()} let:rowAttrs>
          <Table.Row {...rowAttrs}>
            {#each row.cells as cell (cell.id)}
              <Subscribe attrs={cell.attrs()} let:attrs>
                <Table.Cell {...attrs}>
                  {#if cell.id === ""}
                    <div class="flex justify-end">
                      <Render of={cell.render()} />
                    </div>
                  {:else if cell.id === "status" && cell.value === "running"}
                    <div class="capitalize text-green-500">
                      <Render of={cell.render()} />
                    </div>
                  {:else if cell.id === "type"}
                    <div class="capitalize">
                      <Badge variant="secondary">{cell.value}</Badge>
                    </div>
                  {:else if cell.id === "status"}
                    <div class="bg-muted w-12 h-2 rounded-full" />
                  {:else}
                    <Render of={cell.render()} />
                  {/if}
                </Table.Cell>
              </Subscribe>
            {/each}
          </Table.Row>
        </Subscribe>
      {/each}
    </Table.Body>
  </Table.Root>
</div>
