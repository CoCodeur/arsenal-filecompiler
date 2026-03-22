<script lang="ts">
  import type { NasConfig } from "../lib/api";
  import { saveConfig, testNasConnection } from "../lib/api";

  export let nasConfig: NasConfig;
  export let defaultOutputDir: string;

  let testing = false;
  let testResult: { success: boolean; message: string } | null = null;
  let saving = false;
  let saved = false;

  async function testConnection() {
    testing = true;
    testResult = null;
    try {
      testResult = await testNasConnection(nasConfig);
    } catch (err) {
      testResult = { success: false, message: `${err}` };
    } finally {
      testing = false;
    }
  }

  async function handleSave() {
    saving = true;
    saved = false;
    try {
      await saveConfig({ nas: nasConfig, default_output_dir: defaultOutputDir });
      saved = true;
      setTimeout(() => (saved = false), 2000);
    } catch (err) {
      console.error("Failed to save config:", err);
    } finally {
      saving = false;
    }
  }
</script>

<div class="flex-1 p-6 overflow-y-auto">
  <div class="max-w-lg space-y-6">
    <section>
      <h2 class="text-sm font-semibold mb-4 text-white/80">Connexion NAS</h2>
      <div class="space-y-3">
        <div>
          <label for="server" class="block text-xs text-white/50 mb-1">Serveur (IP ou hostname)</label>
          <input
            id="server"
            type="text"
            bind:value={nasConfig.server}
            placeholder="192.168.1.168"
            class="w-full px-3 py-2 bg-white/5 border border-white/10 rounded text-sm
                   focus:outline-none focus:border-arsenal-500 transition-colors"
          />
        </div>
        <div>
          <label for="share" class="block text-xs text-white/50 mb-1">Chemin du partage</label>
          <input
            id="share"
            type="text"
            bind:value={nasConfig.share_path}
            placeholder="Server_NAS/image seiko/Leroy Merlin"
            class="w-full px-3 py-2 bg-white/5 border border-white/10 rounded text-sm
                   focus:outline-none focus:border-arsenal-500 transition-colors"
          />
        </div>
        <div class="grid grid-cols-2 gap-3">
          <div>
            <label for="username" class="block text-xs text-white/50 mb-1">Utilisateur</label>
            <input
              id="username"
              type="text"
              bind:value={nasConfig.username}
              class="w-full px-3 py-2 bg-white/5 border border-white/10 rounded text-sm
                     focus:outline-none focus:border-arsenal-500 transition-colors"
            />
          </div>
          <div>
            <label for="password" class="block text-xs text-white/50 mb-1">Mot de passe</label>
            <input
              id="password"
              type="password"
              bind:value={nasConfig.password}
              class="w-full px-3 py-2 bg-white/5 border border-white/10 rounded text-sm
                     focus:outline-none focus:border-arsenal-500 transition-colors"
            />
          </div>
        </div>
        <button
          class="text-sm px-4 py-2 bg-white/10 hover:bg-white/15 rounded transition-colors"
          on:click={testConnection}
          disabled={testing}
        >
          {testing ? "Test en cours..." : "Tester la connexion"}
        </button>
        {#if testResult}
          <p class="text-xs {testResult.success ? 'text-green-400' : 'text-red-400'}">
            {testResult.message}
          </p>
        {/if}
      </div>
    </section>

    <section>
      <h2 class="text-sm font-semibold mb-4 text-white/80">Général</h2>
      <div>
        <label for="output" class="block text-xs text-white/50 mb-1">Dossier de sortie par défaut</label>
        <input
          id="output"
          type="text"
          bind:value={defaultOutputDir}
          placeholder="C:\FileCompiler"
          class="w-full px-3 py-2 bg-white/5 border border-white/10 rounded text-sm
                 focus:outline-none focus:border-arsenal-500 transition-colors"
        />
      </div>
    </section>

    <button
      class="px-6 py-2.5 bg-arsenal-600 hover:bg-arsenal-500 rounded-lg font-medium
             transition-all text-sm disabled:opacity-40"
      on:click={handleSave}
      disabled={saving}
    >
      {#if saved}
        Sauvegardé !
      {:else if saving}
        Sauvegarde...
      {:else}
        Sauvegarder
      {/if}
    </button>
  </div>
</div>
