<script lang="ts">
  import { page } from '$app/stores';
  import { onMount } from 'svelte';
  import { workspaces, activeWorkspace } from '$lib/stores/workspace';
  import RequestPanel from '$lib/components/RequestPanel.svelte';
  import ResponsePanel from '$lib/components/ResponsePanel.svelte';

  onMount(async () => {
    try {
      const workspace = await workspaces.getWorkspace($page.params.id);
      if (workspace) {
        activeWorkspace.set(workspace);
      }
    } catch (error) {
      console.error('Failed to load workspace:', error);
    }
  });
</script>

<div class="p-4 space-y-4">
  <RequestPanel />
  <ResponsePanel />
</div> 