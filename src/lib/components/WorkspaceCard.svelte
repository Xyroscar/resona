<script lang="ts">
  import { goto } from '$app/navigation';
  import type { Workspace } from '../../types';
  import { formatDate } from '../utils/date';
  import { collections } from '$lib/stores/collection';
  import { workspaces } from '$lib/stores/workspace';
  import WorkspaceModal from './modals/WorkspaceModal.svelte';

  export let workspace: Workspace;
  let showEditModal = false;

  function openWorkspace() {
    goto(`/workspace/${workspace.id}`);
  }

  function handleKeyDown(event: KeyboardEvent) {
    if (event.key === 'Enter' || event.key === ' ') {
      event.preventDefault();
      openWorkspace();
    }
  }

  async function handleDelete(event: MouseEvent) {
    event.stopPropagation();
    if (confirm('Are you sure you want to delete this workspace?')) {
      try {
        await workspaces.deleteWorkspace(workspace.id);
      } catch (error) {
        console.error('Failed to delete workspace:', error);
      }
    }
  }

  function handleEdit(event: MouseEvent) {
    event.stopPropagation();
    showEditModal = true;
  }

  // Compute collections and variables count
  $: workspaceCollections = $collections.filter(c => c.workspace_id === workspace.id);
  $: workspaceVariables = workspace.variables || [];
</script>

<div class="w-full">
  <button
    class="card bg-base-100 shadow-lg hover:shadow-xl transition-shadow w-full text-left"
    on:click={openWorkspace}
    on:keydown={handleKeyDown}
  >
    <div class="card-body p-6">
      <div class="flex justify-between items-start">
        <h2 class="card-title text-xl">{workspace.name}</h2>
        <div class="dropdown dropdown-end">
            <!-- svelte-ignore a11y_click_events_have_key_events -->
            <!-- svelte-ignore a11y_no_static_element_interactions -->
            <div 
              class="btn btn-ghost btn-sm btn-square"
              on:click|stopPropagation
            >
            <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" viewBox="0 0 20 20" fill="currentColor">
              <path d="M10 6a2 2 0 110-4 2 2 0 010 4zM10 12a2 2 0 110-4 2 2 0 010 4zM10 18a2 2 0 110-4 2 2 0 010 4z" />
            </svg>
          </div>
          <ul class="dropdown-content menu p-2 shadow bg-base-100 rounded-box w-52">
            <li><a href="#top" on:click|preventDefault={handleEdit}>Edit</a></li>
            <li><a href="#top" class="text-error" on:click|preventDefault={handleDelete}>Delete</a></li>
          </ul>
        </div>
      </div>
      
      {#if workspace.description}
        <p class="text-base-content/70 text-sm mt-2">{workspace.description}</p>
      {/if}

      <div class="flex gap-6 mt-6">
        <div class="stat bg-base-200 rounded-lg p-4">
          <div class="stat-title text-xs">Collections</div>
          <div class="stat-value text-lg">{workspaceCollections.length}</div>
        </div>
        <div class="stat bg-base-200 rounded-lg p-4">
          <div class="stat-title text-xs">Variables</div>
          <div class="stat-value text-lg">{workspaceVariables.length}</div>
        </div>
      </div>

      <div class="card-actions justify-end mt-4">
        <div class="text-xs text-base-content/50">
          Updated {formatDate(workspace.updated_at)}
        </div>
      </div>
    </div>
  </button>
</div>

<WorkspaceModal 
  show={showEditModal}
  {workspace}
  on:close={() => showEditModal = false}
/> 