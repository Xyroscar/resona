<script lang="ts">
  import * as Dialog from "$lib/components/ui/dialog/index.js";
  import * as Tabs from "$lib/components/ui/tabs/index.js";
  import * as Select from "$lib/components/ui/select/index.js";
  import { Button } from "$lib/components/ui/button/index.js";
  import { Input } from "$lib/components/ui/input/index.js";
  import { Label } from "$lib/components/ui/label/index.js";
  import { Separator } from "$lib/components/ui/separator/index.js";
  import SettingsIcon from "@lucide/svelte/icons/settings";
  import type { AppSettings } from "$lib/types/workspace";
  import {
    get_settings,
    update_settings,
    reset_settings,
  } from "$lib/services/settings";
  import { onMount } from "svelte";

  type Props = {
    open: boolean;
    onOpenChange: (open: boolean) => void;
  };

  let { open = $bindable(), onOpenChange }: Props = $props();

  let settings = $state<AppSettings>({
    theme: "system",
    defaultTimeout: 30000,
    followRedirects: true,
    validateSSL: true,
    maxHistoryItems: 100,
    autoSaveRequests: true,
  });

  let loading = $state(false);

  onMount(async () => {
    settings = await get_settings();
  });

  $effect(() => {
    if (open) {
      loadSettings();
    }
  });

  async function loadSettings() {
    settings = await get_settings();
  }

  async function handleSave() {
    loading = true;
    await update_settings(settings);
    loading = false;
    onOpenChange(false);
  }

  async function handleReset() {
    settings = await reset_settings();
  }

  function handleThemeChange(value: string) {
    settings.theme = value as "light" | "dark" | "system";
  }
</script>

