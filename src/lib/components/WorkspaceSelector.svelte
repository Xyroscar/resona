<script lang="ts">
    import { workspaces, activeWorkspace } from '../stores/workspace';
    import { onMount } from 'svelte';

    let showCreateModal = false;
    let newWorkspaceName = '';
    let newWorkspaceDescription = '';
    let showDropdown = false;

    onMount(() => {
        workspaces.loadWorkspaces();
    });

    async function handleCreateWorkspace() {
        try {
            const workspace = await workspaces.createWorkspace(
                newWorkspaceName,
                newWorkspaceDescription || undefined
            );
            activeWorkspace.set(workspace);
            showCreateModal = false;
            newWorkspaceName = '';
            newWorkspaceDescription = '';
        } catch (error) {
            console.error('Failed to create workspace:', error);
        }
    }
</script>

<div class="dropdown">
    <button type="button" class="btn m-1" aria-haspopup="true" aria-expanded={showDropdown}>
        {#if $activeWorkspace}
            {$activeWorkspace.name}
        {:else}
            Select Workspace
        {/if}
        <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4 ml-2" viewBox="0 0 20 20" fill="currentColor">
            <path fill-rule="evenodd" d="M5.293 7.293a1 1 0 011.414 0L10 10.586l3.293-3.293a1 1 0 111.414 1.414l-4 4a1 1 0 01-1.414 0l-4-4a1 1 0 010-1.414z" clip-rule="evenodd" />
        </svg>
    </button>
    <div 
        class="dropdown-content menu p-2 shadow bg-base-100 rounded-box w-52" 
        role="menu"
    >
        {#each $workspaces as workspace}
            <li>
                <button 
                    class="btn btn-ghost justify-start"
                    class:btn-active={$activeWorkspace?.id === workspace.id}
                    on:click={() => activeWorkspace.set(workspace)}
                >
                    {workspace.name}
                </button>
            </li>
        {/each}
        <li>
            <button 
                class="btn btn-ghost justify-start text-primary"
                on:click={() => showCreateModal = true}
            >
                <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4 mr-2" viewBox="0 0 20 20" fill="currentColor">
                    <path fill-rule="evenodd" d="M10 3a1 1 0 011 1v5h5a1 1 0 110 2h-5v5a1 1 0 11-2 0v-5H4a1 1 0 110-2h5V4a1 1 0 011-1z" clip-rule="evenodd" />
                </svg>
                New Workspace
            </button>
        </li>
    </div>
</div>

{#if showCreateModal}
    <div class="modal modal-open">
        <div class="modal-box">
            <h3 class="font-bold text-lg">Create New Workspace</h3>
            <form on:submit|preventDefault={handleCreateWorkspace} class="mt-4">
                <div class="form-control">
                    <label class="label" for="name">
                        <span class="label-text">Workspace Name</span>
                    </label>
                    <input
                        type="text"
                        id="name"
                        class="input input-bordered"
                        bind:value={newWorkspaceName}
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
                        bind:value={newWorkspaceDescription}
                        placeholder="Optional description"
                    />
                </div>

                <div class="modal-action">
                    <button 
                        type="button" 
                        class="btn" 
                        on:click={() => showCreateModal = false}
                    >
                        Cancel
                    </button>
                    <button 
                        type="submit" 
                        class="btn btn-primary" 
                        disabled={!newWorkspaceName}
                    >
                        Create
                    </button>
                </div>
            </form>
        </div>
    </div>
{/if} 