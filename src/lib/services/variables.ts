import type {
  Variable,
  VariableScope,
  ResolvedVariable,
} from "$lib/types/variable";

const variables: Map<string, Variable> = new Map<string, Variable>([
  [
    "var-1",
    {
      id: "var-1",
      name: "BASE_URL",
      value: "https://api.example.com",
      scope: "global",
      scopeId: null,
      isSecret: false,
      description: "Base URL for all API requests",
    },
  ],
  [
    "var-2",
    {
      id: "var-2",
      name: "API_KEY",
      value: "sk-1234567890",
      scope: "global",
      scopeId: null,
      isSecret: true,
      description: "API authentication key",
    },
  ],
  [
    "var-3",
    {
      id: "var-3",
      name: "USER_ID",
      value: "user-123",
      scope: "workspace",
      scopeId: "1",
      isSecret: false,
      description: "Default user ID for testing",
    },
  ],
  [
    "var-4",
    {
      id: "var-4",
      name: "AUTH_TOKEN",
      value: "Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9",
      scope: "workspace",
      scopeId: "1",
      isSecret: true,
      description: "Authentication token",
    },
  ],
]);

export async function get_all_variables(): Promise<Variable[]> {
  return [...variables.values()];
}

export async function get_variables_by_scope(
  scope: VariableScope,
  scopeId: string | null
): Promise<Variable[]> {
  return [...variables.values()].filter(
    (v) => v.scope === scope && v.scopeId === scopeId
  );
}

export async function get_global_variables(): Promise<Variable[]> {
  return get_variables_by_scope("global", null);
}

export async function get_workspace_variables(
  workspaceId: string
): Promise<Variable[]> {
  return get_variables_by_scope("workspace", workspaceId);
}

export async function get_collection_variables(
  collectionId: string
): Promise<Variable[]> {
  return get_variables_by_scope("collection", collectionId);
}

export async function get_request_variables(
  requestId: string
): Promise<Variable[]> {
  return get_variables_by_scope("request", requestId);
}

export async function get_resolved_variables(
  workspaceId: string,
  collectionId: string | null,
  requestId: string | null
): Promise<ResolvedVariable[]> {
  const resolved: Map<string, ResolvedVariable> = new Map();

  const globalVars = await get_global_variables();
  for (const v of globalVars) {
    resolved.set(v.name, {
      name: v.name,
      value: v.value,
      scope: v.scope,
      isSecret: v.isSecret,
    });
  }

  const workspaceVars = await get_workspace_variables(workspaceId);
  for (const v of workspaceVars) {
    resolved.set(v.name, {
      name: v.name,
      value: v.value,
      scope: v.scope,
      isSecret: v.isSecret,
    });
  }

  if (collectionId) {
    const collectionVars = await get_collection_variables(collectionId);
    for (const v of collectionVars) {
      resolved.set(v.name, {
        name: v.name,
        value: v.value,
        scope: v.scope,
        isSecret: v.isSecret,
      });
    }
  }

  if (requestId) {
    const requestVars = await get_request_variables(requestId);
    for (const v of requestVars) {
      resolved.set(v.name, {
        name: v.name,
        value: v.value,
        scope: v.scope,
        isSecret: v.isSecret,
      });
    }
  }

  return [...resolved.values()];
}

export async function get_variable(id: string): Promise<Variable | undefined> {
  return variables.get(id);
}

export async function create_variable(
  variable: Omit<Variable, "id">
): Promise<Variable> {
  const newVariable: Variable = {
    ...variable,
    id: crypto.randomUUID(),
  };
  variables.set(newVariable.id, newVariable);
  return newVariable;
}

export async function update_variable(
  id: string,
  updates: Partial<Omit<Variable, "id">>
): Promise<boolean> {
  const variable = variables.get(id);
  if (!variable) return false;

  Object.assign(variable, updates);
  return true;
}

export async function delete_variable(id: string): Promise<boolean> {
  return variables.delete(id);
}

export function interpolate_variables(
  text: string,
  resolvedVariables: ResolvedVariable[]
): string {
  let result = text;
  for (const variable of resolvedVariables) {
    const pattern = new RegExp(`\\{\\{\\s*${variable.name}\\s*\\}\\}`, "g");
    result = result.replace(pattern, variable.value);
  }
  return result;
}

export function extract_variable_names(text: string): string[] {
  const pattern = /\{\{\s*([a-zA-Z_][a-zA-Z0-9_]*)\s*\}\}/g;
  const matches: string[] = [];
  let match;
  while ((match = pattern.exec(text)) !== null) {
    if (!matches.includes(match[1])) {
      matches.push(match[1]);
    }
  }
  return matches;
}
