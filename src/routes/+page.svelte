<script lang="ts">
  import { workspaces } from '$lib/stores/workspace';
  import { onMount } from 'svelte';
  import WorkspaceCard from '$lib/components/WorkspaceCard.svelte';
  import WorkspaceModal from '$lib/components/modals/WorkspaceModal.svelte';

  let showCreateModal = false;
  let loading = true;
  let error: string | null = null;

  onMount(async () => {
    try {
      await workspaces.loadWorkspaces();
    } catch (err) {
      console.error('Failed to load workspaces:', err);
      error = 'Failed to load workspaces';
    } finally {
      loading = false;
    }
  });
</script>

<div class="container mx-auto px-6 py-8 max-w-7xl">
  <div class="flex justify-between items-center mb-8">
    <div>
      <h1 class="text-2xl font-bold">Workspaces</h1>
      <p class="text-base-content/70">Manage your API collections and environments</p>
    </div>
    <button 
      class="btn btn-primary"
      on:click={() => showCreateModal = true}
    >
      <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 mr-2" viewBox="0 0 20 20" fill="currentColor">
        <path fill-rule="evenodd" d="M10 3a1 1 0 011 1v5h5a1 1 0 110 2h-5v5a1 1 0 11-2 0v-5H4a1 1 0 110-2h5V4a1 1 0 011-1z" clip-rule="evenodd" />
      </svg>
      New Workspace
    </button>
  </div>

  {#if loading}
    <div class="flex justify-center py-16">
      <span class="loading loading-spinner loading-lg"></span>
    </div>
  {:else if error}
    <div class="alert alert-error">
      <svg xmlns="http://www.w3.org/2000/svg" class="stroke-current shrink-0 h-6 w-6" fill="none" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z" /></svg>
      <span>{error}</span>
    </div>
  {:else if $workspaces.length === 0}
    <div class="flex flex-col items-center justify-center py-16 text-center">
      <svg xmlns="http://www.w3.org/2000/svg" class="h-16 w-16 text-base-content/20" fill="none" viewBox="0 0 24 24" stroke="currentColor">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M20 7l-8-4-8 4m16 0l-8 4m8-4v10l-8 4m0-10L4 7m8 4v10M4 7v10l8 4" />
      </svg>
      <h3 class="mt-4 text-lg font-medium">No workspaces yet</h3>
      <p class="mt-1 text-base-content/70">Create a workspace to get started with your API collections</p>
      <button 
        class="btn btn-primary mt-4"
        on:click={() => showCreateModal = true}
      >
        Create Your First Workspace
      </button>
    </div>
  {:else}
    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8">
      {#each $workspaces as workspace}
        <WorkspaceCard {workspace} />
      {/each}
    </div>
  {/if}
</div>

<WorkspaceModal 
  show={showCreateModal}
  on:close={() => showCreateModal = false}
/>
