import { invoke } from "@tauri-apps/api/core";
import type {
  Variable,
  VariableScope,
  ResolvedVariable,
} from "$lib/types/variable";

// Rust variable types
type RustVariable = {
  id: string;
  name: string;
  value: string;
  scope: string;
  scope_id: string | null;
  is_secret: boolean;
  description: string | null;
  created_at: string;
  updated_at: string;
};

type RustResolvedVariable = {
  name: string;
  value: string;
  scope: string;
  is_secret: boolean;
};

type CreateVariableInput = {
  name: string;
  value: string;
  scope: string;
  scope_id: string | null;
  is_secret: boolean;
  description: string | null;
};

type UpdateVariableInput = {
  id: string;
  name?: string;
  value?: string;
  is_secret?: boolean;
  description?: string;
};

function toVariable(rust: RustVariable): Variable {
  return {
    id: rust.id,
    name: rust.name,
    value: rust.value,
    scope: rust.scope as VariableScope,
    scopeId: rust.scope_id,
    isSecret: rust.is_secret,
    description: rust.description ?? undefined,
  };
}

function toResolvedVariable(rust: RustResolvedVariable): ResolvedVariable {
  return {
    name: rust.name,
    value: rust.value,
    scope: rust.scope as VariableScope,
    isSecret: rust.is_secret,
  };
}

export async function get_global_variables(): Promise<Variable[]> {
  const vars = await invoke<RustVariable[]>("get_global_variables");
  return vars.map(toVariable);
}

export async function get_workspace_variables(
  workspaceId: string
): Promise<Variable[]> {
  const vars = await invoke<RustVariable[]>("get_workspace_variables", {
    workspaceId,
  });
  return vars.map(toVariable);
}

export async function get_collection_variables(
  collectionId: string
): Promise<Variable[]> {
  const vars = await invoke<RustVariable[]>("get_collection_variables", {
    collectionId,
  });
  return vars.map(toVariable);
}

export async function get_request_variables(
  requestId: string
): Promise<Variable[]> {
  const vars = await invoke<RustVariable[]>("get_request_variables", {
    requestId,
  });
  return vars.map(toVariable);
}

export async function get_variables_by_scope(
  scope: VariableScope,
  scopeId: string | null
): Promise<Variable[]> {
  switch (scope) {
    case "global":
      return get_global_variables();
    case "workspace":
      return scopeId ? get_workspace_variables(scopeId) : [];
    case "collection":
      return scopeId ? get_collection_variables(scopeId) : [];
    case "request":
      return scopeId ? get_request_variables(scopeId) : [];
    default:
      return [];
  }
}

export async function get_resolved_variables(
  workspaceId: string,
  collectionId: string | null,
  requestId: string | null
): Promise<ResolvedVariable[]> {
  const vars = await invoke<RustResolvedVariable[]>("get_resolved_variables", {
    workspaceId,
    collectionId,
    requestId,
  });
  return vars.map(toResolvedVariable);
}

export async function get_variable(id: string): Promise<Variable | undefined> {
  try {
    const v = await invoke<RustVariable>("get_variable", { id });
    return toVariable(v);
  } catch {
    return undefined;
  }
}

export async function create_variable(
  variable: Omit<Variable, "id">
): Promise<Variable> {
  const input: CreateVariableInput = {
    name: variable.name,
    value: variable.value,
    scope: variable.scope,
    scope_id: variable.scopeId,
    is_secret: variable.isSecret,
    description: variable.description ?? null,
  };
  const v = await invoke<RustVariable>("create_variable", { input });
  return toVariable(v);
}

export async function update_variable(
  id: string,
  updates: Partial<Omit<Variable, "id">>
): Promise<boolean> {
  try {
    const input: UpdateVariableInput = {
      id,
      name: updates.name,
      value: updates.value,
      is_secret: updates.isSecret,
      description: updates.description,
    };
    await invoke<RustVariable>("update_variable", { input });
    return true;
  } catch {
    return false;
  }
}

export async function delete_variable(id: string): Promise<boolean> {
  try {
    await invoke<void>("delete_variable", { id });
    return true;
  } catch {
    return false;
  }
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
