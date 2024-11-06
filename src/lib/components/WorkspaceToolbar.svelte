<script lang="ts">
  import { goto } from '$app/navigation';
  import { activeWorkspace } from '../stores/workspace';
  import VariablesModal from './modals/VariablesModal.svelte';

  let showVariablesModal = false;

  function goToWorkspaces() {
    goto('/');
  }
</script>

<div class="navbar bg-base-100 border-b border-base-300">
  <div class="flex-none lg:hidden">
    <label for="drawer" class="btn btn-square btn-ghost">
      <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" class="inline-block w-6 h-6 stroke-current">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16"></path>
      </svg>
    </label>
  </div>

  <div class="flex-1 flex items-center gap-2">
    <button 
      class="btn btn-ghost btn-sm"
      on:click={goToWorkspaces}
    >
      <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" viewBox="0 0 20 20" fill="currentColor">
        <path fill-rule="evenodd" d="M9.707 16.707a1 1 0 01-1.414 0l-6-6a1 1 0 010-1.414l6-6a1 1 0 011.414 1.414L5.414 9H17a1 1 0 110 2H5.414l4.293 4.293a1 1 0 010 1.414z" clip-rule="evenodd" />
      </svg>
      All Workspaces
    </button>
    <span class="text-xl font-bold">{$activeWorkspace?.name}</span>
  </div>

  <div class="flex-none">
    <button 
      class="btn btn-ghost btn-sm"
      on:click={() => showVariablesModal = true}
    >
      <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 mr-2" viewBox="0 0 20 20" fill="currentColor">
        <path fill-rule="evenodd" d="M4.649 3.084A1 1 0 015.163 4.4 13.95 13.95 0 004 10c0 1.993.416 3.886 1.164 5.6a1 1 0 01-1.832.8A15.95 15.95 0 012 10c0-2.274.475-4.44 1.332-6.4a1 1 0 011.317-.516zM12.96 7a3 3 0 00-2.342 1.126l-.328.41-.111-.279A2 2 0 008.323 7H8a1 1 0 000 2h.323l.532 1.33-1.035 1.295a1 1 0 01-.781.375H7a1 1 0 100 2h.039a3 3 0 002.342-1.126l.328-.41.111.279A2 2 0 0011.677 14H12a1 1 0 100-2h-.323l-.532-1.33 1.035-1.295A1 1 0 0112.961 9H13a1 1 0 100-2h-.039z" clip-rule="evenodd" />
      </svg>
      Variables
    </button>
  </div>
</div>

<VariablesModal
  show={showVariablesModal}
  on:close={() => showVariablesModal = false}
/> 