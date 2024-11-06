import { writable, derived } from 'svelte/store';
import { invoke } from '@tauri-apps/api/core';
import type { Workspace } from '../../types';

function createWorkspaceStore() {
    const { subscribe, set, update } = writable<Workspace[]>([]);

    return {
        subscribe,
        loadWorkspaces: async () => {
            try {
                const workspaces = await invoke<Workspace[]>('get_workspaces');
                set(workspaces);
            } catch (error) {
                console.error('Error loading workspaces:', error);
                throw error;
            }
        },
        getWorkspace: async (id: string) => {
            try {
                const workspace = await invoke<Workspace>('get_workspace', { id });
                return workspace;
            } catch (error) {
                console.error('Error getting workspace:', error);
                throw error;
            }
        },
        createWorkspace: async (name: string, description?: string) => {
            try {
                const workspace = await invoke<Workspace>('create_workspace', {
                    name,
                    description,
                });
                update(workspaces => [...workspaces, workspace]);
                return workspace;
            } catch (error) {
                console.error('Error creating workspace:', error);
                throw new Error(error instanceof Error ? error.message : 'Failed to create workspace');
            }
        },
        updateWorkspace: async (id: string, name: string, description?: string) => {
            try {
                const workspace = await invoke<Workspace>('update_workspace', {
                    id,
                    name,
                    description,
                });
                update(workspaces => 
                    workspaces.map(w => w.id === id ? workspace : w)
                );
                return workspace;
            } catch (error) {
                console.error('Error updating workspace:', error);
                throw new Error(error instanceof Error ? error.message : 'Failed to update workspace');
            }
        },
        deleteWorkspace: async (id: string) => {
            try {
                await invoke('delete_workspace', { id });
                update(workspaces => workspaces.filter(w => w.id !== id));
            } catch (error) {
                console.error('Error deleting workspace:', error);
                throw new Error(error instanceof Error ? error.message : 'Failed to delete workspace');
            }
        },
    };
}

export const workspaces = createWorkspaceStore();
export const activeWorkspace = writable<Workspace | null>(null);

// Derived store for the current workspace's variables
export const currentVariables = derived(
    activeWorkspace,
    $workspace => $workspace?.variables || []
); 