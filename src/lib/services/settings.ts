import { invoke } from "@tauri-apps/api/core";
import type { AppSettings, UpdateSettingsInput } from "$lib/types/workspace";

export async function get_settings(): Promise<AppSettings> {
  return invoke<AppSettings>("get_settings");
}

export async function update_settings(
  input: UpdateSettingsInput
): Promise<AppSettings> {
  return invoke<AppSettings>("update_settings", { input });
}

export async function reset_settings(): Promise<AppSettings> {
  return invoke<AppSettings>("reset_settings");
}
