import tippy from 'tippy.js';
import 'tippy.js/dist/tippy.css'; // optional for styling
import type { Attachment } from 'svelte/attachments';
import { writeText } from '@tauri-apps/plugin-clipboard-manager';
import { WebviewWindow } from '@tauri-apps/api/webviewWindow';

// https://svelte.dev/docs/svelte/@attach#Attachment-factories
export function tooltip(getContent: () => string): Attachment {
  return (element: Element) => {
    const tooltip = tippy(element, {
      content: "",
    });
    $effect(() => {
      tooltip.setContent(getContent())
    })
    return tooltip.destroy;
  };
}

export async function copyToClipboard(error: MouseEvent & { currentTarget: EventTarget & HTMLElement }, content: string) {
  // TODO: add a way to simulate a "click" animation
  error.stopPropagation();
  await writeText(content);
}

let isClickthrough = false;

export async function setClickthrough(bool: boolean) {
  const liveWindow = await WebviewWindow.getByLabel("live");
  await liveWindow?.setIgnoreCursorEvents(bool);
  isClickthrough = bool;
}

export async function toggleClickthrough() {
  const liveWindow = await WebviewWindow.getByLabel("live");
  await liveWindow?.setIgnoreCursorEvents(!isClickthrough);
  isClickthrough = !isClickthrough;
}
