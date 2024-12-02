/**
 *
 *
 * Author: Elias Sjödin
 * Created: 2024-12-02
 */

import { readFile, readFileJSON, writeFileJSON } from "$lib/services/files-service";
import { proxiesStore, type ProxyGroup } from "$lib/stores/proxies-store.svelte";

export const loadProxyGroup = async (groupName: string) => {
  const filePath = `proxies/${groupName}.json`;
  const proxiesContent = await readFileJSON(filePath);
  if (!proxiesContent) return [];

	const proxies = proxiesContent
		.split(/\r?\n/)
		.filter((line: string) => line.trim())
		.map((proxy: string) => proxy.trim());
  return proxies;
};

export const loadProxyGroups = async () => {
  const proxyGroups = await readFileJSON("proxies.json");
	if (!proxyGroups) return;
	proxiesStore.groups = [ ...proxiesStore.groups, ...proxyGroups ];
}

export const addProxyGroup = async ({ name, file: filePath }: ProxyGroup) => {
	const trimmedName = name
		.split(" ")
		.filter(value => Boolean(value))
		.join(" ")
		.trim();

	const existingFile = await readFile(`proxies/${trimmedName}.json`);
  if (existingFile) {
    console.log("File already exists for this proxy group");
    return;
  }

	const proxiesContent = await readFile(filePath);
  if (!proxiesContent) {
    console.log("Invalid proxy file content");
    return;
  }

	const proxies = proxiesContent
    .split(/\r?\n/)
    .filter(line => line.trim())
    .map(proxy => proxy.trim());

  await writeFileJSON(`proxies/${trimmedName}.json`, proxies);

  let metadata = await readFileJSON("proxies.json");
  if (!metadata) metadata = [];

	let newGroup = {
		name: trimmedName,
		file: `proxies/${trimmedName}.json`,
		originalFilePath: filePath,
	};
  metadata.push(newGroup);

  await writeFileJSON("proxies.json", metadata);
	proxiesStore.groups = [ ...proxiesStore.groups, newGroup ];
};
