import type { Request } from "./request";

export type Collection = {
  id: string;
  name: string;
  description: string;
  workspaceId: string;
  requests: Request[];
};
