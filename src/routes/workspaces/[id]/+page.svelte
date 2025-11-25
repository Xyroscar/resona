<script lang="ts">
  import { goto } from "$app/navigation";
  import { onMount } from "svelte";
  import * as Sidebar from "$lib/components/ui/sidebar/index.js";
  import * as Dialog from "$lib/components/ui/dialog/index.js";
  import { Button } from "$lib/components/ui/button/index.js";
  import { Input } from "$lib/components/ui/input/index.js";
  import { Label } from "$lib/components/ui/label/index.js";
  import WorkspaceSidebar from "$lib/components/workspace-sidebar.svelte";
  import RequestPanel from "$lib/components/request-panel.svelte";
  import * as Empty from "$lib/components/ui/empty/index.js";
  import SendIcon from "@lucide/svelte/icons/send";
  import { get_workspace } from "$lib/services/workspaces";
  import {
    get_collections_by_workspace,
    get_standalone_requests_by_workspace,
    create_collection,
    update_collection,
    delete_collection,
    create_request,
    update_request,
    delete_request,
  } from "$lib/services/collections";
  import { get_resolved_variables } from "$lib/services/variables";
  import type { Workspace } from "$lib/types/workspace";
  import type { Collection } from "$lib/types/collection";
  import type { Request, HttpMethod } from "$lib/types/request";
  import type { Response } from "$lib/types/response";
  import type { ResolvedVariable } from "$lib/types/variable";
  import ResponsePanel from "$lib/components/response-panel.svelte";

  const { params } = $props<{ params: { id: string } }>();

  let workspace = $state<Workspace | null>(null);
  let collections = $state<Collection[]>([]);
  let standaloneRequests = $state<Request[]>([]);
  let selectedRequest = $state<Request | null>(null);
  let response = $state<Response | null>(null);
  let loading = $state(false);
  let resolvedVariables = $state<ResolvedVariable[]>([]);

  let collectionDialogOpen = $state(false);
  let collectionDialogMode = $state<"create" | "edit">("create");
  let editingCollection = $state<Collection | null>(null);
  let collectionName = $state("");
  let collectionDescription = $state("");

  let requestDialogOpen = $state(false);
  let requestDialogMode = $state<"create" | "edit">("create");
  let editingRequest = $state<Request | null>(null);
  let requestName = $state("");
  let requestMethod = $state<HttpMethod>("GET");
  let requestCollectionId = $state<string | null>(null);

  let deleteDialogOpen = $state(false);
  let deleteTarget = $state<{
    type: "collection" | "request";
    item: Collection | Request;
  } | null>(null);

  onMount(async () => {
    await loadData();
  });

  async function loadData() {
    workspace = await get_workspace(params.id);
    collections = await get_collections_by_workspace(params.id);
    standaloneRequests = await get_standalone_requests_by_workspace(params.id);
    resolvedVariables = await get_resolved_variables(params.id, null, null);
  }

  function goBack() {
    goto("/workspaces");
  }

  function handleRequestSelect(request: Request) {
    selectedRequest = request;
    response = null;
  }

  function handleCreateCollection() {
    collectionDialogMode = "create";
    editingCollection = null;
    collectionName = "";
    collectionDescription = "";
    collectionDialogOpen = true;
  }

  function handleEditCollection(collection: Collection) {
    collectionDialogMode = "edit";
    editingCollection = collection;
    collectionName = collection.name;
    collectionDescription = collection.description;
    collectionDialogOpen = true;
  }

  function handleDeleteCollection(collection: Collection) {
    deleteTarget = { type: "collection", item: collection };
    deleteDialogOpen = true;
  }

  function handleCreateRequest(collectionId: string | null) {
    requestDialogMode = "create";
    editingRequest = null;
    requestName = "";
    requestMethod = "GET";
    requestCollectionId = collectionId;
    requestDialogOpen = true;
  }

  function handleEditRequest(request: Request) {
    requestDialogMode = "edit";
    editingRequest = request;
    requestName = request.name;
    requestMethod = request.method;
    requestCollectionId = request.collectionId;
    requestDialogOpen = true;
  }

  function handleDeleteRequest(request: Request) {
    deleteTarget = { type: "request", item: request };
    deleteDialogOpen = true;
  }

  async function handleCollectionSubmit() {
    if (collectionDialogMode === "create") {
      await create_collection({
        name: collectionName,
        description: collectionDescription,
        workspaceId: params.id,
      });
    } else if (editingCollection) {
      await update_collection(editingCollection.id, {
        name: collectionName,
        description: collectionDescription,
      });
    }
    await loadData();
    collectionDialogOpen = false;
  }

  async function handleRequestSubmit() {
    if (requestDialogMode === "create") {
      await create_request({
        name: requestName,
        method: requestMethod,
        url: "",
        headers: [],
        params: [],
        bodyType: "none",
        body: "",
        formData: [],
        collectionId: requestCollectionId,
        workspaceId: params.id,
      });
    } else if (editingRequest) {
      await update_request(editingRequest.id, {
        name: requestName,
        method: requestMethod,
      });
    }
    await loadData();
    requestDialogOpen = false;
  }

  async function handleDeleteConfirm() {
    if (!deleteTarget) return;

    if (deleteTarget.type === "collection") {
      await delete_collection((deleteTarget.item as Collection).id);
    } else {
      const request = deleteTarget.item as Request;
      await delete_request(request.id);
      if (selectedRequest?.id === request.id) {
        selectedRequest = null;
      }
    }
    await loadData();
    deleteDialogOpen = false;
    deleteTarget = null;
  }

  async function handleSendRequest(request: Request) {
    loading = true;
    response = null;

    // Simulate API request (will be replaced with actual Tauri call later)
    const startTime = Date.now();

    try {
      // Mock response for now
      await new Promise((resolve) =>
        setTimeout(resolve, 500 + Math.random() * 1000)
      );

      const mockBody = JSON.stringify(
        {
          success: true,
          message: "This is a mock response",
          data: {
            id: 1,
            name: "Example",
            timestamp: new Date().toISOString(),
          },
        },
        null,
        2
      );

      response = {
        status: 200,
        statusText: "OK",
        headers: [
          { key: "Content-Type", value: "application/json" },
          { key: "X-Request-Id", value: crypto.randomUUID() },
          { key: "Cache-Control", value: "no-cache" },
        ],
        body: mockBody,
        contentType: "application/json",
        duration: Date.now() - startTime,
        size: new Blob([mockBody]).size,
      };
    } catch (error) {
      response = {
        status: 500,
        statusText: "Error",
        headers: [],
        body: JSON.stringify({ error: "Request failed" }),
        contentType: "application/json",
        duration: Date.now() - startTime,
        size: 0,
      };
    } finally {
      loading = false;
    }
  }

  async function handleUpdateRequest(request: Request) {
    await update_request(request.id, request);
    await loadData();
  }

  const methods: HttpMethod[] = [
    "GET",
    "POST",
    "PUT",
    "PATCH",
    "DELETE",
    "HEAD",
    "OPTIONS",
  ];
