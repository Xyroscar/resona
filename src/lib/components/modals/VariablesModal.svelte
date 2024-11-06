<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import type { Variable } from '../../../types';
  import { activeWorkspace } from '../../stores/workspace';

  export let show = false;
  
  let variables: Variable[] = [];
  
  $: if ($activeWorkspace) {
    variables = $activeWorkspace.variables || [];
  }

  const dispatch = createEventDispatcher<{
    close: void;
  }>();

  function addVariable() {
    variables = [...variables, {
      id: crypto.randomUUID(),
      name: '',
      value: '',
      description: ''
    }];
  }

  function removeVariable(id: string) {
    variables = variables.filter(v => v.id !== id);
  }

  async function handleSave() {
    // TODO: Implement variable saving
    dispatch('close');
  }
</script>

{#if show}
  <div class="modal modal-open">
    <div class="modal-box max-w-3xl">
      <h3 class="font-bold text-lg mb-4">Workspace Variables</h3>
      
      <div class="space-y-4">
        {#each variables as variable (variable.id)}
          <div class="flex gap-2">
            <input
              type="text"
              class="input input-bordered w-1/3"
              placeholder="Variable name"
              bind:value={variable.name}
            />
            <input
              type="text"
              class="input input-bordered flex-1"
              placeholder="Value"
              bind:value={variable.value}
            />
            <button 
              class="btn btn-ghost btn-sm text-error"
              on:click={() => removeVariable(variable.id)}
            >
              ×
            </button>
          </div>
        {/each}

        <button class="btn btn-ghost btn-sm w-full" on:click={addVariable}>
          Add Variable
        </button>
      </div>

      <div class="modal-action">
        <button class="btn" on:click={() => dispatch('close')}>Cancel</button>
        <button class="btn btn-primary" on:click={handleSave}>Save Changes</button>
      </div>
    </div>
  </div>
{/if} 