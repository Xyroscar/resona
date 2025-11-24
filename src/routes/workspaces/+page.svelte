<script lang="ts">
  import * as Empty from "$lib/components/ui/empty/index.js";
  import { Button } from "$lib/components/ui/button/index.js";
  import FolderCodeIcon from "@lucide/svelte/icons/folder-code";
  import {
    create_workspace,
    get_workspaces,
    update_workspace,
  } from "$lib/services/workspaces";
  import type { Workspace } from "$lib/types/workspace";
  import * as Card from "$lib/components/ui/card/index";
  import * as Dialog from "$lib/components/ui/dialog/index.js";
  import { Input } from "$lib/components/ui/input/index.js";
  import { Label } from "$lib/components/ui/label/index.js";
  import { goto } from "$app/navigation";

  $effect(() => {
    get_workspaces().then((ws) => {
      if (ws.length == 0) {
        showPrompt = true;
      } else {
        console.log(ws);
        workspaces = ws;
      }
    });
  });

  let showPrompt = $state(false);
  let workspaces = $state<Workspace[]>([]);
  let dialogOpen = $state(false);
  let dialogMode = $state<"create" | "edit">("create");
  let selectedWorkspace = $state<Workspace | null>(null);
  let workspaceName = $state("");
  let workspaceDescription = $state("");

  function openCreateDialog() {
    dialogMode = "create";
    selectedWorkspace = null;
    workspaceName = "";
    workspaceDescription = "";
    dialogOpen = true;
  }

  function openEditDialog(workspace: Workspace) {
    dialogMode = "edit";
    selectedWorkspace = workspace;
    workspaceName = workspace.Name ?? "";
    workspaceDescription = workspace.Description ?? "";
    dialogOpen = true;
  }

  function handleWorkspaceClick(id: string) {
    goto(`/workspaces/${id}`)
  }

  async function handleDialogSubmit() {
    let success = false;
    if (dialogMode === "create") {
      const w: Workspace = {
        // Id will be assigned in create_workspace
        Id: "",
        Name: workspaceName,
        Description: workspaceDescription,
      };
      success = await create_workspace(w);
    } else if (selectedWorkspace != null) {
      const w: Workspace = {
        Id: selectedWorkspace.Id,
        Name: workspaceName,
        Description: workspaceDescription,
      };
      success = await update_workspace(w);
    }

    if (success) {
      const updated = await get_workspaces();
      workspaces = updated;
      showPrompt = updated.length === 0;
    }

    dialogOpen = false;
  }
</script>

<Dialog.Root bind:open={dialogOpen}>
  {#if showPrompt}
    <Empty.Root
      class="flex min-h-[calc(100vh-4rem)] items-center justify-center"
    >
      <Empty.Header>
        <Empty.Media variant="icon">
          <FolderCodeIcon />
        </Empty.Media>
        <Empty.Title>No Workspaces Yet</Empty.Title>
        <Empty.Description>
          You haven't created any Workspaces yet. Get started by creating your
          first project.
        </Empty.Description>
      </Empty.Header>
      <Empty.Content>
        <div class="flex gap-2">
          <Button onclick={openCreateDialog}>Create Workspace</Button>
        </div>
      </Empty.Content>
    </Empty.Root>
  {:else}
    <div class="min-h-[calc(100vh-4rem)] p-6 space-y-4">
      <div class="flex items-center justify-between">
        <h1 class="text-2xl font-semibold">Workspaces</h1>
        <Button onclick={openCreateDialog}>Create Workspace</Button>
      </div>
      <div
        class="grid gap-6
          grid-cols-1
          sm:grid-cols-2
          md:grid-cols-3
          xl:grid-cols-4
          2xl:grid-cols-5"
      >
        {#each workspaces as workspace (workspace.Id)}
          <Card.Root
            class="min-h-40 w-full max-w-xs mx-auto flex flex-col justify-between cursor-pointer hover:shadow-md transition-shadow"
          >
            <Card.Header>
              <Card.Title class="truncate">{workspace.Name}</Card.Title>
              <Card.Description class="text-xs text-muted-foreground">
                {workspace.Description}
              </Card.Description>
            </Card.Header>
            <Card.Footer class="flex items-center justify-center gap-2">
              <Button size="sm" class="justify-center" onclick={() => handleWorkspaceClick(workspace.Id)}>Open</Button>
              <Button
                size="sm"
                class="justify-center"
                onclick={() => openEditDialog(workspace)}
              >
                Edit
              </Button>
            </Card.Footer>
          </Card.Root>
        {/each}
      </div>
    </div>
  {/if}

  <Dialog.Content class="sm:max-w-[425px]">
    <Dialog.Header>
      <Dialog.Title>
        {dialogMode === "create" ? "Create workspace" : "Edit workspace"}
      </Dialog.Title>
      <Dialog.Description>
        {#if dialogMode === "create"}
          Create a new workspace. Fill in the details below and click create.
        {:else}
          Update your workspace details and click save.
        {/if}
      </Dialog.Description>
    </Dialog.Header>
    <form class="grid gap-4 py-4" onsubmit={handleDialogSubmit}>
      <div class="grid grid-cols-4 items-center gap-4">
        <Label for="workspace-name" class="text-end">Name</Label>
        <Input
          id="workspace-name"
          class="col-span-3"
          bind:value={workspaceName}
        />
      </div>
      <div class="grid grid-cols-4 items-center gap-4">
        <Label for="workspace-description" class="text-end">Description</Label>
        <Input
          id="workspace-description"
          class="col-span-3"
          bind:value={workspaceDescription}
        />
      </div>
      <Dialog.Footer>
        <Button type="submit">
          {dialogMode === "create" ? "Create workspace" : "Save changes"}
        </Button>
      </Dialog.Footer>
    </form>
  </Dialog.Content>
</Dialog.Root>
