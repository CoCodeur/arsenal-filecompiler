<script lang="ts">
  import type { CompilationProgress } from "../lib/api";

  export let progress: CompilationProgress;

  $: percent = progress.total > 0 ? Math.round((progress.current / progress.total) * 100) : 0;
</script>

<div class="flex-1 flex items-center gap-3">
  <div class="flex-1 h-2 bg-white/10 rounded-full overflow-hidden">
    <div
      class="h-full rounded-full transition-all duration-300
             {progress.status === 'error' ? 'bg-red-500' : 'bg-arsenal-500'}"
      style="width: {percent}%"
    />
  </div>
  <span class="text-xs text-white/50 font-mono w-12 text-right">{percent}%</span>
</div>