<Dialog.Root bind:open {onOpenChange}>
  <Dialog.Content
    class="sm:max-w-[600px] max-h-[80vh] overflow-hidden flex flex-col"
  >
    <Dialog.Header>
      <Dialog.Title class="flex items-center gap-2">
        <SettingsIcon class="size-5" />
        Settings
      </Dialog.Title>
      <Dialog.Description>
        Configure application settings and preferences.
      </Dialog.Description>
    </Dialog.Header>

    <Tabs.Root value="general" class="flex-1 overflow-hidden">
      <Tabs.List class="grid w-full grid-cols-3">
        <Tabs.Trigger value="general">General</Tabs.Trigger>
        <Tabs.Trigger value="requests">Requests</Tabs.Trigger>
        <Tabs.Trigger value="advanced">Advanced</Tabs.Trigger>
      </Tabs.List>

      <div class="mt-4 overflow-auto max-h-[400px]">
        <Tabs.Content value="general" class="space-y-4">
          <div class="space-y-2">
            <Label for="theme">Theme</Label>
            <Select.Root
              type="single"
              value={settings.theme}
              onValueChange={handleThemeChange}
            >
              <Select.Trigger id="theme" class="w-full">
                {settings.theme === "light"
                  ? "Light"
                  : settings.theme === "dark"
                    ? "Dark"
                    : "System"}
              </Select.Trigger>
              <Select.Content>
                <Select.Item value="system">System</Select.Item>
                <Select.Item value="light">Light</Select.Item>
                <Select.Item value="dark">Dark</Select.Item>
              </Select.Content>
            </Select.Root>
            <p class="text-xs text-muted-foreground">
              Choose the application color theme.
            </p>
          </div>

          <Separator />

          <div class="space-y-2">
            <Label for="maxHistory">History Items</Label>
            <Input
              id="maxHistory"
              type="number"
              min="10"
              max="1000"
              bind:value={settings.maxHistoryItems}
            />
            <p class="text-xs text-muted-foreground">
              Maximum number of request history items to keep.
            </p>
          </div>

          <div class="flex items-center justify-between">
            <div class="space-y-0.5">
              <Label>Auto-save Requests</Label>
              <p class="text-xs text-muted-foreground">
                Automatically save request changes.
              </p>
            </div>
            <button
              type="button"
              role="switch"
              aria-checked={settings.autoSaveRequests}
              aria-label="Toggle auto-save requests"
              class="relative inline-flex h-6 w-11 shrink-0 cursor-pointer rounded-full border-2 border-transparent transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50 {settings.autoSaveRequests
                ? 'bg-primary'
                : 'bg-input'}"
              onclick={() =>
                (settings.autoSaveRequests = !settings.autoSaveRequests)}
            >
              <span
                class="pointer-events-none block h-5 w-5 rounded-full bg-background shadow-lg ring-0 transition-transform {settings.autoSaveRequests
                  ? 'translate-x-5'
                  : 'translate-x-0'}"
              ></span>
            </button>
          </div>
        </Tabs.Content>

        <Tabs.Content value="requests" class="space-y-4">
          <div class="space-y-2">
            <Label for="timeout">Default Timeout (ms)</Label>
            <Input
              id="timeout"
              type="number"
              min="1000"
              max="300000"
              step="1000"
              bind:value={settings.defaultTimeout}
            />
            <p class="text-xs text-muted-foreground">
              Default timeout for HTTP requests in milliseconds.
            </p>
          </div>

          <Separator />

          <div class="flex items-center justify-between">
            <div class="space-y-0.5">
              <Label>Follow Redirects</Label>
              <p class="text-xs text-muted-foreground">
                Automatically follow HTTP redirects.
              </p>
            </div>
            <button
              type="button"
              role="switch"
              aria-checked={settings.followRedirects}
              aria-label="Toggle follow redirects"
              class="relative inline-flex h-6 w-11 shrink-0 cursor-pointer rounded-full border-2 border-transparent transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50 {settings.followRedirects
                ? 'bg-primary'
                : 'bg-input'}"
              onclick={() =>
                (settings.followRedirects = !settings.followRedirects)}
            >
              <span
                class="pointer-events-none block h-5 w-5 rounded-full bg-background shadow-lg ring-0 transition-transform {settings.followRedirects
                  ? 'translate-x-5'
                  : 'translate-x-0'}"
              ></span>
            </button>
          </div>

          <div class="flex items-center justify-between">
            <div class="space-y-0.5">
              <Label>Validate SSL Certificates</Label>
              <p class="text-xs text-muted-foreground">
                Verify SSL/TLS certificates for HTTPS requests.
              </p>
            </div>
            <button
              type="button"
              role="switch"
              aria-checked={settings.validateSSL}
              aria-label="Toggle SSL validation"
              class="relative inline-flex h-6 w-11 shrink-0 cursor-pointer rounded-full border-2 border-transparent transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50 {settings.validateSSL
                ? 'bg-primary'
                : 'bg-input'}"
              onclick={() => (settings.validateSSL = !settings.validateSSL)}
            >
              <span
                class="pointer-events-none block h-5 w-5 rounded-full bg-background shadow-lg ring-0 transition-transform {settings.validateSSL
                  ? 'translate-x-5'
                  : 'translate-x-0'}"
              ></span>
            </button>
          </div>
        </Tabs.Content>

        <Tabs.Content value="advanced" class="space-y-4">
          <div class="rounded-lg border p-4 bg-muted/30">
            <h4 class="font-medium mb-2">Reset Settings</h4>
            <p class="text-sm text-muted-foreground mb-4">
              Reset all settings to their default values. This action cannot be
              undone.
            </p>
            <Button variant="destructive" size="sm" onclick={handleReset}>
              Reset to Defaults
            </Button>
          </div>

          <div class="rounded-lg border p-4 bg-muted/30">
            <h4 class="font-medium mb-2">Data Management</h4>
            <p class="text-sm text-muted-foreground mb-4">
              Export or import your workspaces, collections, and settings.
            </p>
            <div class="flex gap-2">
              <Button variant="outline" size="sm" disabled>Export Data</Button>
              <Button variant="outline" size="sm" disabled>Import Data</Button>
            </div>
          </div>
        </Tabs.Content>
      </div>
    </Tabs.Root>

    <Dialog.Footer class="mt-4">
      <Button variant="outline" onclick={() => onOpenChange(false)}>
        Cancel
      </Button>
      <Button onclick={handleSave} disabled={loading}>
        {loading ? "Saving..." : "Save Changes"}
      </Button>
    </Dialog.Footer>
  </Dialog.Content>
</Dialog.Root>
