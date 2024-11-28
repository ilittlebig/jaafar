/**
 *
 *
 * Author: Elias Sjödin
 * Created: 2024-11-28
 */

import { readFileJSON, writeFileJSON } from "$lib/services/files-service"
import { settingsStore } from "$lib/stores/settings-store.svelte";
import { deepMerge } from "$lib/utils"

export const loadSettings = async () => {
	const settings = await readFileJSON("settings.json");
	if (settings) {
		const mergedSettings = deepMerge({ ...settingsStore }, settings);
		for (const key in mergedSettings) {
			settingsStore[key] = mergedSettings[key];
		}
	}
	await saveSettings();
}

export const saveSettings = async () => {
	await writeFileJSON("settings.json", settingsStore);
}
