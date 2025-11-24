import type { Workspace } from "$lib/types/workspace";

const ws: Map<string, Workspace> = new Map<string, Workspace>([
  [
    "1",
    {
      Id: "1",
      Name: "Test 1",
      Description: "This is a test description",
    },
  ],
  [
    "2",
    {
      Id: "2",
      Name: "Test 2",
      Description: "This is a longer test description",
    },
  ],
  [
    "3",
    {
      Id: "3",
      Name: "Test 3",
      Description: "This is a slightly longer test description",
    },
  ],
  [
    "4",
    {
      Id: "4",
      Name: "Test 4",
      Description: "This is an even slightly longer test description",
    },
  ],
  [
    "5",
    {
      Id: "5",
      Name: "Test 5",
      Description:
        "This is a veryyyyyyyyyyyyyyyyyyyyyyyyyyy longggggggggggggggggggggggggggg test descriptionnnnnnnnnnnnnnnnnnnnnnnnnnnnnn",
    },
  ],
]);

function convert_to_list(ws: Map<string, Workspace>): Workspace[] {
  return [...ws.values()];
}

export async function get_workspaces(): Promise<Workspace[]> {
  return convert_to_list(ws);
}

export async function update_workspace(workspace: Workspace): Promise<boolean> {
  let w = ws.get(workspace.Id);
  if (w != undefined) {
    w.Name = workspace.Name;
    w.Description = workspace.Description;
  } else {
    return false;
  }
  return true;
}

export async function create_workspace(workspace: Workspace): Promise<boolean> {
  workspace.Id = crypto.randomUUID();
  ws.set(workspace.Id, workspace);
  return true;
}
