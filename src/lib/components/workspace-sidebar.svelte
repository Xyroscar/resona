<script lang="ts">
  import * as Collapsible from "$lib/components/ui/collapsible/index.js";
  import * as Sidebar from "$lib/components/ui/sidebar/index.js";
  import * as DropdownMenu from "$lib/components/ui/dropdown-menu/index.js";
  import ChevronRightIcon from "@lucide/svelte/icons/chevron-right";
  import FolderIcon from "@lucide/svelte/icons/folder";
  import FolderOpenIcon from "@lucide/svelte/icons/folder-open";
  import PlusIcon from "@lucide/svelte/icons/plus";
  import MoreHorizontalIcon from "@lucide/svelte/icons/more-horizontal";
  import FileIcon from "@lucide/svelte/icons/file";
  import ArrowLeftIcon from "@lucide/svelte/icons/arrow-left";
  import type { ComponentProps } from "svelte";
  import type { Collection } from "$lib/types/collection";
  import type { Request } from "$lib/types/request";

  type Props = ComponentProps<typeof Sidebar.Root> & {
    workspaceName: string;
    collections: Collection[];
    standaloneRequests: Request[];
    selectedRequestId: string | null;
    onRequestSelect: (request: Request) => void;
    onCreateCollection: () => void;
    onCreateRequest: (collectionId: string | null) => void;
    onEditCollection: (collection: Collection) => void;
    onDeleteCollection: (collection: Collection) => void;
    onEditRequest: (request: Request) => void;
    onDeleteRequest: (request: Request) => void;
    onBack: () => void;
  };

  let {
    ref = $bindable(null),
    workspaceName,
    collections,
    standaloneRequests,
    selectedRequestId,
    onRequestSelect,
    onCreateCollection,
    onCreateRequest,
    onEditCollection,
    onDeleteCollection,
    onEditRequest,
    onDeleteRequest,
    onBack,
    ...restProps
  }: Props = $props();

  function getMethodColor(method: string): string {
    const colors: Record<string, string> = {
      GET: "text-green-500",
      POST: "text-blue-500",
      PUT: "text-orange-500",
      PATCH: "text-yellow-500",
      DELETE: "text-red-500",
      HEAD: "text-purple-500",
      OPTIONS: "text-gray-500",
    };
    return colors[method] || "text-gray-500";
  }
</script>

