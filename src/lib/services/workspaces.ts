import type { Workspace } from "$lib/types/workspace";

let ws: Workspace[] = [
    {
        Id: "1",
        Name: "Test 1",
        Description: "This is a test description"
    },
    {
        Id: "2",
        Name: "Test 2",
        Description: "This is a longer test description"
    },
    {
        Id: "3",
        Name: "Test 3",
        Description: "This is a slightly longer test description"
    },
    {
        Id: "4",
        Name: "Test 4",
        Description: "This is an even slightly longer test description"
    },
    {
        Id: "5",
        Name: "Test 5",
        Description: "This is a veryyyyyyyyyyyyyyyyyyyyyyyyyyy longggggggggggggggggggggggggggg test descriptionnnnnnnnnnnnnnnnnnnnnnnnnnnnnn"
    },
]

export async function get_workspaces(): Promise<Workspace[]> {
    return ws
}