<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import { workspaces } from '../../stores/workspace';
  import type { Workspace } from '../../../types';

  export let show = false;
  export let workspace: Workspace | null = null;
  
  let name = '';
  let description = '';
  let isSubmitting = false;
  let error = '';
  let initialized = false;

  $: if (show && !initialized) {
    if (workspace) {
      name = workspace.name;
      description = workspace.description || '';
    } else {
      name = '';
      description = '';
    }
    initialized = true;
    error = '';
  }

  $: if (!show) {
    initialized = false;
  }

  const dispatch = createEventDispatcher<{
    close: void;
  }>();

  async function handleSubmit(event: SubmitEvent) {
    event.preventDefault();
    if (isSubmitting) return;
    error = '';
    
    try {
      isSubmitting = true;
      if (workspace) {
        await workspaces.updateWorkspace(
          workspace.id,
          name,
          description || undefined
        );
      } else {
        await workspaces.createWorkspace(
          name,
          description || undefined
        );
      }
      
      handleClose();
    } catch (e) {
      error = e instanceof Error ? e.message : 'Failed to save workspace';
      console.error('Failed to save workspace:', e);
    } finally {
      isSubmitting = false;
    }
  }

  function handleClose() {
    name = '';
    description = '';
    error = '';
    initialized = false;
    dispatch('close');
  }
</script>

<div class="modal {show ? 'modal-open' : ''}" role="dialog">
  <div class="modal-box">
    <h3 class="font-bold text-lg">
      {workspace ? 'Edit' : 'Create New'} Workspace
    </h3>
    
    <form on:submit={handleSubmit} class="mt-4">
      {#if error}
        <div class="alert alert-error mb-4">
          <svg xmlns="http://www.w3.org/2000/svg" class="stroke-current shrink-0 h-6 w-6" fill="none" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z" /></svg>
          <span>{error}</span>
        </div>
      {/if}

      <div class="form-control">
        <label class="label" for="name">
          <span class="label-text">Workspace Name</span>
        </label>
        <input
          type="text"
          id="name"
          class="input input-bordered"
          bind:value={name}
          placeholder="My Workspace"
          required
        />
      </div>
      
      <div class="form-control mt-4">
        <label class="label" for="description">
          <span class="label-text">Description</span>
        </label>
        <textarea
          id="description"
          class="textarea textarea-bordered"
          bind:value={description}
          placeholder="Optional description"
        ></textarea>
      </div>

      <div class="modal-action">
        <button 
          type="button" 
          class="btn" 
          on:click={handleClose}
          disabled={isSubmitting}
        >
          Cancel
        </button>
        <button 
          type="submit" 
          class="btn btn-primary" 
          disabled={!name || isSubmitting}
        >
          {#if isSubmitting}
            <span class="loading loading-spinner loading-sm"></span>
          {/if}
          {workspace ? 'Save' : 'Create'}
        </button>
      </div>
    </form>
  </div>
  
  <button 
    type="button"
    class="modal-backdrop" 
    on:click={handleClose}
    aria-label="Close modal"
  ></button>
</div> 