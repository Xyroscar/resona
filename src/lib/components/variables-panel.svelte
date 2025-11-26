<script lang="ts">
  import * as Dialog from "$lib/components/ui/dialog/index.js";
  import * as Tabs from "$lib/components/ui/tabs/index.js";
  import * as Select from "$lib/components/ui/select/index.js";
  import { Button } from "$lib/components/ui/button/index.js";
  import { Input } from "$lib/components/ui/input/index.js";
  import { Label } from "$lib/components/ui/label/index.js";
  import { Badge } from "$lib/components/ui/badge/index.js";
  import { Textarea } from "$lib/components/ui/textarea/index.js";
  import PlusIcon from "@lucide/svelte/icons/plus";
  import TrashIcon from "@lucide/svelte/icons/trash-2";
  import PencilIcon from "@lucide/svelte/icons/pencil";
  import LockIcon from "@lucide/svelte/icons/lock";
  import EyeIcon from "@lucide/svelte/icons/eye";
  import EyeOffIcon from "@lucide/svelte/icons/eye-off";
  import VariableIcon from "@lucide/svelte/icons/variable";
  import type { Variable, VariableScope } from "$lib/types/variable";
  import {
    get_global_variables,
    get_workspace_variables,
    get_collection_variables,
    create_variable,
    update_variable,
    delete_variable,
  } from "$lib/services/variables";
  import { onMount } from "svelte";

  type Props = {
    open: boolean;
    onOpenChange: (open: boolean) => void;
    workspaceId?: string;
    collectionId?: string;
  };

  let {
    open = $bindable(),
    onOpenChange,
    workspaceId,
    collectionId,
  }: Props = $props();

  let globalVariables = $state<Variable[]>([]);
  let workspaceVariables = $state<Variable[]>([]);
  let collectionVariables = $state<Variable[]>([]);

  let editDialogOpen = $state(false);
  let editingVariable = $state<Variable | null>(null);
  let isCreating = $state(false);

  let formName = $state("");
  let formValue = $state("");
  let formDescription = $state("");
  let formIsSecret = $state(false);
  let formScope = $state<VariableScope>("global");

  let showSecretValues = $state<Set<string>>(new Set());

  $effect(() => {
    if (open) {
      loadVariables();
    }
  });

  async function loadVariables() {
    globalVariables = await get_global_variables();
    if (workspaceId) {
      workspaceVariables = await get_workspace_variables(workspaceId);
    }
    if (collectionId) {
      collectionVariables = await get_collection_variables(collectionId);
    }
  }

  function openCreateDialog(scope: VariableScope) {
    isCreating = true;
    editingVariable = null;
    formName = "";
    formValue = "";
    formDescription = "";
    formIsSecret = false;
    formScope = scope;
    editDialogOpen = true;
  }

  function openEditDialog(variable: Variable) {
    isCreating = false;
    editingVariable = variable;
    formName = variable.name;
    formValue = variable.value;
    formDescription = variable.description || "";
    formIsSecret = variable.isSecret;
    formScope = variable.scope;
    editDialogOpen = true;
  }

  async function handleSave() {
    const scopeId =
      formScope === "global"
        ? null
        : formScope === "workspace"
          ? (workspaceId ?? null)
          : formScope === "collection"
            ? (collectionId ?? null)
            : null;

    if (isCreating) {
      await create_variable({
        name: formName,
        value: formValue,
        scope: formScope,
        scopeId,
        isSecret: formIsSecret,
        description: formDescription || undefined,
      });
    } else if (editingVariable) {
      await update_variable(editingVariable.id, {
        name: formName,
        value: formValue,
        isSecret: formIsSecret,
        description: formDescription || undefined,
      });
    }

    await loadVariables();
    editDialogOpen = false;
  }

  async function handleDelete(variable: Variable) {
    await delete_variable(variable.id);
    await loadVariables();
  }

  function toggleSecretVisibility(id: string) {
    if (showSecretValues.has(id)) {
      showSecretValues.delete(id);
    } else {
      showSecretValues.add(id);
    }
    showSecretValues = new Set(showSecretValues);
  }

  function getScopeColor(scope: VariableScope): string {
    const colors: Record<VariableScope, string> = {
      global: "bg-purple-500/10 text-purple-500",
      workspace: "bg-blue-500/10 text-blue-500",
      collection: "bg-green-500/10 text-green-500",
      request: "bg-orange-500/10 text-orange-500",
    };
    return colors[scope];
  }

  function getMaskedValue(value: string): string {
    return "•".repeat(Math.min(value.length, 20));
  }
