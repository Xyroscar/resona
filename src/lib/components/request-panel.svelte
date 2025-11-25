<script lang="ts">
  import { Button } from "$lib/components/ui/button/index.js";
  import { Input } from "$lib/components/ui/input/index.js";
  import * as DropdownMenu from "$lib/components/ui/dropdown-menu/index.js";
  import SendIcon from "@lucide/svelte/icons/send";
  import ChevronDownIcon from "@lucide/svelte/icons/chevron-down";
  import type { Request, HttpMethod } from "$lib/types/request";

  type Props = {
    request: Request;
    onSend: (request: Request) => void;
    onUpdate: (request: Request) => void;
  };

  let { request, onSend, onUpdate }: Props = $props();

  let localRequest = $state<Request>({ ...request });
  let activeTab = $state<"params" | "headers" | "body">("params");

  $effect(() => {
    localRequest = { ...request };
  });

  const methods: HttpMethod[] = [
    "GET",
    "POST",
    "PUT",
    "PATCH",
    "DELETE",
    "HEAD",
    "OPTIONS",
  ];

  function getMethodColor(method: string): string {
    const colors: Record<string, string> = {
      GET: "bg-green-500/10 text-green-500 hover:bg-green-500/20",
      POST: "bg-blue-500/10 text-blue-500 hover:bg-blue-500/20",
      PUT: "bg-orange-500/10 text-orange-500 hover:bg-orange-500/20",
      PATCH: "bg-yellow-500/10 text-yellow-500 hover:bg-yellow-500/20",
      DELETE: "bg-red-500/10 text-red-500 hover:bg-red-500/20",
      HEAD: "bg-purple-500/10 text-purple-500 hover:bg-purple-500/20",
      OPTIONS: "bg-gray-500/10 text-gray-500 hover:bg-gray-500/20",
    };
    return colors[method] || "bg-gray-500/10 text-gray-500";
  }

  function handleMethodChange(method: HttpMethod) {
    localRequest.method = method;
    onUpdate(localRequest);
  }

  function handleUrlChange(e: Event) {
    const target = e.target as HTMLInputElement;
    localRequest.url = target.value;
    onUpdate(localRequest);
  }

  function handleSend() {
    onSend(localRequest);
  }
</script>

<div class="flex flex-col h-full">
  <div class="border-b p-4">
    <div class="flex items-center gap-2">
      <DropdownMenu.Root>
        <DropdownMenu.Trigger>
          {#snippet child({ props })}
            <Button
              variant="outline"
              class={`font-mono font-semibold min-w-[100px] ${getMethodColor(localRequest.method)}`}
              {...props}
            >
              {localRequest.method}
              <ChevronDownIcon class="ml-1 size-4" />
            </Button>
          {/snippet}
        </DropdownMenu.Trigger>
        <DropdownMenu.Content>
          {#each methods as method (method)}
            <DropdownMenu.Item onclick={() => handleMethodChange(method)}>
              <span
                class={`font-mono font-semibold ${getMethodColor(method).split(" ")[1]}`}
              >
                {method}
              </span>
            </DropdownMenu.Item>
          {/each}
        </DropdownMenu.Content>
      </DropdownMenu.Root>

      <Input
        class="flex-1 font-mono text-sm"
        placeholder="Enter request URL"
        value={localRequest.url}
        oninput={handleUrlChange}
      />

      <Button onclick={handleSend}>
        <SendIcon class="size-4 mr-2" />
        Send
      </Button>
    </div>
  </div>

  <div class="flex-1 flex flex-col overflow-hidden">
    <div class="border-b">
      <div class="flex">
        <button
          class={`px-4 py-2 text-sm font-medium border-b-2 transition-colors ${
            activeTab === "params"
              ? "border-primary text-primary"
              : "border-transparent text-muted-foreground hover:text-foreground"
          }`}
          onclick={() => (activeTab = "params")}
        >
          Params
        </button>
        <button
          class={`px-4 py-2 text-sm font-medium border-b-2 transition-colors ${
            activeTab === "headers"
              ? "border-primary text-primary"
              : "border-transparent text-muted-foreground hover:text-foreground"
          }`}
          onclick={() => (activeTab = "headers")}
        >
          Headers
        </button>
        <button
          class={`px-4 py-2 text-sm font-medium border-b-2 transition-colors ${
            activeTab === "body"
              ? "border-primary text-primary"
              : "border-transparent text-muted-foreground hover:text-foreground"
          }`}
          onclick={() => (activeTab = "body")}
        >
          Body
        </button>
      </div>
    </div>

    <div class="flex-1 p-4 overflow-auto">
      {#if activeTab === "params"}
        <div class="space-y-2">
          {#each localRequest.params as param, i (i)}
            <div class="flex items-center gap-2">
              <Input
                class="flex-1"
                placeholder="Key"
                value={param.key}
                oninput={(e) => {
                  const target = e.target as HTMLInputElement;
                  localRequest.params[i].key = target.value;
                  onUpdate(localRequest);
                }}
              />
              <Input
                class="flex-1"
                placeholder="Value"
                value={param.value}
                oninput={(e) => {
                  const target = e.target as HTMLInputElement;
                  localRequest.params[i].value = target.value;
                  onUpdate(localRequest);
                }}
              />
              <Button
                variant="ghost"
                size="sm"
                onclick={() => {
                  localRequest.params = localRequest.params.filter(
                    (_, idx) => idx !== i
                  );
                  onUpdate(localRequest);
                }}
              >
                ×
              </Button>
            </div>
          {/each}
          <Button
            variant="outline"
            size="sm"
            onclick={() => {
              localRequest.params = [
                ...localRequest.params,
                { key: "", value: "", enabled: true },
              ];
              onUpdate(localRequest);
            }}
          >
            Add Parameter
          </Button>
        </div>
      {:else if activeTab === "headers"}
        <div class="space-y-2">
          {#each localRequest.headers as header, i (i)}
            <div class="flex items-center gap-2">
              <Input
                class="flex-1"
                placeholder="Key"
                value={header.key}
                oninput={(e) => {
                  const target = e.target as HTMLInputElement;
                  localRequest.headers[i].key = target.value;
                  onUpdate(localRequest);
                }}
              />
              <Input
                class="flex-1"
                placeholder="Value"
                value={header.value}
                oninput={(e) => {
                  const target = e.target as HTMLInputElement;
                  localRequest.headers[i].value = target.value;
                  onUpdate(localRequest);
                }}
              />
              <Button
                variant="ghost"
                size="sm"
                onclick={() => {
                  localRequest.headers = localRequest.headers.filter(
                    (_, idx) => idx !== i
                  );
                  onUpdate(localRequest);
                }}
              >
                ×
              </Button>
            </div>
          {/each}
          <Button
            variant="outline"
            size="sm"
            onclick={() => {
              localRequest.headers = [
                ...localRequest.headers,
                { key: "", value: "", enabled: true },
              ];
              onUpdate(localRequest);
            }}
          >
            Add Header
          </Button>
        </div>
      {:else if activeTab === "body"}
        <textarea
          class="w-full h-64 p-3 font-mono text-sm border rounded-md bg-background resize-none focus:outline-none focus:ring-2 focus:ring-ring"
          placeholder="Request body (JSON, XML, etc.)"
          value={localRequest.body}
          oninput={(e) => {
            const target = e.target as HTMLTextAreaElement;
            localRequest.body = target.value;
            onUpdate(localRequest);
          }}
        ></textarea>
      {/if}
    </div>
  </div>
</div>
