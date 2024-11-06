export interface Variable {
  id: string;
  name: string;
  value: string;
  description?: string;
}

export interface Collection {
  id: string;
  workspace_id: string;
  name: string;
  description?: string;
  created_at: number;
  updated_at: number;
  requests: Request[];
}

export interface Request {
  id: string;
  name: string;
  url: string;
  method: 'GET' | 'POST' | 'PUT' | 'DELETE' | 'PATCH';
  headers: Header[];
  queryParams: QueryParam[];
  body?: string;
}

export interface Header {
  id: string;
  key: string;
  value: string;
  enabled: boolean;
}

export interface QueryParam {
  id: string;
  key: string;
  value: string;
  enabled: boolean;
}

export interface RequestResponse {
  status: number;
  statusText: string;
  headers: Record<string, string>;
  body: any;
  time: number;
}

export interface Workspace {
  id: string;
  name: string;
  variables?: Variable[];
  collections?: Collection[];
  description?: string;
  created_at: number;
  updated_at: number;
} 