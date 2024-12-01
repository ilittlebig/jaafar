/**
 *
 *
 * Author: Elias Sjödin
 * Created: 2024-11-30
 */

import { readFile, writeFileJSON } from "$lib/services/files-service";
import { accountsStore, type Account } from "$lib/stores/accounts-store.svelte";
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

const validateAccount = (account: Account) => {
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
	let accounts: Account[];
  try {
    accounts = await csvToJson(csvContent);
  } catch (err) {
    throw new Error("Invalid CSV format. Ensure the file is properly formatted.");
  }
	return accounts;
}

const validateAccounts = (accounts: Account[]) => {
  const validationErrors: Record<number, string[]> = [];
  accounts.forEach((account: Account, index: number) => {
		const errors = validateAccount(account);
    if (errors.length > 0) {
      validationErrors[index + 1] = errors;
    }
  });
	return validationErrors;
};

export const loadAccounts = () => {
}

export const importAccounts = async (filePath: string): Promise<Record<number, string[]>> => {
	const csvContent = await readCsvContent(filePath);
	const accounts = await parseCsvToJson(csvContent);
	const validationErrors = validateAccounts(accounts);

	if (Object.keys(validationErrors).length === 0) {
		await writeFileJSON("accounts.json", accounts);
		accounts.forEach((account: Account) => accountsStore.push(account));
	}

	return validationErrors;
};
