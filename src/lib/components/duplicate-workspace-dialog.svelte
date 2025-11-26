<script lang="ts">
  import * as Dialog from "$lib/components/ui/dialog/index.js";
  import { Button } from "$lib/components/ui/button/index.js";
  import { Input } from "$lib/components/ui/input/index.js";
  import { Label } from "$lib/components/ui/label/index.js";
  import { Textarea } from "$lib/components/ui/textarea/index.js";
  import { Badge } from "$lib/components/ui/badge/index.js";
  import { Separator } from "$lib/components/ui/separator/index.js";
  import CopyIcon from "@lucide/svelte/icons/copy";
  import LinkIcon from "@lucide/svelte/icons/link";
  import type { Workspace } from "$lib/types/workspace";
  import type { Variable } from "$lib/types/variable";
  import { get_workspace_variables } from "$lib/services/variables";
  import {
    duplicate_workspace,
    type DuplicateWorkspaceOptions,
  } from "$lib/services/sync";

  type Props = {
    open: boolean;
    onOpenChange: (open: boolean) => void;
    sourceWorkspace: Workspace | null;
    onDuplicate: (options: DuplicateWorkspaceOptions) => Promise<void>;
  };

  let {
    open = $bindable(),
    onOpenChange,
    sourceWorkspace,
    onDuplicate,
  }: Props = $props();

  let newName = $state("");
  let newDescription = $state("");
  let environment = $state("");
  let copyVariables = $state(true);
  let copySecrets = $state(false);
  let createSyncGroup = $state(false);
  let syncGroupName = $state("");
  let variablesToSync = $state<string[]>([]);

  let workspaceVariables = $state<Variable[]>([]);
  let loading = $state(false);

  $effect(() => {
    if (open && sourceWorkspace) {
      newName = `${sourceWorkspace.Name} (Copy)`;
      newDescription = sourceWorkspace.Description;
      environment = "";
      syncGroupName = `${sourceWorkspace.Name} Environments`;
      loadVariables();
    }
  });

  async function loadVariables() {
    if (sourceWorkspace) {
      workspaceVariables = await get_workspace_variables(sourceWorkspace.Id);
    }
  }

  function toggleVariableSync(varName: string) {
    if (variablesToSync.includes(varName)) {
      variablesToSync = variablesToSync.filter((v) => v !== varName);
    } else {
      variablesToSync = [...variablesToSync, varName];
    }
  }

  function selectAllVariables() {
    variablesToSync = workspaceVariables
      .filter((v) => !v.isSecret || copySecrets)
      .map((v) => v.name);
  }

  function deselectAllVariables() {
    variablesToSync = [];
  }

  async function handleSubmit() {
    if (!sourceWorkspace) return;

    loading = true;
    try {
      await onDuplicate({
        sourceWorkspaceId: sourceWorkspace.Id,
        newName,
        newDescription,
        environment: environment || undefined,
        copyVariables,
        copySecrets,
        createSyncGroup,
        syncGroupName: createSyncGroup ? syncGroupName : undefined,
        variablesToSync: createSyncGroup ? variablesToSync : undefined,
      });
      onOpenChange(false);
    } finally {
      loading = false;
    }
  }
</script>

