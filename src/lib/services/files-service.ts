/**
 *
 *
 * Author: Elias Sjödin
 * Created: 2024-11-28
 */

import { appDataDir, BaseDirectory } from "@tauri-apps/api/path";
import {
  mkdir,
  writeTextFile,
  readTextFile,
  exists,
} from "@tauri-apps/plugin-fs";

export const writeFile = async (fileName: string, data: any) => {
  try {
    const appDirExists = await exists("", { baseDir: BaseDirectory.AppData });
    if (!appDirExists) {
      const appDataDirPath = await appDataDir();
      await mkdir(appDataDirPath);
    }
    writeTextFile(fileName, data, { baseDir: BaseDirectory.AppData });
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

export const writeFileToPath = async (fileName: string, data: any) => {
  try {
    writeTextFile(fileName, data, { baseDir: BaseDirectory.Home });
  } catch (err) {
    console.log("Could not write to file: " + fileName, err)
  }
}

export const readFile = async (fileName: string) => {
  try {
    const fileExists = await exists(fileName, { baseDir: BaseDirectory.AppData });
    if (!fileExists) return;

    const fileContent = await readTextFile(
      fileName,
      { baseDir: BaseDirectory.AppData }
    );
    return fileContent;
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
