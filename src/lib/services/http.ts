import { invoke } from "@tauri-apps/api/core";
import type { Request } from "$lib/types/request";
import type { ResolvedVariable } from "$lib/types/variable";
import { interpolate_variables } from "./variables";

export type HttpResponse = {
  status: number;
  statusText: string;
  headers: { key: string; value: string }[];
  body: string;
  timeMs: number;
  sizeBytes: number;
};

type RustHttpRequest = {
  method: string;
  url: string;
  headers: { key: string; value: string; enabled: boolean }[];
  params: { key: string; value: string; enabled: boolean }[];
  body_type: string;
  body: string;
  form_data: {
    key: string;
    value: string;
    item_type: string;
    enabled: boolean;
  }[];
};

type RustHttpResponse = {
  status: number;
  status_text: string;
  headers: { key: string; value: string }[];
  body: string;
  time_ms: number;
  size_bytes: number;
};

export async function send_request(
  request: Request,
  variables: ResolvedVariable[] = []
): Promise<HttpResponse> {
  // Interpolate variables in URL
  const url = interpolate_variables(request.url, variables);

  // Interpolate variables in headers
  const headers = request.headers.map((h) => ({
    key: interpolate_variables(h.key, variables),
    value: interpolate_variables(h.value, variables),
    enabled: h.enabled,
  }));

  // Interpolate variables in params
  const params = request.params.map((p) => ({
    key: interpolate_variables(p.key, variables),
    value: interpolate_variables(p.value, variables),
    enabled: p.enabled,
  }));

  // Interpolate variables in body
  const body = interpolate_variables(request.body, variables);

  // Interpolate variables in form data
  const formData = request.formData.map((f) => ({
    key: interpolate_variables(f.key, variables),
    value: interpolate_variables(f.value, variables),
    item_type: f.type,
    enabled: f.enabled,
  }));

  const rustRequest: RustHttpRequest = {
    method: request.method,
    url,
    headers,
    params,
    body_type: request.bodyType,
    body,
    form_data: formData,
  };

  const response = await invoke<RustHttpResponse>("send_http_request", {
    request: rustRequest,
  });

  return {
    status: response.status,
    statusText: response.status_text,
    headers: response.headers,
    body: response.body,
    timeMs: response.time_ms,
    sizeBytes: response.size_bytes,
  };
}

export function format_size(bytes: number): string {
  if (bytes < 1024) return `${bytes} B`;
  if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`;
  return `${(bytes / (1024 * 1024)).toFixed(1)} MB`;
}

export function format_time(ms: number): string {
  if (ms < 1000) return `${ms} ms`;
  return `${(ms / 1000).toFixed(2)} s`;
}
