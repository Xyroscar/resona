import type { Collection } from "$lib/types/collection";
import type { Request, HttpMethod } from "$lib/types/request";

const collections: Map<string, Collection> = new Map<string, Collection>([
  [
    "col-1",
    {
      id: "col-1",
      name: "User API",
      description: "User management endpoints",
      workspaceId: "1",
      requests: [
        {
          id: "req-1",
          name: "Get Users",
          method: "GET",
          url: "https://api.example.com/users",
          headers: [],
          params: [],
          bodyType: "none",
          body: "",
          formData: [],
          collectionId: "col-1",
          workspaceId: "1",
        },
        {
          id: "req-2",
          name: "Create User",
          method: "POST",
          url: "https://api.example.com/users",
          headers: [
            { key: "Content-Type", value: "application/json", enabled: true },
          ],
          params: [],
          bodyType: "json",
          body: '{"name": "John", "email": "john@example.com"}',
          formData: [],
          collectionId: "col-1",
          workspaceId: "1",
        },
      ],
    },
  ],
  [
    "col-2",
    {
      id: "col-2",
      name: "Products API",
      description: "Product catalog endpoints",
      workspaceId: "1",
      requests: [
        {
          id: "req-3",
          name: "List Products",
          method: "GET",
          url: "https://api.example.com/products",
          headers: [],
          params: [{ key: "limit", value: "10", enabled: true }],
          bodyType: "none",
          body: "",
          formData: [],
          collectionId: "col-2",
          workspaceId: "1",
        },
      ],
    },
  ],
]);

const standaloneRequests: Map<string, Request> = new Map<string, Request>([
  [
    "req-standalone-1",
    {
      id: "req-standalone-1",
      name: "Health Check",
      method: "GET",
      url: "https://api.example.com/health",
      headers: [],
      params: [],
      bodyType: "none",
      body: "",
      formData: [],
      collectionId: null,
      workspaceId: "1",
    },
  ],
]);

export async function get_collections_by_workspace(
  workspaceId: string
): Promise<Collection[]> {
  return [...collections.values()].filter((c) => c.workspaceId === workspaceId);
}

export async function get_collection(
  id: string
): Promise<Collection | undefined> {
  return collections.get(id);
}

export async function create_collection(
  collection: Omit<Collection, "id" | "requests">
): Promise<Collection> {
  const newCollection: Collection = {
    ...collection,
    id: crypto.randomUUID(),
    requests: [],
  };
  collections.set(newCollection.id, newCollection);
  return newCollection;
}

export async function update_collection(
  id: string,
  updates: Partial<Pick<Collection, "name" | "description">>
): Promise<boolean> {
  const collection = collections.get(id);
  if (!collection) return false;

  if (updates.name !== undefined) collection.name = updates.name;
  if (updates.description !== undefined)
    collection.description = updates.description;
  return true;
}

export async function delete_collection(id: string): Promise<boolean> {
  return collections.delete(id);
}

export async function get_standalone_requests_by_workspace(
  workspaceId: string
): Promise<Request[]> {
  return [...standaloneRequests.values()].filter(
    (r) => r.workspaceId === workspaceId
  );
}

export async function get_request(id: string): Promise<Request | undefined> {
  for (const collection of collections.values()) {
    const request = collection.requests.find((r) => r.id === id);
    if (request) return request;
  }
  return standaloneRequests.get(id);
}

export async function create_request(
  request: Omit<Request, "id">
): Promise<Request> {
  const newRequest: Request = {
    ...request,
    id: crypto.randomUUID(),
  };

  if (request.collectionId) {
    const collection = collections.get(request.collectionId);
    if (collection) {
      collection.requests.push(newRequest);
    }
  } else {
    standaloneRequests.set(newRequest.id, newRequest);
  }

  return newRequest;
}

export async function update_request(
  id: string,
  updates: Partial<Omit<Request, "id">>
): Promise<boolean> {
  const request = await get_request(id);
  if (!request) return false;

  Object.assign(request, updates);
  return true;
}

export async function delete_request(id: string): Promise<boolean> {
  if (standaloneRequests.delete(id)) return true;

  for (const collection of collections.values()) {
    const index = collection.requests.findIndex((r) => r.id === id);
    if (index !== -1) {
      collection.requests.splice(index, 1);
      return true;
    }
  }
  return false;
}
