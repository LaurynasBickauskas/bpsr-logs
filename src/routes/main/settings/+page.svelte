<script lang="ts">
  import * as Tabs from "$lib/components/ui/tabs/index.js";
  import Integration from "./integration.svelte";
  import Misc from "./misc.svelte";
  import Shortcuts from "./shortcuts.svelte";

  const settingsTabs = [
    { id: "shortcuts", label: "Shortcuts" },
    { id: "misc", label: "Misc" },
    { id: "integration", label: "Integration" },
  ];
</script>

<Tabs.Root value="shortcuts">
  <Tabs.List>
    {#each settingsTabs as settingsTab (settingsTab.id)}
      <Tabs.Trigger value={settingsTab.id}>{settingsTab.label}</Tabs.Trigger>
    {/each}
  </Tabs.List>
  <Shortcuts />
  <Misc />
  <Integration />
</Tabs.Root>
