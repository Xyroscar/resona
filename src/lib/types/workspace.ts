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

export type Theme =
  | "system"
  | "light"
  | "dark"
  | "latte"
  | "frappe"
  | "macchiato"
  | "mocha"
  | "custom";

export type ThemeColors = {
  background: string;
  foreground: string;
  primary: string;
  secondary: string;
  accent: string;
  muted: string;
  border: string;
};

export type CustomTheme = {
  name: string;
  colors: ThemeColors;
};

export type AppSettings = {
  theme: Theme;
  customThemes: CustomTheme[];
  defaultTimeout: number;
  followRedirects: boolean;
  validateSsl: boolean;
  maxHistoryItems: number;
  autoSaveRequests: boolean;
};

export type UpdateSettingsInput = {
  theme?: Theme;
  customThemes?: CustomTheme[];
  defaultTimeout?: number;
  followRedirects?: boolean;
  validateSsl?: boolean;
  maxHistoryItems?: number;
  autoSaveRequests?: boolean;
};
