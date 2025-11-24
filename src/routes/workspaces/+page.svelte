<script lang="ts">
  import * as Empty from "$lib/components/ui/empty/index.js";
  import { Button } from "$lib/components/ui/button/index.js";
  import FolderCodeIcon from "@lucide/svelte/icons/folder-code";
  import { get_workspaces } from "$lib/services/workspaces";

  get_workspaces;
</script>

<Empty.Root class="flex min-h-[calc(100vh-4rem)] items-center justify-center">
  <Empty.Header>
    <Empty.Media variant="icon">
      <FolderCodeIcon />
    </Empty.Media>
    <Empty.Title>No Workspaces Yet</Empty.Title>
    <Empty.Description>
      You haven't created any Workspaces yet. Get started by creating your first
      project.
    </Empty.Description>
  </Empty.Header>
  <Empty.Content>
    <div class="flex gap-2">
      <Button>Create Workspace</Button>
    </div>
  </Empty.Content>
</Empty.Root>
