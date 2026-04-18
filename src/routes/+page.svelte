<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { listen } from "@tauri-apps/api/event";
  import { unified } from "unified";
  import remarkParse from "remark-parse";
  import remarkRehype from "remark-rehype";
  import rehypeStringify from "rehype-stringify";
  import Editor from "$lib/editor/Editor.svelte";

  type Tab = "editor" | "preview";

  let fileName = $state<string | null>(null);
  let currentPath = $state<string | null>(null);
  let markdown = $state("");
  let html = $state("");
  let activeTab = $state<Tab>("editor");

  async function renderMarkdown(md: string) {
    const file = await unified()
      .use(remarkParse)
      .use(remarkRehype)
      .use(rehypeStringify)
      .process(md);
    html = String(file);
  }

  async function openFile() {
    let result: [string, string, string];
    try {
      result = await invoke("open_file");
    } catch {
      return;
    }
    const [name, content, path] = result;
    fileName = name;
    currentPath = path;
    markdown = content;
    await renderMarkdown(content);
  }

  async function saveFile() {
    if (!currentPath) {
      await saveFileAs();
      return;
    }
    try {
      await invoke("save_file", { path: currentPath, content: markdown });
    } catch (e) {
      console.error(e);
    }
  }

  async function saveFileAs() {
    let result: [string, string];
    try {
      result = await invoke("save_file_as", { content: markdown });
    } catch {
      return;
    }
    const [name, path] = result;
    fileName = name;
    currentPath = path;
  }

  function handleChange(value: string) {
    renderMarkdown(value);
  }

  function switchTab(tab: Tab) {
    activeTab = tab;
  }

  $effect(() => {
    const unlistens: (() => void)[] = [];
    Promise.all([
      listen("menu:open", () => openFile()),
      listen("menu:save", () => saveFile()),
      listen("menu:save-as", () => saveFileAs()),
    ]).then((fns) => unlistens.push(...fns));
    return () => unlistens.forEach((fn) => fn());
  });
</script>

<div class="app">
  <header>
    {#if fileName}
      <span class="filename">{fileName}</span>
    {/if}
    <div class="tabs">
      <button
        class="tab"
        class:active={activeTab === "editor"}
        onclick={() => switchTab("editor")}
      >編集</button>
      <button
        class="tab"
        class:active={activeTab === "preview"}
        onclick={() => switchTab("preview")}
      >プレビュー</button>
    </div>
  </header>

  <div class="workspace">
    <div class="pane" class:hidden={activeTab !== "editor"}>
      <Editor bind:content={markdown} onchange={handleChange} />
    </div>

    <div class="pane preview-pane" class:hidden={activeTab !== "preview"}>
      {#if html}
        <article class="preview">{@html html}</article>
      {:else}
        <p class="empty">Markdownファイルを開くか、編集タブで入力してください</p>
      {/if}
    </div>
  </div>
</div>

<style>
  * {
    box-sizing: border-box;
    margin: 0;
    padding: 0;
  }

  :global(body) {
    font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
    background: #1e1e1e;
    color: #d4d4d4;
    height: 100vh;
    overflow: hidden;
  }

  .app {
    display: flex;
    flex-direction: column;
    height: 100vh;
  }

  header {
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 6px 16px;
    background: #2d2d2d;
    border-bottom: 1px solid #3e3e3e;
    flex-shrink: 0;
  }

  .filename {
    font-size: 13px;
    color: #9ca3af;
    flex: 1;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .tabs {
    display: flex;
    gap: 2px;
    margin-left: auto;
  }

  .tab {
    padding: 4px 16px;
    background: transparent;
    color: #9ca3af;
    border: 1px solid transparent;
    border-radius: 4px;
    cursor: pointer;
    font-size: 13px;
  }

  .tab:hover {
    color: #d4d4d4;
    background: #3e3e3e;
  }

  .tab.active {
    color: #e5e7eb;
    background: #3e3e3e;
    border-color: #4b5563;
  }

  .workspace {
    flex: 1;
    overflow: hidden;
    position: relative;
  }

  .pane {
    position: absolute;
    inset: 0;
  }

  .pane.hidden {
    display: none;
  }

  .preview-pane {
    overflow-y: auto;
    padding: 32px;
  }

  .empty {
    color: #6b7280;
    text-align: center;
    margin-top: 40px;
    font-size: 14px;
  }

  .preview {
    max-width: 720px;
    margin: 0 auto;
    line-height: 1.7;
  }

  :global(.preview h1),
  :global(.preview h2),
  :global(.preview h3),
  :global(.preview h4),
  :global(.preview h5),
  :global(.preview h6) {
    color: #e5e7eb;
    margin: 1.5em 0 0.5em;
  }

  :global(.preview h1) { font-size: 2em; border-bottom: 1px solid #3e3e3e; padding-bottom: 0.3em; }
  :global(.preview h2) { font-size: 1.5em; border-bottom: 1px solid #3e3e3e; padding-bottom: 0.2em; }

  :global(.preview p) { margin: 0.8em 0; }

  :global(.preview code) {
    background: #2d2d2d;
    padding: 0.2em 0.4em;
    border-radius: 3px;
    font-size: 0.9em;
    font-family: "JetBrains Mono", "Fira Code", monospace;
  }

  :global(.preview pre) {
    background: #2d2d2d;
    padding: 1em;
    border-radius: 6px;
    overflow-x: auto;
    margin: 1em 0;
  }

  :global(.preview pre code) {
    background: none;
    padding: 0;
  }

  :global(.preview blockquote) {
    border-left: 4px solid #4b5563;
    padding-left: 1em;
    color: #9ca3af;
    margin: 1em 0;
  }

  :global(.preview ul),
  :global(.preview ol) {
    padding-left: 2em;
    margin: 0.8em 0;
  }

  :global(.preview li) { margin: 0.3em 0; }

  :global(.preview a) {
    color: #38bdf8;
    text-decoration: none;
  }

  :global(.preview a:hover) { text-decoration: underline; }

  :global(.preview table) {
    width: 100%;
    border-collapse: collapse;
    margin: 1em 0;
  }

  :global(.preview th),
  :global(.preview td) {
    border: 1px solid #3e3e3e;
    padding: 0.5em 1em;
  }

  :global(.preview th) { background: #2d2d2d; }

  :global(.preview hr) {
    border: none;
    border-top: 1px solid #3e3e3e;
    margin: 2em 0;
  }
</style>
