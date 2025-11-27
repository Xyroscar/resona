export type Workspace = {
  Id: string;
  Name: string;
  Description: string;
  Tags: string[];
  SyncGroupId?: string | null;
  CreatedAt: string;
  UpdatedAt: string;
};

export type CreateWorkspaceInput = {
  name: string;
  description: string;
  tags?: string[];
};

export type UpdateWorkspaceInput = {
  id: string;
  name?: string;
  description?: string;
  tags?: string[];
  sync_group_id?: string;
};

export type WorkspaceSyncGroup = {
  Id: string;
  Name: string;
  WorkspaceIds: string[];
  SyncedVariableNames: string[];
  SyncSecrets: boolean;
  CreatedAt: string;
  UpdatedAt: string;
};

export type CreateSyncGroupInput = {
  name: string;
  workspace_ids: string[];
  synced_variable_names?: string[];
  sync_secrets?: boolean;
};

export type UpdateSyncGroupInput = {
  id: string;
  name?: string;
  synced_variable_names?: string[];
  sync_secrets?: boolean;
};

export type AppSettings = {
  theme: "light" | "dark" | "system";
  defaultTimeout: number;
  followRedirects: boolean;
  validateSSL: boolean;
  maxHistoryItems: number;
  autoSaveRequests: boolean;
};