<Sidebar.Root bind:ref {...restProps}>
  <Sidebar.Header>
    <Sidebar.Menu>
      <Sidebar.MenuItem>
        <Sidebar.MenuButton size="lg" onclick={onBack}>
          <div
            class="bg-sidebar-primary text-sidebar-primary-foreground flex aspect-square size-8 items-center justify-center rounded-lg"
          >
            <ArrowLeftIcon class="size-4" />
          </div>
          <div class="flex flex-col gap-0.5 leading-none">
            <span class="font-medium truncate">{workspaceName}</span>
            <span class="text-xs text-muted-foreground">Back to workspaces</span
            >
          </div>
        </Sidebar.MenuButton>
      </Sidebar.MenuItem>
    </Sidebar.Menu>
  </Sidebar.Header>

  <Sidebar.Content class="gap-0">
    <Sidebar.Group>
      <Sidebar.GroupLabel class="flex items-center justify-between pr-2">
        <span>Collections</span>
        <button
          class="hover:bg-sidebar-accent rounded p-1"
          onclick={onCreateCollection}
        >
          <PlusIcon class="size-4" />
        </button>
      </Sidebar.GroupLabel>
      <Sidebar.GroupContent>
        <Sidebar.Menu>
          {#each collections as collection (collection.id)}
            <Collapsible.Root open class="group/collapsible">
              <Sidebar.MenuItem>
                <Collapsible.Trigger class="w-full">
                  {#snippet child({ props })}
                    <Sidebar.MenuButton {...props}>
                      <FolderIcon
                        class="size-4 group-data-[state=closed]/collapsible:block hidden"
                      />
                      <FolderOpenIcon
                        class="size-4 group-data-[state=open]/collapsible:block hidden"
                      />
                      <span class="truncate">{collection.name}</span>
                      <ChevronRightIcon
                        class="ms-auto size-4 transition-transform group-data-[state=open]/collapsible:rotate-90"
                      />
                    </Sidebar.MenuButton>
                  {/snippet}
                </Collapsible.Trigger>
                <DropdownMenu.Root>
                  <DropdownMenu.Trigger>
                    {#snippet child({ props })}
                      <Sidebar.MenuAction {...props}>
                        <MoreHorizontalIcon class="size-4" />
                      </Sidebar.MenuAction>
                    {/snippet}
                  </DropdownMenu.Trigger>
                  <DropdownMenu.Content side="right" align="start">
                    <DropdownMenu.Item
                      onclick={() => onCreateRequest(collection.id)}
                    >
                      Add Request
                    </DropdownMenu.Item>
                    <DropdownMenu.Item
                      onclick={() => onEditCollection(collection)}
                    >
                      Edit Collection
                    </DropdownMenu.Item>
                    <DropdownMenu.Separator />
                    <DropdownMenu.Item
                      class="text-destructive"
                      onclick={() => onDeleteCollection(collection)}
                    >
                      Delete Collection
                    </DropdownMenu.Item>
                  </DropdownMenu.Content>
                </DropdownMenu.Root>
              </Sidebar.MenuItem>
              <Collapsible.Content>
                <Sidebar.MenuSub>
                  {#each collection.requests as request (request.id)}
                    <Sidebar.MenuSubItem>
                      <Sidebar.MenuSubButton
                        isActive={selectedRequestId === request.id}
                        onclick={() => onRequestSelect(request)}
                      >
                        <span
                          class={`text-xs font-mono font-semibold ${getMethodColor(request.method)}`}
                        >
                          {request.method.substring(0, 3)}
                        </span>
                        <span class="truncate">{request.name}</span>
                      </Sidebar.MenuSubButton>
                      <DropdownMenu.Root>
                        <DropdownMenu.Trigger>
                          {#snippet child({ props })}
                            <Sidebar.MenuAction {...props}>
                              <MoreHorizontalIcon class="size-4" />
                            </Sidebar.MenuAction>
                          {/snippet}
                        </DropdownMenu.Trigger>
                        <DropdownMenu.Content side="right" align="start">
                          <DropdownMenu.Item
                            onclick={() => onEditRequest(request)}
                          >
                            Edit Request
                          </DropdownMenu.Item>
                          <DropdownMenu.Separator />
                          <DropdownMenu.Item
                            class="text-destructive"
                            onclick={() => onDeleteRequest(request)}
                          >
                            Delete Request
                          </DropdownMenu.Item>
                        </DropdownMenu.Content>
                      </DropdownMenu.Root>
                    </Sidebar.MenuSubItem>
                  {/each}
                </Sidebar.MenuSub>
              </Collapsible.Content>
            </Collapsible.Root>
          {/each}
        </Sidebar.Menu>
      </Sidebar.GroupContent>
    </Sidebar.Group>

    <Sidebar.Separator />

    <Sidebar.Group>
      <Sidebar.GroupLabel class="flex items-center justify-between pr-2">
        <span>Requests</span>
        <button
          class="hover:bg-sidebar-accent rounded p-1"
          onclick={() => onCreateRequest(null)}
        >
          <PlusIcon class="size-4" />
        </button>
      </Sidebar.GroupLabel>
      <Sidebar.GroupContent>
        <Sidebar.Menu>
          {#each standaloneRequests as request (request.id)}
            <Sidebar.MenuItem>
              <Sidebar.MenuButton
                isActive={selectedRequestId === request.id}
                onclick={() => onRequestSelect(request)}
              >
                <span
                  class={`text-xs font-mono font-semibold ${getMethodColor(request.method)}`}
                >
                  {request.method.substring(0, 3)}
                </span>
                <span class="truncate">{request.name}</span>
              </Sidebar.MenuButton>
              <DropdownMenu.Root>
                <DropdownMenu.Trigger>
                  {#snippet child({ props })}
                    <Sidebar.MenuAction {...props}>
                      <MoreHorizontalIcon class="size-4" />
                    </Sidebar.MenuAction>
                  {/snippet}
                </DropdownMenu.Trigger>
                <DropdownMenu.Content side="right" align="start">
                  <DropdownMenu.Item onclick={() => onEditRequest(request)}>
                    Edit Request
                  </DropdownMenu.Item>
                  <DropdownMenu.Separator />
                  <DropdownMenu.Item
                    class="text-destructive"
                    onclick={() => onDeleteRequest(request)}
                  >
                    Delete Request
                  </DropdownMenu.Item>
                </DropdownMenu.Content>
              </DropdownMenu.Root>
            </Sidebar.MenuItem>
          {/each}
        </Sidebar.Menu>
      </Sidebar.GroupContent>
    </Sidebar.Group>
  </Sidebar.Content>

  <Sidebar.Rail />
</Sidebar.Root>
