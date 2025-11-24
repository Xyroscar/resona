<script lang="ts">
  import * as Empty from "$lib/components/ui/empty/index.js";
  import { Button } from "$lib/components/ui/button/index.js";
  import FolderCodeIcon from "@lucide/svelte/icons/folder-code";
  import { get_workspaces } from "$lib/services/workspaces";
  import type { Workspace } from "$lib/types/workspace";
  import * as Card from "$lib/components/ui/card/index";

  let showPrompt = false;
  let workspaces: Workspace[] = [];

  get_workspaces().then((ws) => {
    if (ws.length == 0) {
      showPrompt = true;
    } else {
      workspaces = ws;
    }
  });
</script>

{#if showPrompt}
  <Empty.Root class="flex min-h-[calc(100vh-4rem)] items-center justify-center">
    <Empty.Header>
      <Empty.Media variant="icon">
        <FolderCodeIcon />
      </Empty.Media>
      <Empty.Title>No Workspaces Yet</Empty.Title>
      <Empty.Description>
        You haven't created any Workspaces yet. Get started by creating your
        first project.
      </Empty.Description>
    </Empty.Header>
    <Empty.Content>
      <div class="flex gap-2">
        <Button>Create Workspace</Button>
      </div>
    </Empty.Content>
  </Empty.Root>
{:else}
  <div class="min-h-[calc(100vh-4rem)] p-6">
    <div
      class="grid gap-6
        grid-cols-1
        sm:grid-cols-2
        md:grid-cols-3
        xl:grid-cols-4
        2xl:grid-cols-5"
    >
      {#each workspaces as workspace (workspace.Id)}
        <Card.Root
          class="min-h-40 w-full max-w-xs mx-auto flex flex-col justify-between cursor-pointer hover:shadow-md transition-shadow"
        >
          <Card.Header>
            <Card.Title class="truncate">{workspace.Name}</Card.Title>
            <Card.Description class="text-xs text-muted-foreground">
              {workspace.Description}
            </Card.Description>
          </Card.Header>
          <Card.Footer class="flex items-center justify-center gap-2">
            <Button size="sm" class="justify-center">Open</Button>
            <Button size="sm" class="justify-center">Edit</Button>
          </Card.Footer>
        </Card.Root>
      {/each}
    </div>
  </div>
{/if}