<Dialog.Root bind:open {onOpenChange}>
  <Dialog.Content
    class="sm:max-w-[550px] max-h-[85vh] overflow-hidden flex flex-col"
  >
    <Dialog.Header>
      <Dialog.Title class="flex items-center gap-2">
        <CopyIcon class="size-5" />
        Duplicate Workspace
      </Dialog.Title>
      <Dialog.Description>
        Create a copy of "{sourceWorkspace?.Name}" with all its collections and
        requests.
      </Dialog.Description>
    </Dialog.Header>

    <form
      class="flex-1 overflow-auto space-y-4 py-4"
      onsubmit={(e) => {
        e.preventDefault();
        handleSubmit();
      }}
    >
      <div class="space-y-4">
        <div class="grid grid-cols-4 items-center gap-4">
          <Label for="new-name" class="text-end">Name</Label>
          <Input
            id="new-name"
            class="col-span-3"
            placeholder="New workspace name"
            bind:value={newName}
          />
        </div>

        <div class="grid grid-cols-4 items-center gap-4">
          <Label for="new-desc" class="text-end">Description</Label>
          <Textarea
            id="new-desc"
            class="col-span-3"
            placeholder="Workspace description"
            bind:value={newDescription}
            rows={2}
          />
        </div>

        <div class="grid grid-cols-4 items-center gap-4">
          <Label for="environment" class="text-end">Environment</Label>
          <Input
            id="environment"
            class="col-span-3"
            placeholder="e.g., Development, Staging, Production"
            bind:value={environment}
          />
        </div>
      </div>

      <Separator />

      <div class="space-y-4">
        <h4 class="font-medium">Variables</h4>

        <div class="flex items-center justify-between">
          <div class="space-y-0.5">
            <Label>Copy Variables</Label>
            <p class="text-xs text-muted-foreground">
              Copy workspace variables to the new workspace.
            </p>
          </div>
          <button
            type="button"
            role="switch"
            aria-checked={copyVariables}
            aria-label="Toggle copy variables"
            class="relative inline-flex h-6 w-11 shrink-0 cursor-pointer rounded-full border-2 border-transparent transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 {copyVariables
              ? 'bg-primary'
              : 'bg-input'}"
            onclick={() => (copyVariables = !copyVariables)}
          >
            <span
              class="pointer-events-none block h-5 w-5 rounded-full bg-background shadow-lg ring-0 transition-transform {copyVariables
                ? 'translate-x-5'
                : 'translate-x-0'}"
            ></span>
          </button>
        </div>

        {#if copyVariables}
          <div class="flex items-center justify-between">
            <div class="space-y-0.5">
              <Label>Copy Secret Values</Label>
              <p class="text-xs text-muted-foreground">
                Copy secret values (otherwise they'll be empty).
              </p>
            </div>
            <button
              type="button"
              role="switch"
              aria-checked={copySecrets}
              aria-label="Toggle copy secrets"
              class="relative inline-flex h-6 w-11 shrink-0 cursor-pointer rounded-full border-2 border-transparent transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 {copySecrets
                ? 'bg-primary'
                : 'bg-input'}"
              onclick={() => (copySecrets = !copySecrets)}
            >
              <span
                class="pointer-events-none block h-5 w-5 rounded-full bg-background shadow-lg ring-0 transition-transform {copySecrets
                  ? 'translate-x-5'
                  : 'translate-x-0'}"
              ></span>
            </button>
          </div>
        {/if}
      </div>

      <Separator />

      <div class="space-y-4">
        <div class="flex items-center justify-between">
          <div class="space-y-0.5">
            <div class="flex items-center gap-2">
              <LinkIcon class="size-4" />
              <Label>Create Sync Group</Label>
            </div>
            <p class="text-xs text-muted-foreground">
              Link workspaces to sync selected variables between environments.
            </p>
          </div>
          <button
            type="button"
            role="switch"
            aria-checked={createSyncGroup}
            aria-label="Toggle create sync group"
            class="relative inline-flex h-6 w-11 shrink-0 cursor-pointer rounded-full border-2 border-transparent transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 {createSyncGroup
              ? 'bg-primary'
              : 'bg-input'}"
            onclick={() => (createSyncGroup = !createSyncGroup)}
          >
            <span
              class="pointer-events-none block h-5 w-5 rounded-full bg-background shadow-lg ring-0 transition-transform {createSyncGroup
                ? 'translate-x-5'
                : 'translate-x-0'}"
            ></span>
          </button>
        </div>

        {#if createSyncGroup}
          <div class="grid grid-cols-4 items-center gap-4">
            <Label for="sync-name" class="text-end">Group Name</Label>
            <Input
              id="sync-name"
              class="col-span-3"
              placeholder="Sync group name"
              bind:value={syncGroupName}
            />
          </div>

          <div class="space-y-2">
            <div class="flex items-center justify-between">
              <Label>Variables to Sync</Label>
              <div class="flex gap-2">
                <Button
                  type="button"
                  variant="ghost"
                  size="sm"
                  onclick={selectAllVariables}
                >
                  Select All
                </Button>
                <Button
                  type="button"
                  variant="ghost"
                  size="sm"
                  onclick={deselectAllVariables}
                >
                  Deselect All
                </Button>
              </div>
            </div>
            <p class="text-xs text-muted-foreground mb-2">
              Selected variables will be kept in sync across linked workspaces.
            </p>

            {#if workspaceVariables.length === 0}
              <div class="text-center py-4 text-muted-foreground text-sm">
                No workspace variables to sync.
              </div>
            {:else}
              <div
                class="grid grid-cols-2 gap-2 max-h-[150px] overflow-auto p-2 border rounded-md"
              >
                {#each workspaceVariables as variable (variable.id)}
                  {#if !variable.isSecret || copySecrets}
                    <button
                      type="button"
                      class="flex items-center gap-2 p-2 rounded text-left text-sm hover:bg-accent transition-colors {variablesToSync.includes(
                        variable.name
                      )
                        ? 'bg-accent'
                        : ''}"
                      onclick={() => toggleVariableSync(variable.name)}
                    >
                      <input
                        type="checkbox"
                        checked={variablesToSync.includes(variable.name)}
                        class="rounded"
                        onchange={() => {}}
                      />
                      <span class="font-mono text-xs truncate"
                        >{variable.name}</span
                      >
                      {#if variable.isSecret}
                        <Badge variant="secondary" class="text-xs">secret</Badge
                        >
                      {/if}
                    </button>
                  {/if}
                {/each}
              </div>
            {/if}
          </div>
        {/if}
      </div>

      <Dialog.Footer class="pt-4">
        <Button
          type="button"
          variant="outline"
          onclick={() => onOpenChange(false)}
        >
          Cancel
        </Button>
        <Button type="submit" disabled={loading || !newName.trim()}>
          {loading ? "Creating..." : "Duplicate Workspace"}
        </Button>
      </Dialog.Footer>
    </form>
  </Dialog.Content>
</Dialog.Root>
