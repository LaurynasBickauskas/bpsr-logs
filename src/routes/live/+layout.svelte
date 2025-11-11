<script lang="ts">
  import { onMount } from "svelte";
  import Footer from "./footer.svelte";
  import Header from "./header.svelte";
  import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";
  import { goto } from "$app/navigation";

  let { children } = $props();

  // TODO: workaround, need to wait for svelte tanstack devs to respond
  onMount(() => {
    const interval = setInterval(refreshWindow, 5 * 60 * 1000); // refresh every 5m
    return () => clearInterval(interval);
  });
  function refreshWindow() {
    window.location.reload();
  }

  const appWebview = getCurrentWebviewWindow();
  appWebview.listen<string>("navigate", (event) => {
    const route = event.payload;
    goto(route);
  });
</script>

<div class="flex h-screen flex-col text-sm text-white rounded-md border border-neutral-700">
  <Header/>
  <main class="flex-1 overflow-y-auto">
    {@render children()}
  </main>
  <Footer />
</div>

<style>
  :global {
    /* Hide scrollbars globally but keep scrolling functional */
    * {
      -ms-overflow-style: none; /* IE and Edge */
      scrollbar-width: none; /* Firefox */
    }
    *::-webkit-scrollbar {
      display: none; /* Chrome, Safari, Edge */
    }
  }
</style>
