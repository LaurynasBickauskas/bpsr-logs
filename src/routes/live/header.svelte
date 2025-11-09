<script lang="ts">
  import { onMount } from "svelte";
  import { commands, type HeaderInfo } from "$lib/bindings";

  onMount(() => {
    fetchData();
    const interval = setInterval(fetchData, 200);
    return () => clearInterval(interval);
  });

  async function fetchData() {
    try {
      const result = await commands.getHeaderInfo();
      if (result.status !== "ok") {
        console.warn("Failed to get header: ", result.error);
        return;
      } else {
        headerInfo = result.data;
        // console.log("header: ", +Date.now(), $state.snapshot(headerInfo));
      }
    } catch (e) {
      console.error("Error fetching data: ", e);
    }
  }


  let headerInfo: HeaderInfo = $state({
    totalDmg: 0,
    elapsedMs: 0,
    timeLastCombatPacketMs: Date.now(), // TODO: tempfix
  });
</script>

<header
  data-tauri-drag-region
  class="sticky top-0 flex h-2
   w-full items-center justify-between bg-neutral-900/80 px-2 text-xs text-neutral-200"
>
</header>
