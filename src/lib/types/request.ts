export type HttpMethod =
  | "GET"
  | "POST"
  | "PUT"
  | "PATCH"
  | "DELETE"
  | "HEAD"
  | "OPTIONS";

export type RequestHeader = {
  key: string;
  value: string;
  enabled: boolean;
};

export type RequestParam = {
  key: string;
  value: string;
  enabled: boolean;
};

export type Request = {
  id: string;
  name: string;
  method: HttpMethod;
  url: string;
  headers: RequestHeader[];
  params: RequestParam[];
  body: string;
  collectionId: string | null;
  workspaceId: string;
};
