<script lang="ts">
  import { createEventDispatcher } from "svelte";
  import { open } from "@tauri-apps/plugin-dialog";
  import { parseXmlFile } from "../lib/api";

  export let outputDir = "";

  const dispatch = createEventDispatcher();

  let xmlFileName = "";
  let parsing = false;
  let error = "";

  async function selectXmlFile() {
    error = "";
    const selected = await open({
      title: "Sélectionner le fichier XML de commande",
      filters: [{ name: "XML", extensions: ["xml"] }],
      multiple: false,
    });
    if (!selected) return;

    const path = selected as string;
    xmlFileName = path.split(/[/\\]/).pop() || path;
    parsing = true;

    try {
      const order = await parseXmlFile(path);
      dispatch("orderParsed", { path, order });
    } catch (err) {
      error = `Erreur de parsing : ${err}`;
    } finally {
      parsing = false;
    }
  }

  async function selectOutputDir() {
    const selected = await open({
      title: "Sélectionner le dossier de destination",
      directory: true,
    });
    if (selected) {
      dispatch("outputSelected", selected as string);
    }
  }
</script>

<!-- XML File -->
<div class="bg-white/5 rounded-lg p-4 border border-white/10">
  <label class="block text-xs text-white/50 uppercase tracking-wider mb-2">Fichier XML</label>
  <button
    class="w-full text-left px-3 py-2 bg-white/5 hover:bg-white/10 rounded border border-white/10
           transition-colors text-sm truncate"
    on:click={selectXmlFile}
    disabled={parsing}
  >
    {#if parsing}
      <span class="text-white/40">Analyse en cours...</span>
    {:else if xmlFileName}
      <span class="text-white/80">{xmlFileName}</span>
    {:else}
      <span class="text-white/30">Cliquer pour sélectionner...</span>
    {/if}
  </button>
  {#if error}
    <p class="text-red-400 text-xs mt-1">{error}</p>
  {/if}
</div>

<!-- Output Directory -->
<div class="bg-white/5 rounded-lg p-4 border border-white/10">
  <label class="block text-xs text-white/50 uppercase tracking-wider mb-2">Dossier de destination</label>
  <button
    class="w-full text-left px-3 py-2 bg-white/5 hover:bg-white/10 rounded border border-white/10
           transition-colors text-sm truncate"
    on:click={selectOutputDir}
  >
    {#if outputDir}
      <span class="text-white/80">{outputDir}</span>
    {:else}
      <span class="text-white/30">Cliquer pour sélectionner...</span>
    {/if}
  </button>
</div>
