/**
 *
 *
 * Author: Elias Sjödin
 * Created: 2024-11-28
 */

type Callback = (event: KeyboardEvent) => void;
let hotkeys: { [key: string]: Callback } = {};

export const addHotkey = (key: string, callback: Callback) => {
	hotkeys = { ...hotkeys, [key]: callback };
}

export const removeHotkey = (key: string) => {
	delete hotkeys[key];
}

const handleKeyDown = (event: KeyboardEvent) => {
  const key = event.key.toLowerCase();
  const meta = event.metaKey ? "meta+" : "";
  const shortcut = `${meta}${key}`;
	if (hotkeys[shortcut]) hotkeys[shortcut](event);
}

const init = () => {
  window.addEventListener("keydown", handleKeyDown);
  return () => window.removeEventListener("keydown", handleKeyDown);
}

export const cleanup = init();
