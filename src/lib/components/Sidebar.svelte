<script lang="ts">
  import { collections, variables, activeCollection, activeRequest } from '../../stores';
  import type { Collection, Request } from '../../types';
  import CollectionModal from './modals/CollectionModal.svelte';

  let showCollectionModal = false;

  function selectRequest(collection: Collection, request: Request) {
    activeCollection.set(collection);
    activeRequest.set(request);
  }

  function handleCollectionSave(event: CustomEvent<Collection>) {
    collections.update(cols => [...cols, event.detail]);
    showCollectionModal = false;
  }
</script>

<div class="bg-base-200 min-h-screen h-full p-4 border-r border-base-300">

  <!-- Collections Section -->
  <div class="flex flex-col">
    <div class="flex justify-between items-center mb-4">
      <h2 class="font-medium flex items-center">
        <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4 mr-2" viewBox="0 0 20 20" fill="currentColor">
          <path d="M7 3a1 1 0 000 2h6a1 1 0 100-2H7zM4 7a1 1 0 011-1h10a1 1 0 110 2H5a1 1 0 01-1-1zM2 11a2 2 0 012-2h12a2 2 0 012 2v4a2 2 0 01-2 2H4a2 2 0 01-2-2v-4z" />
        </svg>
        Collections
      </h2>
      <button class="btn btn-sm btn-primary" on:click={() => showCollectionModal = true}>
        <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4 mr-1" viewBox="0 0 20 20" fill="currentColor">
          <path fill-rule="evenodd" d="M10 3a1 1 0 011 1v5h5a1 1 0 110 2h-5v5a1 1 0 11-2 0v-5H4a1 1 0 110-2h5V4a1 1 0 011-1z" clip-rule="evenodd" />
        </svg>
        New
      </button>
    </div>
    
    {#if $collections.length === 0}
      <div class="bg-base-100 rounded-lg p-4 text-center">
        <p class="text-sm text-base-content/70">No collections yet</p>
        <p class="text-xs text-base-content/50 mt-1">Create a new collection to get started</p>
      </div>
    {/if}

    {#each $collections as collection}
      <div class="collapse collapse-arrow bg-base-100 shadow-sm mb-2">
        <input type="checkbox" />
        <div class="collapse-title font-medium">
          {collection.name}
        </div>
        <div class="collapse-content">
          <div class="flex flex-col gap-1">
            {#each collection.requests as request}
              <button
                class="flex items-center w-full px-3 py-2 hover:bg-base-300 rounded-lg text-left transition-colors"
                class:bg-primary-content={$activeRequest?.id === request.id}
                on:click={() => selectRequest(collection, request)}
              >
                <span class="text-xs font-medium px-2 py-0.5 rounded mr-2" 
                      class:bg-success={request.method === 'GET'}
                      class:bg-info={request.method === 'POST'}
                      class:bg-warning={request.method === 'PUT'}
                      class:bg-error={request.method === 'DELETE'}
                >
                  {request.method}
                </span>
                <span class="text-sm truncate">{request.name}</span>
              </button>
            {/each}
            <button class="btn btn-sm btn-ghost mt-2 w-full">
              <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4 mr-2" viewBox="0 0 20 20" fill="currentColor">
                <path fill-rule="evenodd" d="M10 3a1 1 0 011 1v5h5a1 1 0 110 2h-5v5a1 1 0 11-2 0v-5H4a1 1 0 110-2h5V4a1 1 0 011-1z" clip-rule="evenodd" />
              </svg>
              Add Request
            </button>
          </div>
        </div>
      </div>
    {/each}
  </div>
</div>

<CollectionModal
  show={showCollectionModal}
  on:save={handleCollectionSave}
  on:close={() => showCollectionModal = false}
/> 