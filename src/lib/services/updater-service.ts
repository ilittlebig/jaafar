/**
 *
 *
 * Author: Elias Sjödin
 * Created: 2024-11-27
 */

import { check } from "@tauri-apps/plugin-updater";
import { ask } from "@tauri-apps/plugin-dialog";
import { relaunch } from "@tauri-apps/plugin-process";

export const checkForUpdates = async () => {
	const update = await check();
	if (update?.available) {
    const yes = await ask(`Update to ${update.version} is available!\n\nRelease notes: ${update.body}`, {
      title: "Update Now!",
      kind: "info",
      okLabel: "Update",
      cancelLabel: "Cancel"
    });

    if (yes) {
      await update.downloadAndInstall();
      await relaunch();
    }
	}
}
