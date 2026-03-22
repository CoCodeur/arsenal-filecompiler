<script lang="ts">
  import FileSelector from "./components/FileSelector.svelte";
  import CompilationView from "./components/CompilationView.svelte";
  import LogViewer from "./components/LogViewer.svelte";
  import Settings from "./components/Settings.svelte";
  import OrderPreview from "./components/OrderPreview.svelte";
  import type { Order, CompilationResult, NasConfig, CompilationProgress } from "./lib/api";
  import { compileOrder, loadConfig, onProgress } from "./lib/api";

  let currentView: "main" | "settings" = "main";
  let xmlPath = "";
  let outputDir = "";
  let order: Order | null = null;
  let isCompiling = false;
  let progress: CompilationProgress | null = null;
  let result: CompilationResult | null = null;
  let logs: string[] = [];
  let nasConfig: NasConfig = {
    server: "",
    share_path: "",
    username: "",
    password: "",
  };

  // Load config on mount
  loadConfig()
    .then((config) => {
      nasConfig = config.nas;
      outputDir = config.default_output_dir;
    })
    .catch(() => {
      // Config not yet saved, use defaults
    });

  // Listen for progress events
  onProgress((p) => {
    progress = p;
    logs = [...logs, `[${p.status.toUpperCase()}] ${p.message}`];
  });

  function handleOrderParsed(event: CustomEvent<{ path: string; order: Order }>) {
    xmlPath = event.detail.path;
    order = event.detail.order;
    result = null;
    logs = [];
    progress = null;
  }

  function handleOutputSelected(event: CustomEvent<string>) {
    outputDir = event.detail;
  }

  async function startCompilation() {
    if (!xmlPath || !outputDir) return;

    isCompiling = true;
    result = null;
    logs = [];
    progress = null;

    try {
      result = await compileOrder(xmlPath, outputDir, nasConfig);
      if (result.success) {
        logs = [...logs, `Compilation terminée : ${result.files_copied} fichiers copiés`];
      }
      if (result.files_missing.length > 0) {
        logs = [
          ...logs,
          `Fichiers manquants (${result.files_missing.length}) :`,
          ...result.files_missing.map((f) => `  - ${f}`),
        ];
      }
      if (result.errors.length > 0) {
        logs = [...logs, ...result.errors.map((e) => `[ERREUR] ${e}`)];
      }
    } catch (err) {
      logs = [...logs, `[ERREUR] ${err}`];
    } finally {
      isCompiling = false;
    }
  }
</script>

<main class="h-screen flex flex-col">
  <!-- Header -->
  <header class="flex items-center justify-between px-6 py-3 bg-arsenal-800 border-b border-white/10">
    <div class="flex items-center gap-3">
      <h1 class="text-lg font-semibold tracking-tight">Arsenal File Compiler</h1>
      <span class="text-xs text-white/40 font-mono">v2.0</span>
    </div>
    <button
      class="text-sm text-white/60 hover:text-white transition-colors px-3 py-1 rounded hover:bg-white/10"
      on:click={() => (currentView = currentView === "main" ? "settings" : "main")}
    >
      {currentView === "main" ? "Paramètres" : "Retour"}
    </button>
  </header>

  {#if currentView === "settings"}
    <Settings bind:nasConfig bind:defaultOutputDir={outputDir} />
  {:else}
    <div class="flex-1 flex flex-col gap-4 p-6 overflow-hidden">
      <!-- File Selection -->
      <div class="grid grid-cols-2 gap-4">
        <FileSelector
          on:orderParsed={handleOrderParsed}
          on:outputSelected={handleOutputSelected}
          {outputDir}
        />
      </div>

      <!-- Order Preview -->
      {#if order}
        <OrderPreview {order} />
      {/if}

      <!-- Compile Button -->
      <div class="flex items-center gap-4">
        <button
          class="px-6 py-2.5 bg-arsenal-600 hover:bg-arsenal-500 disabled:opacity-40 disabled:cursor-not-allowed
                 rounded-lg font-medium transition-all text-sm"
          disabled={!xmlPath || !outputDir || isCompiling}
          on:click={startCompilation}
        >
          {isCompiling ? "Compilation en cours..." : "Lancer la Compilation"}
        </button>

        {#if progress}
          <CompilationView {progress} />
        {/if}

        {#if result}
          <span class="text-sm {result.success ? 'text-green-400' : 'text-red-400'}">
            {result.success
              ? `${result.files_copied} fichiers copiés`
              : `Terminé avec ${result.errors.length} erreur(s)`}
          </span>
        {/if}
      </div>

      <!-- Log Viewer -->
      <LogViewer {logs} />
    </div>
  {/if}
</main>
