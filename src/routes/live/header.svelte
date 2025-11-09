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

  function formatElapsed(msElapsed: number) {
    const totalSeconds = Math.floor(Number(msElapsed) / 1000);
    const minutes = Math.floor((totalSeconds % 3600) / 60);
    const seconds = totalSeconds % 60;

    return `${String(minutes).padStart(2, "0")}:${String(seconds).padStart(2, "0")}`;
  }

  let headerInfo: HeaderInfo = $state({
    totalDmg: 0,
    elapsedMs: 0,
    timeLastCombatPacketMs: Date.now(), // TODO: tempfix
  });
</script>

<header
  data-tauri-drag-region
  class="sticky top-0 flex h-7 w-full items-center justify-between bg-neutral-900/80 px-2 text-xs text-neutral-200"
>
  <div class="flex flex-col leading-tight">
    <span class="font-medium">
      Total Damage: {headerInfo.totalDmg.toLocaleString()}
    </span>
    <span class="text-neutral-400">
      Elapsed: {formatElapsed(headerInfo.elapsedMs)}
    </span>
  </div>
  <div class="text-neutral-500">
    Last packet: {new Date(headerInfo.timeLastCombatPacketMs).toLocaleTimeString()}
  </div>
</header>
