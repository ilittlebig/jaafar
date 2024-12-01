/**
 *
 *
 * Author: Elias Sjödin
 * Created: 2024-11-30
 */

import { readFile, writeFileJSON } from "$lib/services/files-service";
import { csvToJson } from "$lib/utils";

const EXPECTED_KEYS = [
  "email",
  "firstname",
  "lastname",
  "phone",
  "address1",
  "address2",
  "city",
  "postcode",
  "country",
];

const validateAccount = (account: any) => {
  const errors: string[] = [];
  EXPECTED_KEYS.forEach(key => {
    if (!(key in account)) errors.push(`Missing key: ${key}`);
  });

  if (account.email && !/\S+@\S+\.\S+/.test(account.email)) {
    errors.push(`Invalid email format: ${account.email}`);
  }
  if (account.country && account.country.length !== 2) {
    errors.push(`Invalid country code (must be 2 letters): ${account.country}`);
  }

  return errors;
};

const readCsvContent = async (filePath: string) => {
  const csvContent = await readFile(filePath);
  if (!csvContent || typeof csvContent !== "string") {
    throw new Error("Something went wrong when reading the file. Try again!");
  }
	return csvContent;
}

const parseCsvToJson = async (csvContent: string) => {
	let csvJson: any;
  try {
    csvJson = await csvToJson(csvContent);
  } catch (err) {
    throw new Error("Invalid CSV format. Ensure the file is properly formatted.");
  }
	return csvJson;
}

const validateCsvData = (csvJson: any) => {
  const validationErrors: Record<number, string[]> = [];
  csvJson.forEach((account: any, index: number) => {
		const errors = validateAccount(account);
    if (errors.length > 0) {
      validationErrors[index + 1] = errors;
    }
  });
	return validationErrors;
};

export const importAccounts = async (filePath: string): Promise<Record<number, string[]>> => {
	const csvContent = await readCsvContent(filePath);
	const csvJson = await parseCsvToJson(csvContent);
	const validationErrors = validateCsvData(csvJson);

  await writeFileJSON("accounts.json", csvJson);
	// TODO: update accounts store

	return validationErrors;
};