</script>

<Sidebar.Provider>
  <WorkspaceSidebar
    workspaceName={workspace?.Name ?? "Loading..."}
    {collections}
    {standaloneRequests}
    selectedRequestId={selectedRequest?.id ?? null}
    onRequestSelect={handleRequestSelect}
    onCreateCollection={handleCreateCollection}
    onCreateRequest={handleCreateRequest}
    onEditCollection={handleEditCollection}
    onDeleteCollection={handleDeleteCollection}
    onEditRequest={handleEditRequest}
    onDeleteRequest={handleDeleteRequest}
    onBack={goBack}
  />

  <Sidebar.Inset>
    <main class="flex flex-col h-screen">
      <header class="flex h-14 shrink-0 items-center gap-2 border-b px-4">
        <Sidebar.Trigger class="-ml-1" />
        <div class="flex-1">
          {#if selectedRequest}
            <h1 class="text-sm font-medium">{selectedRequest.name}</h1>
          {:else}
            <h1 class="text-sm font-medium text-muted-foreground">
              Select a request
            </h1>
          {/if}
        </div>
      </header>

      <div class="flex-1 overflow-hidden flex flex-col">
        {#if selectedRequest}
          <div class="flex-1 min-h-0 flex flex-col">
            <div class="flex-1 min-h-0 overflow-hidden">
              <RequestPanel
                request={selectedRequest}
                variables={resolvedVariables}
                onSend={handleSendRequest}
                onUpdate={handleUpdateRequest}
              />
            </div>
            <div class="h-[300px] min-h-[200px] border-t">
              <ResponsePanel {response} {loading} />
            </div>
          </div>
        {:else}
          <Empty.Root class="flex h-full items-center justify-center">
            <Empty.Header>
              <Empty.Media variant="icon">
                <SendIcon />
              </Empty.Media>
              <Empty.Title>No Request Selected</Empty.Title>
              <Empty.Description>
                Select a request from the sidebar or create a new one to get
                started.
              </Empty.Description>
            </Empty.Header>
          </Empty.Root>
        {/if}
      </div>
    </main>
  </Sidebar.Inset>
</Sidebar.Provider>

<Dialog.Root bind:open={collectionDialogOpen}>
  <Dialog.Content class="sm:max-w-[425px]">
    <Dialog.Header>
      <Dialog.Title>
        {collectionDialogMode === "create"
          ? "Create Collection"
          : "Edit Collection"}
      </Dialog.Title>
      <Dialog.Description>
        {collectionDialogMode === "create"
          ? "Create a new collection to organize your requests."
          : "Update your collection details."}
      </Dialog.Description>
    </Dialog.Header>
    <form class="grid gap-4 py-4" onsubmit={handleCollectionSubmit}>
      <div class="grid grid-cols-4 items-center gap-4">
        <Label for="collection-name" class="text-end">Name</Label>
        <Input
          id="collection-name"
          class="col-span-3"
          bind:value={collectionName}
        />
      </div>
      <div class="grid grid-cols-4 items-center gap-4">
        <Label for="collection-description" class="text-end">Description</Label>
        <Input
          id="collection-description"
          class="col-span-3"
          bind:value={collectionDescription}
        />
      </div>
      <Dialog.Footer>
        <Button type="submit">
          {collectionDialogMode === "create" ? "Create" : "Save"}
        </Button>
      </Dialog.Footer>
    </form>
  </Dialog.Content>
</Dialog.Root>

<Dialog.Root bind:open={requestDialogOpen}>
  <Dialog.Content class="sm:max-w-[425px]">
    <Dialog.Header>
      <Dialog.Title>
        {requestDialogMode === "create" ? "Create Request" : "Edit Request"}
      </Dialog.Title>
      <Dialog.Description>
        {requestDialogMode === "create"
          ? "Create a new API request."
          : "Update your request details."}
      </Dialog.Description>
    </Dialog.Header>
    <form class="grid gap-4 py-4" onsubmit={handleRequestSubmit}>
      <div class="grid grid-cols-4 items-center gap-4">
        <Label for="request-name" class="text-end">Name</Label>
        <Input id="request-name" class="col-span-3" bind:value={requestName} />
      </div>
      <div class="grid grid-cols-4 items-center gap-4">
        <Label for="request-method" class="text-end">Method</Label>
        <select
          id="request-method"
          class="col-span-3 flex h-9 w-full rounded-md border border-input bg-transparent px-3 py-1 text-sm shadow-sm transition-colors focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring"
          bind:value={requestMethod}
        >
          {#each methods as method (method)}
            <option value={method}>{method}</option>
          {/each}
        </select>
      </div>
      <Dialog.Footer>
        <Button type="submit">
          {requestDialogMode === "create" ? "Create" : "Save"}
        </Button>
      </Dialog.Footer>
    </form>
  </Dialog.Content>
</Dialog.Root>

<Dialog.Root bind:open={deleteDialogOpen}>
  <Dialog.Content class="sm:max-w-[425px]">
    <Dialog.Header>
      <Dialog.Title>Confirm Delete</Dialog.Title>
      <Dialog.Description>
        Are you sure you want to delete this {deleteTarget?.type}? This action
        cannot be undone.
      </Dialog.Description>
    </Dialog.Header>
    <Dialog.Footer>
      <Button variant="outline" onclick={() => (deleteDialogOpen = false)}
        >Cancel</Button
      >
      <Button variant="destructive" onclick={handleDeleteConfirm}>Delete</Button
      >
    </Dialog.Footer>
  </Dialog.Content>
</Dialog.Root>
