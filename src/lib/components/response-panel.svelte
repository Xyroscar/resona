<script lang="ts">
  import * as Tabs from "$lib/components/ui/tabs/index.js";
  import { Badge } from "$lib/components/ui/badge/index.js";
  import { Button } from "$lib/components/ui/button/index.js";
  import { Spinner } from "$lib/components/ui/spinner/index.js";
  import CodeEditor from "./code-editor.svelte";
  import CopyIcon from "@lucide/svelte/icons/copy";
  import CheckIcon from "@lucide/svelte/icons/check";
  import type { Response } from "$lib/types/response";
  import type { BodyType } from "$lib/types/request";

  type Props = {
    response: Response | null;
    loading?: boolean;
  };

  let { response, loading = false }: Props = $props();

  let copied = $state(false);

  function getStatusColor(status: number): string {
    if (status >= 200 && status < 300) return "bg-green-500/10 text-green-500";
    if (status >= 300 && status < 400) return "bg-blue-500/10 text-blue-500";
    if (status >= 400 && status < 500)
      return "bg-yellow-500/10 text-yellow-500";
    if (status >= 500) return "bg-red-500/10 text-red-500";
    return "bg-gray-500/10 text-gray-500";
  }

  function formatSize(bytes: number): string {
    if (bytes < 1024) return `${bytes} B`;
    if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(2)} KB`;
    return `${(bytes / (1024 * 1024)).toFixed(2)} MB`;
  }

  function formatDuration(ms: number): string {
    if (ms < 1000) return `${ms} ms`;
    return `${(ms / 1000).toFixed(2)} s`;
  }

  function getBodyType(): BodyType {
    if (!response) return "text";
    const ct = response.contentType.toLowerCase();
    if (ct.includes("json")) return "json";
    if (ct.includes("xml")) return "xml";
    if (ct.includes("html")) return "html";
    return "text";
  }

  function formatBody(body: string, type: BodyType): string {
    if (type === "json") {
      try {
        return JSON.stringify(JSON.parse(body), null, 2);
      } catch {
        return body;
      }
    }
    return body;
  }

  async function copyToClipboard() {
    if (!response) return;
    await navigator.clipboard.writeText(response.body);
    copied = true;
    setTimeout(() => (copied = false), 2000);
  }

  let bodyType = $derived(getBodyType());
  let formattedBody = $derived(
    response ? formatBody(response.body, bodyType) : ""
  );
</script>

<div class="flex flex-col h-full border-t">
  {#if loading}
    <div class="flex-1 flex items-center justify-center">
      <div class="flex flex-col items-center gap-3">
        <Spinner class="size-8" />
        <span class="text-sm text-muted-foreground">Sending request...</span>
      </div>
    </div>
  {:else if response}
    <div class="flex items-center gap-4 px-4 py-2 border-b bg-muted/30">
      <Badge class={getStatusColor(response.status)}>
        {response.status}
        {response.statusText}
      </Badge>
      <span class="text-sm text-muted-foreground">
        {formatDuration(response.duration)}
      </span>
      <span class="text-sm text-muted-foreground">
        {formatSize(response.size)}
      </span>
      <div class="flex-1"></div>
      <Button variant="ghost" size="sm" onclick={copyToClipboard}>
        {#if copied}
          <CheckIcon class="size-4 mr-1" />
          Copied
        {:else}
          <CopyIcon class="size-4 mr-1" />
          Copy
        {/if}
      </Button>
    </div>

    <Tabs.Root value="body" class="flex-1 flex flex-col">
      <Tabs.List
        class="w-full justify-start rounded-none border-b bg-transparent p-0"
      >
        <Tabs.Trigger
          value="body"
          class="rounded-none border-b-2 border-transparent data-[state=active]:border-primary data-[state=active]:bg-transparent"
        >
          Body
        </Tabs.Trigger>
        <Tabs.Trigger
          value="headers"
          class="rounded-none border-b-2 border-transparent data-[state=active]:border-primary data-[state=active]:bg-transparent"
        >
          Headers ({response.headers.length})
        </Tabs.Trigger>
      </Tabs.List>

      <Tabs.Content
        value="body"
        class="flex-1 m-0 p-0 data-[state=active]:flex data-[state=active]:flex-col"
      >
        <div class="flex-1 p-2">
          <CodeEditor
            value={formattedBody}
            language={bodyType}
            readonly={true}
            class="h-full"
          />
        </div>
      </Tabs.Content>

      <Tabs.Content
        value="headers"
        class="flex-1 m-0 data-[state=active]:flex data-[state=active]:flex-col overflow-auto"
      >
        <div class="p-4">
          <table class="w-full text-sm">
            <thead>
              <tr class="border-b">
                <th
                  class="text-left py-2 pr-4 font-medium text-muted-foreground"
                  >Name</th
                >
                <th class="text-left py-2 font-medium text-muted-foreground"
                  >Value</th
                >
              </tr>
            </thead>
            <tbody>
              {#each response.headers as header (header.key)}
                <tr class="border-b last:border-0">
                  <td class="py-2 pr-4 font-mono text-xs">{header.key}</td>
                  <td class="py-2 font-mono text-xs break-all"
                    >{header.value}</td
                  >
                </tr>
              {/each}
            </tbody>
          </table>
        </div>
      </Tabs.Content>
    </Tabs.Root>
  {:else}
    <div class="flex-1 flex items-center justify-center">
      <span class="text-sm text-muted-foreground"
        >Send a request to see the response</span
      >
    </div>
  {/if}
</div>
