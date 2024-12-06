/**
 *
 *
 * Author: Elias Sjödin
 * Created: 2024-12-02
 */

import { deleteFile, readFile, readFileJSON, writeFileJSON } from "$lib/services/files-service";
import { proxiesStore, type ProxyGroup } from "$lib/stores/proxies-store.svelte";

const rollbackDeletedFile = async (filePath: string, data: any) => {
	try {
		await writeFileJSON(filePath, data);
	} catch (err) {
		console.log("Failed to rollback proxy group deletion. Manual recovery may be required.", err);
	}
}

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
	proxiesStore.groups = [ ...proxyGroups ];
}

export const addProxyGroup = async ({ name, file: filePath }: ProxyGroup) => {
	const trimmedName = name
		.split(" ")
		.filter(value => Boolean(value))
		.join(" ")
		.trim();

	const existingFile = await readFile(`proxies/${trimmedName}.json`);
  if (existingFile) {
    throw new Error("A group with this name already exists");
  }

	const proxiesContent = await readFile(filePath);
  if (!proxiesContent) {
    throw new Error("Invalid proxy file content");
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
		amount: proxies.length,
	};
  metadata.push(newGroup);

  await writeFileJSON("proxies.json", metadata);
	proxiesStore.groups = [ ...proxiesStore.groups, newGroup ];
};

export const deleteProxyGroup = async (groupName: string) => {
	const filePath = `proxies/${groupName}.json`;

  let metadata = await readFileJSON("proxies.json");
  if (!metadata || !Array.isArray(metadata)) {
    throw new Error("Unable to load proxy metadata");
  }

  const groupIndex = metadata.findIndex(group => group.name === groupName);
  if (groupIndex === -1) {
    throw new Error(`Proxy group "${groupName}" does not exist`);
  }

	// In-case a rollback is needed after the file is deleted
	const groupData = await readFileJSON(filePath);

  try {
    await deleteFile(filePath);
  } catch {
    throw new Error("Failed to delete proxy group");
  }

	try {
		metadata.splice(groupIndex, 1);
		await writeFileJSON("proxies.json", metadata);
		proxiesStore.groups = metadata;
	} catch {
		if (groupData) await rollbackDeletedFile(filePath, groupData);
		throw new Error("Failed to update proxy metadata. Rollback attempted.");
	}
}
