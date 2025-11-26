export type Workspace = {
  Id: string;
  Name: string;
  Description: string;
  syncGroupId?: string | null;
  environment?: string;
};

export type WorkspaceSyncGroup = {
  id: string;
  name: string;
  workspaceIds: string[];
  syncedVariableNames: string[];
  syncSecrets: boolean;
};

export type AppSettings = {
  theme: "light" | "dark" | "system";
  defaultTimeout: number;
  followRedirects: boolean;
  validateSSL: boolean;
  maxHistoryItems: number;
  autoSaveRequests: boolean;
};
