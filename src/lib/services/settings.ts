import type { AppSettings } from "$lib/types/workspace";

let settings: AppSettings = {
  theme: "system",
  defaultTimeout: 30000,
  followRedirects: true,
  validateSSL: true,
  maxHistoryItems: 100,
  autoSaveRequests: true,
};

export async function get_settings(): Promise<AppSettings> {
  return { ...settings };
}

export async function update_settings(
  updates: Partial<AppSettings>
): Promise<AppSettings> {
  settings = { ...settings, ...updates };
  return { ...settings };
}

export async function reset_settings(): Promise<AppSettings> {
  settings = {
    theme: "system",
    defaultTimeout: 30000,
    followRedirects: true,
    validateSSL: true,
    maxHistoryItems: 100,
    autoSaveRequests: true,
  };
  return { ...settings };
}
