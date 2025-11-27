import { invoke } from "@tauri-apps/api/core";
import type {
  Workspace,
  CreateWorkspaceInput,
  UpdateWorkspaceInput,
  WorkspaceSyncGroup,
  CreateSyncGroupInput,
  UpdateSyncGroupInput,
} from "$lib/types/workspace";

export async function get_workspaces(): Promise<Workspace[]> {
  return invoke<Workspace[]>("get_workspaces");
}

export async function get_workspace(id: string): Promise<Workspace> {
  return invoke<Workspace>("get_workspace", { id });
}

export async function create_workspace(
  input: CreateWorkspaceInput
): Promise<Workspace> {
  return invoke<Workspace>("create_workspace", { input });
}

export async function update_workspace(
  input: UpdateWorkspaceInput
): Promise<Workspace> {
  return invoke<Workspace>("update_workspace", { input });
}

export async function delete_workspace(id: string): Promise<void> {
  return invoke<void>("delete_workspace", { id });
}

export async function get_sync_groups(): Promise<WorkspaceSyncGroup[]> {
  return invoke<WorkspaceSyncGroup[]>("get_sync_groups");
}

export async function get_sync_group(id: string): Promise<WorkspaceSyncGroup> {
  return invoke<WorkspaceSyncGroup>("get_sync_group", { id });
}

export async function get_sync_group_for_workspace(
  workspaceId: string
): Promise<WorkspaceSyncGroup | null> {
  return invoke<WorkspaceSyncGroup | null>("get_sync_group_for_workspace", {
    workspaceId,
  });
}

export async function create_sync_group(
  input: CreateSyncGroupInput
): Promise<WorkspaceSyncGroup> {
  return invoke<WorkspaceSyncGroup>("create_sync_group", { input });
}

export async function update_sync_group(
  input: UpdateSyncGroupInput
): Promise<WorkspaceSyncGroup> {
  return invoke<WorkspaceSyncGroup>("update_sync_group", { input });
}

export async function delete_sync_group(id: string): Promise<void> {
  return invoke<void>("delete_sync_group", { id });
}

export async function get_workspaces_by_sync_group(
  syncGroupId: string
): Promise<Workspace[]> {
  return invoke<Workspace[]>("get_workspaces_by_sync_group", { syncGroupId });
}

export async function add_workspace_to_sync_group(
  syncGroupId: string,
  workspaceId: string
): Promise<void> {
  return invoke<void>("add_workspace_to_sync_group", {
    syncGroupId,
    workspaceId,
  });
}

export async function remove_workspace_from_sync_group(
  syncGroupId: string,
  workspaceId: string
): Promise<void> {
  return invoke<void>("remove_workspace_from_sync_group", {
    syncGroupId,
    workspaceId,
  });
}
