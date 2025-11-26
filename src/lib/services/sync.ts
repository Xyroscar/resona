import type { Workspace, WorkspaceSyncGroup } from "$lib/types/workspace";
import type { Collection } from "$lib/types/collection";
import type { Request } from "$lib/types/request";
import type { Variable } from "$lib/types/variable";
import {
  get_collections_by_workspace,
  get_standalone_requests_by_workspace,
  create_collection,
  create_request,
} from "./collections";
import {
  get_workspace_variables,
  create_variable,
  update_variable,
  get_variables_by_scope,
} from "./variables";

const syncGroups: Map<string, WorkspaceSyncGroup> = new Map();

export async function get_sync_groups(): Promise<WorkspaceSyncGroup[]> {
  return [...syncGroups.values()];
}

export async function get_sync_group(
  id: string
): Promise<WorkspaceSyncGroup | null> {
  return syncGroups.get(id) ?? null;
}

export async function get_sync_group_for_workspace(
  workspaceId: string
): Promise<WorkspaceSyncGroup | null> {
  for (const group of syncGroups.values()) {
    if (group.workspaceIds.includes(workspaceId)) {
      return group;
    }
  }
  return null;
}

export async function create_sync_group(
  name: string,
  workspaceIds: string[],
  syncedVariableNames: string[] = [],
  syncSecrets: boolean = false
): Promise<WorkspaceSyncGroup> {
  const group: WorkspaceSyncGroup = {
    id: crypto.randomUUID(),
    name,
    workspaceIds,
    syncedVariableNames,
    syncSecrets,
  };
  syncGroups.set(group.id, group);
  return group;
}

export async function update_sync_group(
  id: string,
  updates: Partial<Omit<WorkspaceSyncGroup, "id">>
): Promise<boolean> {
  const group = syncGroups.get(id);
  if (!group) return false;
  Object.assign(group, updates);
  return true;
}

export async function delete_sync_group(id: string): Promise<boolean> {
  return syncGroups.delete(id);
}

export async function add_workspace_to_sync_group(
  groupId: string,
  workspaceId: string
): Promise<boolean> {
  const group = syncGroups.get(groupId);
  if (!group) return false;
  if (!group.workspaceIds.includes(workspaceId)) {
    group.workspaceIds.push(workspaceId);
  }
  return true;
}

export async function remove_workspace_from_sync_group(
  groupId: string,
  workspaceId: string
): Promise<boolean> {
  const group = syncGroups.get(groupId);
  if (!group) return false;
  group.workspaceIds = group.workspaceIds.filter((id) => id !== workspaceId);
  return true;
}

export type DuplicateWorkspaceOptions = {
  sourceWorkspaceId: string;
  newName: string;
  newDescription: string;
  environment?: string;
  copyVariables: boolean;
  copySecrets: boolean;
  createSyncGroup: boolean;
  syncGroupName?: string;
  variablesToSync?: string[];
};

export type DuplicateWorkspaceResult = {
  workspace: Workspace;
  syncGroup?: WorkspaceSyncGroup;
};

export async function duplicate_workspace(
  options: DuplicateWorkspaceOptions,
  createWorkspaceFn: (workspace: Omit<Workspace, "Id">) => Promise<Workspace>
): Promise<DuplicateWorkspaceResult> {
  const {
    sourceWorkspaceId,
    newName,
    newDescription,
    environment,
    copyVariables,
    copySecrets,
    createSyncGroup: shouldCreateSyncGroup,
    syncGroupName,
    variablesToSync = [],
  } = options;

  // Create the new workspace
  const newWorkspace = await createWorkspaceFn({
    Name: newName,
    Description: newDescription,
    environment,
    syncGroupId: null,
  });

  // Copy collections and requests
  const collections = await get_collections_by_workspace(sourceWorkspaceId);
  const collectionIdMap = new Map<string, string>();

  for (const collection of collections) {
    const newCollection = await create_collection({
      name: collection.name,
      description: collection.description,
      workspaceId: newWorkspace.Id,
    });
    collectionIdMap.set(collection.id, newCollection.id);

    // Copy requests in collection
    for (const request of collection.requests) {
      await create_request({
        name: request.name,
        method: request.method,
        url: request.url,
        headers: [...request.headers],
        params: [...request.params],
        bodyType: request.bodyType,
        body: request.body,
        formData: [...request.formData],
        collectionId: newCollection.id,
        workspaceId: newWorkspace.Id,
      });
    }
  }

  // Copy standalone requests
  const standaloneRequests = await get_standalone_requests_by_workspace(
    sourceWorkspaceId
  );
  for (const request of standaloneRequests) {
    await create_request({
      name: request.name,
      method: request.method,
      url: request.url,
      headers: [...request.headers],
      params: [...request.params],
      bodyType: request.bodyType,
      body: request.body,
      formData: [...request.formData],
      collectionId: null,
      workspaceId: newWorkspace.Id,
    });
  }

  // Copy variables
  if (copyVariables) {
    const sourceVariables = await get_workspace_variables(sourceWorkspaceId);
    for (const variable of sourceVariables) {
      // Skip secrets if not copying them
      if (variable.isSecret && !copySecrets) {
        // Create variable with empty value for secrets
        await create_variable({
          name: variable.name,
          value: "",
          scope: "workspace",
          scopeId: newWorkspace.Id,
          isSecret: true,
          description: variable.description,
        });
      } else {
        await create_variable({
          name: variable.name,
          value: variable.value,
          scope: "workspace",
          scopeId: newWorkspace.Id,
          isSecret: variable.isSecret,
          description: variable.description,
        });
      }
    }
  }

  // Create sync group if requested
  let syncGroup: WorkspaceSyncGroup | undefined;
  if (shouldCreateSyncGroup) {
    syncGroup = await create_sync_group(
      syncGroupName || `${newName} Sync Group`,
      [sourceWorkspaceId, newWorkspace.Id],
      variablesToSync,
      copySecrets
    );
    newWorkspace.syncGroupId = syncGroup.id;
  }

  return { workspace: newWorkspace, syncGroup };
}

export async function sync_variables_in_group(
  groupId: string,
  sourceWorkspaceId: string
): Promise<{ synced: number; skipped: number }> {
  const group = syncGroups.get(groupId);
  if (!group) return { synced: 0, skipped: 0 };

  const sourceVariables = await get_workspace_variables(sourceWorkspaceId);
  let synced = 0;
  let skipped = 0;

  for (const targetWorkspaceId of group.workspaceIds) {
    if (targetWorkspaceId === sourceWorkspaceId) continue;

    const targetVariables = await get_workspace_variables(targetWorkspaceId);
    const targetVarMap = new Map(targetVariables.map((v) => [v.name, v]));

    for (const sourceVar of sourceVariables) {
      // Only sync variables that are in the sync list
      if (!group.syncedVariableNames.includes(sourceVar.name)) {
        skipped++;
        continue;
      }

      // Skip secrets if not syncing them
      if (sourceVar.isSecret && !group.syncSecrets) {
        skipped++;
        continue;
      }

      const targetVar = targetVarMap.get(sourceVar.name);
      if (targetVar) {
        // Update existing variable
        await update_variable(targetVar.id, { value: sourceVar.value });
        synced++;
      } else {
        // Create new variable in target workspace
        await create_variable({
          name: sourceVar.name,
          value:
            sourceVar.isSecret && !group.syncSecrets ? "" : sourceVar.value,
          scope: "workspace",
          scopeId: targetWorkspaceId,
          isSecret: sourceVar.isSecret,
          description: sourceVar.description,
        });
        synced++;
      }
    }
  }

  return { synced, skipped };
}
