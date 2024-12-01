/**
 *
 *
 * Author: Elias Sjödin
 * Created: 2024-11-28
 */

import { resolve, dirname, appDataDir, BaseDirectory } from "@tauri-apps/api/path";
import {
  mkdir,
  writeTextFile,
  readTextFile,
  exists,
} from "@tauri-apps/plugin-fs";
import { open } from "@tauri-apps/plugin-dialog";

interface Filter {
	name: string;
	extensions: string[];
}

interface SelectFileOptions {
	multiple?: boolean;
	directory?: boolean;
	filters?: Filter[];
}

const isAbsolutePath = (path: string): boolean => {
  return path.startsWith("/") || /^[a-zA-Z]:[\\/]/.test(path);
};

export const writeFile = async (fileName: string, data: any) => {
  try {
		const appDataPath = await appDataDir();
		const filePath = isAbsolutePath(fileName)
      ? fileName
      : await resolve(appDataPath, fileName);

    const directory = await dirname(filePath);
    const dirExists = await exists(directory);

    if (!dirExists) await mkdir(directory, { recursive: true });
    await writeTextFile(filePath, data);
  } catch (err) {
    console.log("Could not write to file: " + fileName, err)
  }
}

export const writeFileJSON = async (fileName: string, data: any) => {
  try {
    await writeFile(fileName, JSON.stringify(data, null, 4));
  } catch (err) {
    console.log("Could not write JSON to file: " + fileName, err)
  }
}

export const readFile = async (fileName: string) => {
  try {
    const fileExists = await exists(fileName, { baseDir: BaseDirectory.AppData });
    if (!fileExists) return;

    return await readTextFile(fileName, {
			baseDir: BaseDirectory.AppData
		});
  } catch (err) {
    console.log("Could not read file: " + fileName, err)
  }
}

export const readFileJSON = async (fileName: string) => {
  try {
    const fileContent = await readFile(fileName);
    if (!fileContent) return;
    return JSON.parse(fileContent);
  } catch (err) {
    console.log("Could not read JSON file: " + fileName, err)
  }
}

export const selectFile = async (options: SelectFileOptions) => {
	const filePath = await open({
		multiple: options.multiple,
		directory: options.directory,
		filters: options.filters,
	});
	return filePath;
}