</script>

<Dialog.Root bind:open {onOpenChange}>
  <Dialog.Content
    class="sm:max-w-[700px] max-h-[80vh] overflow-hidden flex flex-col"
  >
    <Dialog.Header>
      <Dialog.Title class="flex items-center gap-2">
        <VariableIcon class="size-5" />
        Variables & Secrets
      </Dialog.Title>
      <Dialog.Description>
        Manage variables and secrets at different scopes. Variables can be used
        in requests with {"{{VAR_NAME}}"} syntax.
      </Dialog.Description>
    </Dialog.Header>

    <Tabs.Root value="global" class="flex-1 overflow-hidden">
      <Tabs.List class="grid w-full grid-cols-3">
        <Tabs.Trigger value="global">Global</Tabs.Trigger>
        <Tabs.Trigger value="workspace" disabled={!workspaceId}
          >Workspace</Tabs.Trigger
        >
        <Tabs.Trigger value="collection" disabled={!collectionId}
          >Collection</Tabs.Trigger
        >
      </Tabs.List>

      <div class="mt-4 overflow-auto max-h-[400px]">
        <Tabs.Content value="global" class="space-y-2">
          <div class="flex justify-between items-center mb-4">
            <p class="text-sm text-muted-foreground">
              Global variables are available in all workspaces.
            </p>
            <Button size="sm" onclick={() => openCreateDialog("global")}>
              <PlusIcon class="size-4 mr-1" />
              Add Variable
            </Button>
          </div>
          {#if globalVariables.length === 0}
            <div class="text-center py-8 text-muted-foreground">
              No global variables defined.
            </div>
          {:else}
            {#each globalVariables as variable (variable.id)}
              {@render variableRow(variable)}
            {/each}
          {/if}
        </Tabs.Content>

        <Tabs.Content value="workspace" class="space-y-2">
          <div class="flex justify-between items-center mb-4">
            <p class="text-sm text-muted-foreground">
              Workspace variables are available in this workspace only.
            </p>
            <Button
              size="sm"
              onclick={() => openCreateDialog("workspace")}
              disabled={!workspaceId}
            >
              <PlusIcon class="size-4 mr-1" />
              Add Variable
            </Button>
          </div>
          {#if workspaceVariables.length === 0}
            <div class="text-center py-8 text-muted-foreground">
              No workspace variables defined.
            </div>
          {:else}
            {#each workspaceVariables as variable (variable.id)}
              {@render variableRow(variable)}
            {/each}
          {/if}
        </Tabs.Content>

        <Tabs.Content value="collection" class="space-y-2">
          <div class="flex justify-between items-center mb-4">
            <p class="text-sm text-muted-foreground">
              Collection variables are available in this collection only.
            </p>
            <Button
              size="sm"
              onclick={() => openCreateDialog("collection")}
              disabled={!collectionId}
            >
              <PlusIcon class="size-4 mr-1" />
              Add Variable
            </Button>
          </div>
          {#if collectionVariables.length === 0}
            <div class="text-center py-8 text-muted-foreground">
              No collection variables defined.
            </div>
          {:else}
            {#each collectionVariables as variable (variable.id)}
              {@render variableRow(variable)}
            {/each}
          {/if}
        </Tabs.Content>
      </div>
    </Tabs.Root>

    <Dialog.Footer class="mt-4">
      <Button variant="outline" onclick={() => onOpenChange(false)}>
        Close
      </Button>
    </Dialog.Footer>
  </Dialog.Content>
</Dialog.Root>

{#snippet variableRow(variable: Variable)}
  <div
    class="flex items-center gap-3 p-3 rounded-lg border bg-card hover:bg-accent/50 transition-colors"
  >
    <div class="flex-1 min-w-0">
      <div class="flex items-center gap-2">
        <span class="font-mono font-medium text-sm">{variable.name}</span>
        {#if variable.isSecret}
          <LockIcon class="size-3 text-muted-foreground" />
        {/if}
        <Badge class={getScopeColor(variable.scope)} variant="secondary">
          {variable.scope}
        </Badge>
      </div>
      <div class="flex items-center gap-2 mt-1">
        <code
          class="text-xs text-muted-foreground font-mono truncate max-w-[300px]"
        >
          {#if variable.isSecret && !showSecretValues.has(variable.id)}
            {getMaskedValue(variable.value)}
          {:else}
            {variable.value}
          {/if}
        </code>
        {#if variable.isSecret}
          <button
            type="button"
            class="text-muted-foreground hover:text-foreground"
            onclick={() => toggleSecretVisibility(variable.id)}
            aria-label={showSecretValues.has(variable.id)
              ? "Hide value"
              : "Show value"}
          >
            {#if showSecretValues.has(variable.id)}
              <EyeOffIcon class="size-3" />
            {:else}
              <EyeIcon class="size-3" />
            {/if}
          </button>
        {/if}
      </div>
      {#if variable.description}
        <p class="text-xs text-muted-foreground mt-1">{variable.description}</p>
      {/if}
    </div>
    <div class="flex items-center gap-1">
      <Button
        variant="ghost"
        size="icon"
        onclick={() => openEditDialog(variable)}
        aria-label="Edit variable"
      >
        <PencilIcon class="size-4" />
      </Button>
      <Button
        variant="ghost"
        size="icon"
        onclick={() => handleDelete(variable)}
        aria-label="Delete variable"
      >
        <TrashIcon class="size-4 text-destructive" />
      </Button>
    </div>
  </div>
{/snippet}

<Dialog.Root bind:open={editDialogOpen}>
  <Dialog.Content class="sm:max-w-[425px]">
    <Dialog.Header>
      <Dialog.Title>
        {isCreating ? "Create Variable" : "Edit Variable"}
      </Dialog.Title>
      <Dialog.Description>
        {isCreating
          ? "Add a new variable or secret."
          : "Update variable details."}
      </Dialog.Description>
    </Dialog.Header>
    <form
      class="grid gap-4 py-4"
      onsubmit={(e) => {
        e.preventDefault();
        handleSave();
      }}
    >
      <div class="grid grid-cols-4 items-center gap-4">
        <Label for="var-name" class="text-end">Name</Label>
        <Input
          id="var-name"
          class="col-span-3 font-mono"
          placeholder="VARIABLE_NAME"
          bind:value={formName}
          pattern="[A-Za-z_][A-Za-z0-9_]*"
        />
      </div>
      <div class="grid grid-cols-4 items-center gap-4">
        <Label for="var-value" class="text-end">Value</Label>
        <Input
          id="var-value"
          class="col-span-3 font-mono"
          type={formIsSecret ? "password" : "text"}
          placeholder="Variable value"
          bind:value={formValue}
        />
      </div>
      <div class="grid grid-cols-4 items-center gap-4">
        <Label for="var-desc" class="text-end">Description</Label>
        <Textarea
          id="var-desc"
          class="col-span-3"
          placeholder="Optional description"
          bind:value={formDescription}
          rows={2}
        />
      </div>
      <div class="grid grid-cols-4 items-center gap-4">
        <Label class="text-end">Secret</Label>
        <div class="col-span-3 flex items-center gap-2">
          <button
            type="button"
            role="switch"
            aria-checked={formIsSecret}
            aria-label="Mark as secret"
            class="relative inline-flex h-6 w-11 shrink-0 cursor-pointer rounded-full border-2 border-transparent transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 {formIsSecret
              ? 'bg-primary'
              : 'bg-input'}"
            onclick={() => (formIsSecret = !formIsSecret)}
          >
            <span
              class="pointer-events-none block h-5 w-5 rounded-full bg-background shadow-lg ring-0 transition-transform {formIsSecret
                ? 'translate-x-5'
                : 'translate-x-0'}"
            ></span>
          </button>
          <span class="text-sm text-muted-foreground">
            {formIsSecret ? "Value will be hidden" : "Value will be visible"}
          </span>
        </div>
      </div>
      <Dialog.Footer>
        <Button
          type="button"
          variant="outline"
          onclick={() => (editDialogOpen = false)}
        >
          Cancel
        </Button>
        <Button type="submit" disabled={!formName.trim()}>
          {isCreating ? "Create" : "Save"}
        </Button>
      </Dialog.Footer>
    </form>
  </Dialog.Content>
</Dialog.Root>
