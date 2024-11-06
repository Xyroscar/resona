import { writable, derived } from 'svelte/store';
import { invoke } from '@tauri-apps/api/core';
import type { Collection } from '../../types';
import { activeWorkspace } from './workspace';

function createCollectionStore() {
    const { subscribe, set, update } = writable<Collection[]>([]);

    return {
        subscribe,
        loadCollections: async (workspaceId: string) => {
            try {
                const collections = await invoke<Collection[]>('get_workspace_collections', {
                    workspaceId
                });
                set(collections);
            } catch (error) {
                console.error('Error loading collections:', error);
                set([]);
            }
        },
        createCollection: async (workspaceId: string, name: string, description?: string) => {
            try {
                const collection = await invoke<Collection>('create_collection', {
                    workspaceId,
                    name,
                    description,
                });
                update(collections => [...collections, collection]);
                return collection;
            } catch (error) {
                console.error('Error creating collection:', error);
                throw error;
            }
        },
        updateCollection: async (id: string, name: string, description?: string) => {
            try {
                const collection = await invoke<Collection>('update_collection', {
                    id,
                    name,
                    description,
                });
                update(collections => 
                    collections.map(c => c.id === id ? collection : c)
                );
                return collection;
            } catch (error) {
                console.error('Error updating collection:', error);
                throw error;
            }
        },
        deleteCollection: async (id: string) => {
            try {
                await invoke('delete_collection', { id });
                update(collections => collections.filter(c => c.id !== id));
            } catch (error) {
                console.error('Error deleting collection:', error);
                throw error;
            }
        },
    };
}

export const collections = createCollectionStore();
export const activeCollection = writable<Collection | null>(null);

// Auto-load collections when workspace changes
activeWorkspace.subscribe(workspace => {
    if (workspace) {
        collections.loadCollections(workspace.id);
    }
}); 