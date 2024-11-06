<script lang="ts">
  import { page } from '$app/stores';
  import { workspaces, activeWorkspace } from '$lib/stores/workspace';
  import { onMount } from 'svelte';
  import Sidebar from '$lib/components/Sidebar.svelte';
  import WorkspaceToolbar from '$lib/components/WorkspaceToolbar.svelte';

  let drawerOpen = true;

  onMount(async () => {
    const workspace = await workspaces.getWorkspace($page.params.id);
    if (workspace) {
      activeWorkspace.set(workspace);
    }
  });
</script>

<div class="drawer lg:drawer-open">
  <input 
    id="drawer" 
    type="checkbox" 
    class="drawer-toggle" 
    bind:checked={drawerOpen}
  />
  
  <div class="drawer-content flex flex-col">
    {#if drawerOpen}
      <button 
        class="lg:hidden fixed bottom-4 right-4 btn btn-circle btn-primary z-50"
        on:click={() => drawerOpen = false}
      >
        <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
        </svg>
      </button>
    {/if}
    <WorkspaceToolbar />
    <slot />
  </div>

  <div class="drawer-side">
    <label 
      for="drawer" 
      class="drawer-overlay" 
      on:click={() => drawerOpen = false}
    ></label>
    <Sidebar />
  </div>
</div> 