<script lang="ts">
  import CodeMirror from "svelte-codemirror-editor";
  import { json } from "@codemirror/lang-json";
  import { xml } from "@codemirror/lang-xml";
  import { html } from "@codemirror/lang-html";
  import type { BodyType } from "$lib/types/request";

  type Props = {
    value: string;
    language: BodyType;
    readonly?: boolean;
    placeholder?: string;
    class?: string;
    onchange?: (value: string) => void;
  };

  let {
    value = "",
    language = "json",
    readonly = false,
    placeholder = "",
    class: className = "",
    onchange,
  }: Props = $props();

  function getLanguageSupport(lang: BodyType) {
    switch (lang) {
      case "json":
        return json();
      case "xml":
        return xml();
      case "html":
        return html();
      default:
        return undefined;
    }
  }

  function handleChange(newValue: string) {
    if (onchange) {
      onchange(newValue);
    }
  }

  let lang = $derived(getLanguageSupport(language));
</script>

<div class="code-editor-wrapper {className}">
  <CodeMirror
    bind:value
    {lang}
    {readonly}
    {placeholder}
    lineNumbers={true}
    tabSize={2}
    lineWrapping={true}
    onchange={handleChange}
    styles={{
      "&": {
        height: "100%",
        fontSize: "13px",
      },
      ".cm-scroller": {
        fontFamily:
          "ui-monospace, SFMono-Regular, 'SF Mono', Menlo, Consolas, monospace",
      },
      ".cm-gutters": {
        backgroundColor: "hsl(var(--muted))",
        borderRight: "1px solid hsl(var(--border))",
      },
      ".cm-activeLineGutter": {
        backgroundColor: "hsl(var(--accent))",
      },
      ".cm-activeLine": {
        backgroundColor: "hsl(var(--accent) / 0.3)",
      },
      "&.cm-focused": {
        outline: "none",
      },
      ".cm-content": {
        caretColor: "hsl(var(--foreground))",
      },
    }}
  />
</div>

<style>
  .code-editor-wrapper {
    border: 1px solid hsl(var(--border));
    border-radius: calc(var(--radius) - 2px);
    overflow: hidden;
  }

  .code-editor-wrapper :global(.cm-editor) {
    height: 100%;
    background-color: hsl(var(--background));
    color: hsl(var(--foreground));
  }

  .code-editor-wrapper :global(.cm-placeholder) {
    color: hsl(var(--muted-foreground));
  }
</style>
