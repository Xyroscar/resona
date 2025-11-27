import type {
  Workspace,
  WorkspaceSyncGroup,
  CreateWorkspaceInput,
} from "$lib/types/workspace";
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
} from "./variables";
import {
  create_sync_group as createSyncGroupBackend,
  get_sync_group as getSyncGroupBackend,
} from "./workspaces";

export type DuplicateWorkspaceOptions = {
  sourceWorkspaceId: string;
  newName: string;
  newDescription: string;
  tags?: string[];
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
  createWorkspaceFn: (input: CreateWorkspaceInput) => Promise<Workspace>
): Promise<DuplicateWorkspaceResult> {
  const {
    sourceWorkspaceId,
    newName,
    newDescription,
    tags = [],
    copyVariables,
    copySecrets,
    createSyncGroup: shouldCreateSyncGroup,
    syncGroupName,
    variablesToSync = [],
  } = options;

  // Create the new workspace
  const newWorkspace = await createWorkspaceFn({
    name: newName,
    description: newDescription,
    tags,
  });

  // Copy collections and requests
  const collections = await get_collections_by_workspace(sourceWorkspaceId);

  for (const collection of collections) {
    const newCollection = await create_collection({
      name: collection.name,
      description: collection.description,
      workspaceId: newWorkspace.Id,
    });

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
      if (variable.isSecret && !copySecrets) {
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
    syncGroup = await createSyncGroupBackend({
      name: syncGroupName || `${newName} Sync Group`,
      workspace_ids: [sourceWorkspaceId, newWorkspace.Id],
      synced_variable_names: variablesToSync,
      sync_secrets: copySecrets,
    });
  }

  return { workspace: newWorkspace, syncGroup };
}

export async function sync_variables_in_group(
  groupId: string,
  sourceWorkspaceId: string
): Promise<{ synced: number; skipped: number }> {
  const group = await getSyncGroupBackend(groupId);
  if (!group) return { synced: 0, skipped: 0 };

  const sourceVariables = await get_workspace_variables(sourceWorkspaceId);
  let synced = 0;
  let skipped = 0;

  for (const targetWorkspaceId of group.WorkspaceIds) {
    if (targetWorkspaceId === sourceWorkspaceId) continue;

    const targetVariables = await get_workspace_variables(targetWorkspaceId);
    const targetVarMap = new Map(targetVariables.map((v) => [v.name, v]));

    for (const sourceVar of sourceVariables) {
      if (!group.SyncedVariableNames.includes(sourceVar.name)) {
        skipped++;
        continue;
      }

      if (sourceVar.isSecret && !group.SyncSecrets) {
        skipped++;
        continue;
      }

      const targetVar = targetVarMap.get(sourceVar.name);
      if (targetVar) {
        await update_variable(targetVar.id, { value: sourceVar.value });
        synced++;
      } else {
        await create_variable({
          name: sourceVar.name,
          value:
            sourceVar.isSecret && !group.SyncSecrets ? "" : sourceVar.value,
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
