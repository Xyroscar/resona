export type VariableScope = "global" | "workspace" | "collection" | "request";

export type Variable = {
  id: string;
  name: string;
  value: string;
  scope: VariableScope;
  scopeId: string | null;
  isSecret: boolean;
  description?: string;
};

export type ResolvedVariable = {
  name: string;
  value: string;
  scope: VariableScope;
  isSecret: boolean;
};
