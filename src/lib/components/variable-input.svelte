<script lang="ts">
  import { Input } from "$lib/components/ui/input/index.js";
  import type { ResolvedVariable } from "$lib/types/variable";

  type Props = {
    value: string;
    placeholder?: string;
    class?: string;
    variables: ResolvedVariable[];
    oninput?: (value: string) => void;
  };

  let {
    value = "",
    placeholder = "",
    class: className = "",
    variables = [],
    oninput,
  }: Props = $props();

  let showSuggestions = $state(false);
  let filteredVariables = $state<ResolvedVariable[]>([]);
  let selectedIndex = $state(0);
  let inputElement: HTMLInputElement | null = $state(null);
  let cursorPosition = $state(0);

  function findVariableContext(
    text: string,
    position: number
  ): { start: number; query: string } | null {
    const beforeCursor = text.substring(0, position);
    const match = beforeCursor.match(/\{\{\s*([a-zA-Z_][a-zA-Z0-9_]*)?$/);
    if (match) {
      return {
        start: match.index! + 2,
        query: (match[1] || "").toLowerCase(),
      };
    }
    return null;
  }

  function handleInput(e: Event) {
    const target = e.target as HTMLInputElement;
    cursorPosition = target.selectionStart || 0;

    const context = findVariableContext(target.value, cursorPosition);
    if (context) {
      filteredVariables = variables.filter((v) =>
        v.name.toLowerCase().includes(context.query)
      );
      showSuggestions = filteredVariables.length > 0;
      selectedIndex = 0;
    } else {
      showSuggestions = false;
    }

    if (oninput) {
      oninput(target.value);
    }
  }

  function handleKeydown(e: KeyboardEvent) {
    if (!showSuggestions) return;

    switch (e.key) {
      case "ArrowDown":
        e.preventDefault();
        selectedIndex = Math.min(
          selectedIndex + 1,
          filteredVariables.length - 1
        );
        break;
      case "ArrowUp":
        e.preventDefault();
        selectedIndex = Math.max(selectedIndex - 1, 0);
        break;
      case "Enter":
      case "Tab":
        if (filteredVariables.length > 0) {
          e.preventDefault();
          insertVariable(filteredVariables[selectedIndex]);
        }
        break;
      case "Escape":
        showSuggestions = false;
        break;
    }
  }

  function insertVariable(variable: ResolvedVariable) {
    const context = findVariableContext(value, cursorPosition);
    if (!context) return;

    const before = value.substring(0, context.start);
    const after = value.substring(cursorPosition);
    const newValue = `${before}${variable.name}}}${after}`;

    if (oninput) {
      oninput(newValue);
    }

    showSuggestions = false;

    setTimeout(() => {
      if (inputElement) {
        const newPosition = context.start + variable.name.length + 2;
        inputElement.setSelectionRange(newPosition, newPosition);
        inputElement.focus();
      }
    }, 0);
  }

  function handleBlur() {
    setTimeout(() => {
      showSuggestions = false;
    }, 150);
  }

  function getScopeColor(scope: string): string {
    const colors: Record<string, string> = {
      global: "text-purple-500",
      workspace: "text-blue-500",
      collection: "text-green-500",
      request: "text-orange-500",
    };
    return colors[scope] || "text-gray-500";
  }
</script>

<div class="relative {className}">
  <Input
    bind:ref={inputElement}
    {value}
    {placeholder}
    oninput={handleInput}
    onkeydown={handleKeydown}
    onblur={handleBlur}
    class="font-mono text-sm"
  />

  {#if showSuggestions}
    <div
      class="absolute z-50 top-full left-0 right-0 mt-1 bg-popover border rounded-md shadow-lg max-h-48 overflow-auto"
    >
      {#each filteredVariables as variable, i (variable.name)}
        <button
          type="button"
          class="w-full px-3 py-2 text-left text-sm hover:bg-accent flex items-center justify-between gap-2 {i ===
          selectedIndex
            ? 'bg-accent'
            : ''}"
          onmousedown={() => insertVariable(variable)}
        >
          <span class="font-mono">
            {#if variable.isSecret}
              🔒
            {/if}
            {variable.name}
          </span>
          <span class="text-xs {getScopeColor(variable.scope)}"
            >{variable.scope}</span
          >
        </button>
      {/each}
    </div>
  {/if}
</div>
