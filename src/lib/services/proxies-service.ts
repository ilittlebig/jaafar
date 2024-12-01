/**
 *
 *
 * Author: Elias Sjödin
 * Created: 2024-12-02
 */

import { readFile, readFileJSON, writeFileJSON } from "$lib/services/files-service";
import { proxiesStore } from "$lib/stores/proxies-store.svelte";

export const loadProxies = async () => {
  const proxies = await readFileJSON("proxies.json");
	if (!proxies) return;
	proxies.forEach((proxy: string) => proxiesStore.push(proxy));
}

export const importProxies = async (filePath: string) => {
	const proxiesContent = await readFile(filePath);
	if (!proxiesContent) return;

	const proxies = proxiesContent
		.split(/\r?\n/)
		.filter((line) => line.trim())
		.map(proxy => proxy.trim());

	await writeFileJSON("proxies.json", proxies);
	proxies.forEach((proxy: string) => proxiesStore.push(proxy));
};
