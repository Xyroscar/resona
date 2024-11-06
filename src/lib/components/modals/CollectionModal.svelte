<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import type { Collection } from '../../../types';
  import { collections } from '../../stores/collection';
  import { activeWorkspace } from '../../stores/workspace';

  export let show = false;
  export let collection: Collection | null = null;
  
  let name = '';
  let description = '';

  $: if (collection) {
    name = collection.name;
    description = collection.description || '';
  }

  const dispatch = createEventDispatcher<{
    close: void;
  }>();

  async function handleSubmit() {
    try {
      if (!$activeWorkspace) return;

      if (collection) {
        await collections.updateCollection(
          collection.id,
          name,
          description || undefined
        );
      } else {
        await collections.createCollection(
          $activeWorkspace.id,
          name,
          description || undefined
        );
      }
      
      handleClose();
    } catch (error) {
      console.error('Failed to save collection:', error);
    }
  }

  function handleClose() {
    name = '';
    description = '';
    dispatch('close');
  }
</script>

{#if show}
  <div class="modal modal-open">
    <div class="modal-box">
      <h3 class="font-bold text-lg">
        {collection ? 'Edit' : 'Create New'} Collection
      </h3>
      <form on:submit|preventDefault={handleSubmit} class="mt-4">
        <div class="form-control">
          <label class="label" for="name">
            <span class="label-text">Collection Name</span>
          </label>
          <input
            type="text"
            id="name"
            class="input input-bordered"
            bind:value={name}
            placeholder="My Collection"
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
          <button type="button" class="btn" on:click={handleClose}>
            Cancel
          </button>
          <button type="submit" class="btn btn-primary" disabled={!name}>
            {collection ? 'Save' : 'Create'}
          </button>
        </div>
      </form>
    </div>
  </div>
{/if} 