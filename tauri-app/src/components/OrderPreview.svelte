<script lang="ts">
  import type { Order } from "../lib/api";

  export let order: Order;

  $: materialGroups = order.products.reduce(
    (acc, p) => {
      if (!acc[p.material]) acc[p.material] = [];
      acc[p.material].push(p);
      return acc;
    },
    {} as Record<string, typeof order.products>
  );

  $: totalFiles = order.products.reduce((sum, p) => sum + p.quantity, 0);
</script>

<div class="bg-white/5 rounded-lg border border-white/10 p-4">
  <div class="flex items-center justify-between mb-3">
    <div class="flex items-center gap-3">
      <h2 class="font-semibold text-sm">Commande {order.id}</h2>
      <span class="text-xs text-white/40">{order.client}</span>
      <span class="text-xs text-white/30">{order.date}</span>
    </div>
    <div class="flex gap-3 text-xs text-white/50">
      <span>{order.products.length} produits</span>
      <span>{totalFiles} fichiers</span>
      <span>{Object.keys(materialGroups).length} matières</span>
    </div>
  </div>

  <div class="grid grid-cols-2 lg:grid-cols-3 gap-2 max-h-32 overflow-y-auto">
    {#each Object.entries(materialGroups) as [material, products]}
      <div class="bg-white/5 rounded px-3 py-2">
        <div class="text-xs font-medium text-arsenal-500 mb-1">{material}</div>
        {#each products as product}
          <div class="text-xs text-white/60 truncate" title={product.description}>
            {product.reference} <span class="text-white/30">x{product.quantity}</span>
          </div>
        {/each}
      </div>
    {/each}
  </div>
</div>
