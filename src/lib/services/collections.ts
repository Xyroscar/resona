import { invoke } from "@tauri-apps/api/core";
import type { Collection } from "$lib/types/collection";
import type { Request } from "$lib/types/request";

// Collection types for Rust backend
type RustCollection = {
  id: string;
  name: string;
  description: string;
  workspace_id: string;
  created_at: string;
  updated_at: string;
};

type CreateCollectionInput = {
  name: string;
  description: string;
  workspace_id: string;
};

type UpdateCollectionInput = {
  id: string;
  name?: string;
  description?: string;
};

// Request types for Rust backend
type RustRequest = {
  id: string;
  name: string;
  method: string;
  url: string;
  headers: { key: string; value: string; enabled: boolean }[];
  params: { key: string; value: string; enabled: boolean }[];
  body_type: string;
  body: string;
  form_data: {
    key: string;
    value: string;
    item_type: string;
    enabled: boolean;
  }[];
  collection_id: string | null;
  workspace_id: string;
  created_at: string;
  updated_at: string;
};

type CreateRequestInput = {
  name: string;
  method: string;
  url: string;
  headers: { key: string; value: string; enabled: boolean }[];
  params: { key: string; value: string; enabled: boolean }[];
  body_type: string;
  body: string;
  form_data: {
    key: string;
    value: string;
    item_type: string;
    enabled: boolean;
  }[];
  collection_id: string | null;
  workspace_id: string;
};

type UpdateRequestInput = {
  id: string;
  name?: string;
  method?: string;
  url?: string;
  headers?: { key: string; value: string; enabled: boolean }[];
  params?: { key: string; value: string; enabled: boolean }[];
  body_type?: string;
  body?: string;
  form_data?: {
    key: string;
    value: string;
    item_type: string;
    enabled: boolean;
  }[];
  collection_id?: string | null;
};

// Convert Rust collection to frontend Collection
function toCollection(
  rust: RustCollection,
  requests: Request[] = []
): Collection {
  return {
    id: rust.id,
    name: rust.name,
    description: rust.description,
    workspaceId: rust.workspace_id,
    requests,
  };
}

// Convert Rust request to frontend Request
function toRequest(rust: RustRequest): Request {
  return {
    id: rust.id,
    name: rust.name,
    method: rust.method as Request["method"],
    url: rust.url,
    headers: rust.headers,
    params: rust.params,
    bodyType: rust.body_type as Request["bodyType"],
    body: rust.body,
    formData: rust.form_data.map((f) => ({
      key: f.key,
      value: f.value,
      type: f.item_type as "text" | "file",
      enabled: f.enabled,
    })),
    collectionId: rust.collection_id,
    workspaceId: rust.workspace_id,
  };
}

export async function get_collections_by_workspace(
  workspaceId: string
): Promise<Collection[]> {
  const rustCollections = await invoke<RustCollection[]>(
    "get_collections_by_workspace",
    { workspaceId }
  );

  const collections: Collection[] = [];
  for (const rc of rustCollections) {
    const rustRequests = await invoke<RustRequest[]>(
      "get_requests_by_collection",
      {
        collectionId: rc.id,
      }
    );
    collections.push(toCollection(rc, rustRequests.map(toRequest)));
  }

  return collections;
}

export async function get_collection(
  id: string
): Promise<Collection | undefined> {
  try {
    const rc = await invoke<RustCollection>("get_collection", { id });
    const rustRequests = await invoke<RustRequest[]>(
      "get_requests_by_collection",
      {
        collectionId: id,
      }
    );
    return toCollection(rc, rustRequests.map(toRequest));
  } catch {
    return undefined;
  }
}

export async function create_collection(collection: {
  name: string;
  description: string;
  workspaceId: string;
}): Promise<Collection> {
  const input: CreateCollectionInput = {
    name: collection.name,
    description: collection.description,
    workspace_id: collection.workspaceId,
  };
  const rc = await invoke<RustCollection>("create_collection", { input });
  return toCollection(rc, []);
}

export async function update_collection(
  id: string,
  updates: Partial<Pick<Collection, "name" | "description">>
): Promise<boolean> {
  try {
    const input: UpdateCollectionInput = {
      id,
      name: updates.name,
      description: updates.description,
    };
    await invoke<RustCollection>("update_collection", { input });
    return true;
  } catch {
    return false;
  }
}

export async function delete_collection(id: string): Promise<boolean> {
  try {
    await invoke<void>("delete_collection", { id });
    return true;
  } catch {
    return false;
  }
}

export async function get_standalone_requests_by_workspace(
  workspaceId: string
): Promise<Request[]> {
  const rustRequests = await invoke<RustRequest[]>(
    "get_standalone_requests_by_workspace",
    { workspaceId }
  );
  return rustRequests.map(toRequest);
}

export async function get_request(id: string): Promise<Request | undefined> {
  try {
    const rr = await invoke<RustRequest>("get_request", { id });
    return toRequest(rr);
  } catch {
    return undefined;
  }
}

export async function create_request(
  request: Omit<Request, "id">
): Promise<Request> {
  const input: CreateRequestInput = {
    name: request.name,
    method: request.method,
    url: request.url,
    headers: request.headers,
    params: request.params,
    body_type: request.bodyType,
    body: request.body,
    form_data: request.formData.map((f) => ({
      key: f.key,
      value: f.value,
      item_type: f.type,
      enabled: f.enabled,
    })),
    collection_id: request.collectionId,
    workspace_id: request.workspaceId,
  };
  const rr = await invoke<RustRequest>("create_request", { input });
  return toRequest(rr);
}

export async function update_request(
  id: string,
  updates: Partial<Omit<Request, "id">>
): Promise<boolean> {
  try {
    const input: UpdateRequestInput = {
      id,
      name: updates.name,
      method: updates.method,
      url: updates.url,
      headers: updates.headers,
      params: updates.params,
      body_type: updates.bodyType,
      body: updates.body,
      form_data: updates.formData?.map((f) => ({
        key: f.key,
        value: f.value,
        item_type: f.type,
        enabled: f.enabled,
      })),
      collection_id: updates.collectionId,
    };
    await invoke<RustRequest>("update_request", { input });
    return true;
  } catch {
    return false;
  }
}

export async function delete_request(id: string): Promise<boolean> {
  try {
    await invoke<void>("delete_request", { id });
    return true;
  } catch {
    return false;
  }
}
