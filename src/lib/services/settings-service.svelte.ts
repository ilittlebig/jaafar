/**
 *
 *
 * Author: Elias Sjödin
 * Created: 2024-11-28
 */

import { readFileJSON, writeFileJSON } from "$lib/services/files-service"
import { settingsStore, settingsLoaded, type Settings } from "$lib/stores/settings-store.svelte";
import { deepMerge } from "$lib/utils"

const updateSettingsStore = (settings: Settings) => {
	for (const key in settings) {
		if (typeof settings[key] === "object" && settings[key] !== null) {
			settingsStore[key] = {
				...settingsStore[key],
				...settings[key],
			};
		} else {
			settingsStore[key] = settings[key];
		}
	}
};

export const loadSettings = async () => {
	const settings = await readFileJSON("settings.json");
	if (settings) {
		const mergedSettings = deepMerge(settingsStore, settings);
		updateSettingsStore(mergedSettings);
	}
	await saveSettings();
	settingsLoaded.value = true;
}

export const saveSettings = async (settings?: Settings) => {
	if (settings) {
		const updatedSettings = deepMerge(settingsStore, settings);
		updateSettingsStore(updatedSettings);
	}
	await writeFileJSON("settings.json", settings || settingsStore);
}
