export type HttpMethod =
  | "GET"
  | "POST"
  | "PUT"
  | "PATCH"
  | "DELETE"
  | "HEAD"
  | "OPTIONS";

export type BodyType =
  | "none"
  | "json"
  | "xml"
  | "text"
  | "html"
  | "form-data"
  | "x-www-form-urlencoded";

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

export type FormDataItem = {
  key: string;
  value: string;
  type: "text" | "file";
  enabled: boolean;
};

export type Request = {
  id: string;
  name: string;
  method: HttpMethod;
  url: string;
  headers: RequestHeader[];
  params: RequestParam[];
  bodyType: BodyType;
  body: string;
  formData: FormDataItem[];
  collectionId: string | null;
  workspaceId: string;
};
