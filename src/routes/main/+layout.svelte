<script lang="ts">
  import Header from "./header.svelte";
  import { setupShortcuts } from "./settings/shortcuts";
  import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";
  import { goto } from "$app/navigation";

  let { children } = $props();

  $effect.pre(() => {
    (async () => {
      await setupShortcuts();
    })();
  });

  const appWebview = getCurrentWebviewWindow();
  appWebview.listen<string>("navigate", (event) => {
    const route = event.payload;
    goto(route);
  });
</script>

<div class="bg-background text-foreground flex min-h-screen flex-col">
  <Header />
  <main class="flex-1">
    <div class="mx-auto px-8 py-4">
      {@render children()}
    </div>
  </main>
</div>
