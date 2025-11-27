<script lang="ts">
  import * as Empty from "$lib/components/ui/empty/index.js";
  import * as DropdownMenu from "$lib/components/ui/dropdown-menu/index.js";
  import { Button } from "$lib/components/ui/button/index.js";
  import { Badge } from "$lib/components/ui/badge/index.js";
  import FolderCodeIcon from "@lucide/svelte/icons/folder-code";
  import SettingsIcon from "@lucide/svelte/icons/settings";
  import VariableIcon from "@lucide/svelte/icons/variable";
  import MoreVerticalIcon from "@lucide/svelte/icons/more-vertical";
  import CopyIcon from "@lucide/svelte/icons/copy";
  import PencilIcon from "@lucide/svelte/icons/pencil";
  import TrashIcon from "@lucide/svelte/icons/trash-2";
  import {
    create_workspace,
    get_workspaces,
    update_workspace,
    delete_workspace,
  } from "$lib/services/workspaces";
  import {
    duplicate_workspace,
    type DuplicateWorkspaceOptions,
  } from "$lib/services/sync";
  import type { Workspace } from "$lib/types/workspace";
  import * as Card from "$lib/components/ui/card/index";
  import * as Dialog from "$lib/components/ui/dialog/index.js";
  import { Input } from "$lib/components/ui/input/index.js";
  import { Label } from "$lib/components/ui/label/index.js";
  import { goto } from "$app/navigation";
  import SettingsDialog from "$lib/components/settings-dialog.svelte";
  import VariablesPanel from "$lib/components/variables-panel.svelte";
  import DuplicateWorkspaceDialog from "$lib/components/duplicate-workspace-dialog.svelte";

  $effect(() => {
    loadWorkspaces();
  });

  async function loadWorkspaces() {
    const ws = await get_workspaces();
    if (ws.length == 0) {
      showPrompt = true;
    } else {
      workspaces = ws;
      showPrompt = false;
    }
  }

  let showPrompt = $state(false);
  let workspaces = $state<Workspace[]>([]);
  let dialogOpen = $state(false);
  let dialogMode = $state<"create" | "edit">("create");
  let selectedWorkspace = $state<Workspace | null>(null);
  let workspaceName = $state("");
  let workspaceDescription = $state("");

  let settingsOpen = $state(false);
  let variablesOpen = $state(false);
  let duplicateDialogOpen = $state(false);
  let workspaceToDuplicate = $state<Workspace | null>(null);
  let deleteDialogOpen = $state(false);
  let workspaceToDelete = $state<Workspace | null>(null);

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

  function openDuplicateDialog(workspace: Workspace) {
    workspaceToDuplicate = workspace;
    duplicateDialogOpen = true;
  }

  function openDeleteDialog(workspace: Workspace) {
    workspaceToDelete = workspace;
    deleteDialogOpen = true;
  }

  function handleWorkspaceClick(id: string) {
    goto(`/workspaces/${id}`);
  }

  async function handleDialogSubmit() {
    if (dialogMode === "create") {
      await create_workspace({
        name: workspaceName,
        description: workspaceDescription,
        tags: [],
      });
    } else if (selectedWorkspace != null) {
      await update_workspace({
        id: selectedWorkspace.Id,
        name: workspaceName,
        description: workspaceDescription,
      });
    }

    await loadWorkspaces();
    dialogOpen = false;
  }

  async function handleDuplicate(options: DuplicateWorkspaceOptions) {
    await duplicate_workspace(options, async (input) => {
      return await create_workspace(input);
    });
    await loadWorkspaces();
  }

  async function handleDeleteConfirm() {
    if (workspaceToDelete) {
      await delete_workspace(workspaceToDelete.Id);
      await loadWorkspaces();
    }
    deleteDialogOpen = false;
    workspaceToDelete = null;
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
        <div class="flex items-center gap-2">
          <Button
            variant="outline"
            size="icon"
            onclick={() => (variablesOpen = true)}
            aria-label="Global Variables"
          >
            <VariableIcon class="size-4" />
          </Button>
          <Button
            variant="outline"
            size="icon"
            onclick={() => (settingsOpen = true)}
            aria-label="Settings"
          >
            <SettingsIcon class="size-4" />
          </Button>
          <Button onclick={openCreateDialog}>Create Workspace</Button>
        </div>
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
            class="min-h-40 w-full max-w-xs mx-auto flex flex-col justify-between hover:shadow-md transition-shadow group"
          >
            <Card.Header class="relative">
              <div
                class="absolute top-2 right-2 opacity-0 group-hover:opacity-100 transition-opacity"
              >
                <DropdownMenu.Root>
                  <DropdownMenu.Trigger>
                    {#snippet child({ props })}
                      <Button
                        variant="ghost"
                        size="icon"
                        class="h-8 w-8"
                        {...props}
                        aria-label="Workspace options"
                      >
                        <MoreVerticalIcon class="size-4" />
                      </Button>
                    {/snippet}
                  </DropdownMenu.Trigger>
                  <DropdownMenu.Content align="end">
                    <DropdownMenu.Item
                      onclick={() => openEditDialog(workspace)}
                    >
                      <PencilIcon class="size-4 mr-2" />
                      Edit
                    </DropdownMenu.Item>
                    <DropdownMenu.Item
                      onclick={() => openDuplicateDialog(workspace)}
                    >
                      <CopyIcon class="size-4 mr-2" />
                      Duplicate
                    </DropdownMenu.Item>
                    <DropdownMenu.Separator />
                    <DropdownMenu.Item
                      class="text-destructive"
                      onclick={() => openDeleteDialog(workspace)}
                    >
                      <TrashIcon class="size-4 mr-2" />
                      Delete
                    </DropdownMenu.Item>
                  </DropdownMenu.Content>
                </DropdownMenu.Root>
              </div>
              <Card.Title class="truncate pr-8">{workspace.Name}</Card.Title>
              <Card.Description
                class="text-xs text-muted-foreground line-clamp-2"
              >
                {workspace.Description}
              </Card.Description>
              {#if workspace.Tags && workspace.Tags.length > 0}
                <div class="flex flex-wrap gap-1 mt-2">
                  {#each workspace.Tags as tag}
                    <Badge variant="secondary" class="text-xs">{tag}</Badge>
                  {/each}
                </div>
              {/if}
            </Card.Header>
            <Card.Footer class="flex items-center justify-center gap-2">
              <Button
                size="sm"
                class="justify-center flex-1"
                onclick={() => handleWorkspaceClick(workspace.Id)}
              >
                Open
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

<Dialog.Root bind:open={deleteDialogOpen}>
  <Dialog.Content class="sm:max-w-[425px]">
    <Dialog.Header>
      <Dialog.Title>Delete Workspace</Dialog.Title>
      <Dialog.Description>
        Are you sure you want to delete "{workspaceToDelete?.Name}"? This will
        permanently delete all collections, requests, and variables in this
        workspace.
      </Dialog.Description>
    </Dialog.Header>
    <Dialog.Footer>
      <Button variant="outline" onclick={() => (deleteDialogOpen = false)}
        >Cancel</Button
      >
      <Button variant="destructive" onclick={handleDeleteConfirm}>Delete</Button
      >
    </Dialog.Footer>
  </Dialog.Content>
</Dialog.Root>

<SettingsDialog
  bind:open={settingsOpen}
  onOpenChange={(v) => (settingsOpen = v)}
/>

<VariablesPanel
  bind:open={variablesOpen}
  onOpenChange={(v) => (variablesOpen = v)}
/>

<DuplicateWorkspaceDialog
  bind:open={duplicateDialogOpen}
  onOpenChange={(v) => (duplicateDialogOpen = v)}
  sourceWorkspace={workspaceToDuplicate}
  onDuplicate={handleDuplicate}
/>
