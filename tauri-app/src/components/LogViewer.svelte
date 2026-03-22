<script lang="ts">
  import { afterUpdate } from "svelte";

  export let logs: string[] = [];

  let container: HTMLElement;

  afterUpdate(() => {
    if (container) {
      container.scrollTop = container.scrollHeight;
    }
  });
</script>

<div class="flex-1 min-h-0 flex flex-col bg-black/30 rounded-lg border border-white/10">
  <div class="flex items-center justify-between px-4 py-2 border-b border-white/10">
    <span class="text-xs text-white/50 uppercase tracking-wider">Journal</span>
    {#if logs.length > 0}
      <span class="text-xs text-white/30">{logs.length} entrées</span>
    {/if}
  </div>

  <div bind:this={container} class="flex-1 overflow-y-auto p-4 font-mono text-xs space-y-0.5">
    {#if logs.length === 0}
      <p class="text-white/20">En attente de compilation...</p>
    {:else}
      {#each logs as log}
        <p
          class="leading-relaxed
                 {log.includes('[ERREUR]') ? 'text-red-400' : ''}
                 {log.includes('[SUCCESS]') ? 'text-green-400' : ''}
                 {log.includes('[PROCESSING]') ? 'text-white/60' : ''}
                 {!log.includes('[') ? 'text-white/50' : ''}"
        >
          {log}
        </p>
      {/each}
    {/if}
  </div>
</div>
