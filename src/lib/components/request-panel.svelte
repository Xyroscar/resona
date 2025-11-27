<script lang="ts">
  import { Button } from "$lib/components/ui/button/index.js";
  import { Input } from "$lib/components/ui/input/index.js";
  import * as DropdownMenu from "$lib/components/ui/dropdown-menu/index.js";
  import * as Select from "$lib/components/ui/select/index.js";
  import * as Tabs from "$lib/components/ui/tabs/index.js";
  import SendIcon from "@lucide/svelte/icons/send";
  import ChevronDownIcon from "@lucide/svelte/icons/chevron-down";
  import XIcon from "@lucide/svelte/icons/x";
  import CodeEditor from "./code-editor.svelte";
  import VariableInput from "./variable-input.svelte";
  import type { Request, HttpMethod, BodyType } from "$lib/types/request";
  import type { ResolvedVariable } from "$lib/types/variable";

  type Props = {
    request: Request;
    variables?: ResolvedVariable[];
    onSend: (request: Request) => void;
    onUpdate: (request: Request) => void;
  };

  let { request, variables = [], onSend, onUpdate }: Props = $props();

  let localRequest = $state<Request>({ ...request });

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
  const bodyTypes: { value: BodyType; label: string }[] = [
    { value: "none", label: "None" },
    { value: "json", label: "JSON" },
    { value: "xml", label: "XML" },
    { value: "text", label: "Text" },
    { value: "html", label: "HTML" },
    { value: "form-data", label: "Form Data" },
    { value: "x-www-form-urlencoded", label: "URL Encoded" },
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

  function handleUrlChange(value: string) {
    localRequest.url = value;
    onUpdate(localRequest);
  }

  function handleBodyTypeChange(value: BodyType) {
    localRequest.bodyType = value;
    onUpdate(localRequest);
  }

  function handleBodyChange(value: string) {
    localRequest.body = value;
    onUpdate(localRequest);
  }

  function handleSend() {
    onSend(localRequest);
  }

  function formatBody() {
    if (localRequest.bodyType === "json") {
      try {
        localRequest.body = JSON.stringify(
          JSON.parse(localRequest.body),
          null,
          2
        );
        onUpdate(localRequest);
      } catch {
        // Invalid JSON, don't format
      }
    }
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

      <VariableInput
        class="flex-1"
        placeholder={"Enter request URL (use {{VAR_NAME}} for variables)"}
        value={localRequest.url}
        {variables}
        oninput={handleUrlChange}
      />

      <Button onclick={handleSend}>
        <SendIcon class="size-4 mr-2" />
        Send
      </Button>
    </div>
  </div>

  <Tabs.Root value="params" class="flex-1 flex flex-col overflow-hidden">
    <Tabs.List
      class="w-full justify-start rounded-none border-b bg-transparent p-0 h-auto"
    >
      <Tabs.Trigger
        value="params"
        class="rounded-none border-b-2 border-transparent data-[state=active]:border-primary data-[state=active]:bg-transparent px-4 py-2"
      >
        Params
      </Tabs.Trigger>
      <Tabs.Trigger
        value="headers"
        class="rounded-none border-b-2 border-transparent data-[state=active]:border-primary data-[state=active]:bg-transparent px-4 py-2"
      >
        Headers
      </Tabs.Trigger>
      <Tabs.Trigger
        value="body"
        class="rounded-none border-b-2 border-transparent data-[state=active]:border-primary data-[state=active]:bg-transparent px-4 py-2"
      >
        Body
      </Tabs.Trigger>
    </Tabs.List>

    <Tabs.Content value="params" class="flex-1 m-0 p-4 overflow-auto">
      <div class="space-y-2">
        {#each localRequest.params as param, i (i)}
          <div class="flex items-center gap-2">
            <VariableInput
              class="flex-1"
              placeholder="Key"
              value={param.key}
              {variables}
              oninput={(value) => {
                localRequest.params[i].key = value;
                onUpdate(localRequest);
              }}
            />
            <VariableInput
              class="flex-1"
              placeholder="Value"
              value={param.value}
              {variables}
              oninput={(value) => {
                localRequest.params[i].value = value;
                onUpdate(localRequest);
              }}
            />
            <Button
              variant="ghost"
              size="icon"
              onclick={() => {
                localRequest.params = localRequest.params.filter(
                  (_, idx) => idx !== i
                );
                onUpdate(localRequest);
              }}
            >
              <XIcon class="size-4" />
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
    </Tabs.Content>

    <Tabs.Content value="headers" class="flex-1 m-0 p-4 overflow-auto">
      <div class="space-y-2">
        {#each localRequest.headers as header, i (i)}
          <div class="flex items-center gap-2">
            <VariableInput
              class="flex-1"
              placeholder="Key"
              value={header.key}
              {variables}
              oninput={(value) => {
                localRequest.headers[i].key = value;
                onUpdate(localRequest);
              }}
            />
            <VariableInput
              class="flex-1"
              placeholder="Value"
              value={header.value}
              {variables}
              oninput={(value) => {
                localRequest.headers[i].value = value;
                onUpdate(localRequest);
              }}
            />
            <Button
              variant="ghost"
              size="icon"
              onclick={() => {
                localRequest.headers = localRequest.headers.filter(
                  (_, idx) => idx !== i
                );
                onUpdate(localRequest);
              }}
            >
              <XIcon class="size-4" />
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
    </Tabs.Content>

    <Tabs.Content value="body" class="flex-1 m-0 flex flex-col overflow-hidden">
      <div class="flex items-center gap-2 p-2 border-b bg-muted/30">
        <span class="text-sm text-muted-foreground">Content Type:</span>
        <Select.Root
          type="single"
          value={localRequest.bodyType}
          onValueChange={(value) => handleBodyTypeChange(value as BodyType)}
        >
          <Select.Trigger class="w-40 h-8">
            {bodyTypes.find((t) => t.value === localRequest.bodyType)?.label ||
              "None"}
          </Select.Trigger>
          <Select.Content>
            {#each bodyTypes as bodyType (bodyType.value)}
              <Select.Item value={bodyType.value}>{bodyType.label}</Select.Item>
            {/each}
          </Select.Content>
        </Select.Root>
        {#if localRequest.bodyType === "json"}
          <Button variant="ghost" size="sm" onclick={formatBody}>Format</Button>
        {/if}
      </div>
      <div class="flex-1 p-2 overflow-hidden">
        {#if localRequest.bodyType === "none"}
          <div
            class="flex items-center justify-center h-full text-muted-foreground text-sm"
          >
            This request does not have a body
          </div>
        {:else if localRequest.bodyType === "form-data" || localRequest.bodyType === "x-www-form-urlencoded"}
          <div class="space-y-2 p-2">
            {#each localRequest.formData as item, i (i)}
              <div class="flex items-center gap-2">
                <VariableInput
                  class="flex-1"
                  placeholder="Key"
                  value={item.key}
                  {variables}
                  oninput={(value) => {
                    localRequest.formData[i].key = value;
                    onUpdate(localRequest);
                  }}
                />
                <VariableInput
                  class="flex-1"
                  placeholder="Value"
                  value={item.value}
                  {variables}
                  oninput={(value) => {
                    localRequest.formData[i].value = value;
                    onUpdate(localRequest);
                  }}
                />
                <Button
                  variant="ghost"
                  size="icon"
                  onclick={() => {
                    localRequest.formData = localRequest.formData.filter(
                      (_, idx) => idx !== i
                    );
                    onUpdate(localRequest);
                  }}
                >
                  <XIcon class="size-4" />
                </Button>
              </div>
            {/each}
            <Button
              variant="outline"
              size="sm"
              onclick={() => {
                localRequest.formData = [
                  ...localRequest.formData,
                  { key: "", value: "", type: "text", enabled: true },
                ];
                onUpdate(localRequest);
              }}
            >
              Add Field
            </Button>
          </div>
        {:else}
          <CodeEditor
            value={localRequest.body}
            language={localRequest.bodyType}
            placeholder="Enter request body..."
            class="h-full"
            onchange={handleBodyChange}
          />
        {/if}
      </div>
    </Tabs.Content>
  </Tabs.Root>
</div>
