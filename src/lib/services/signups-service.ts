/**
 *
 *
 * Author: Elias Sjödin
 * Created: 2024-12-03
 */

import { v4 as uuidv4 } from 'uuid';
import { readFileJSON, writeFileJSON } from "$lib/services/files-service";
import {
	signupsStore,
	type Signup,
	type SignupMode,
	type SignupStatus
} from "$lib/stores/signups-store.svelte";

const FILE_PATH = "signups.json";

export const loadSignups = async () => {
  const signups = await readFileJSON(FILE_PATH);
	if (!signups) return;
	signupsStore.signups = [ ...signups ];
}

export const addNewSignup = async ({ product, proxyGroup, mode }: Omit<Signup, "status">) => {
	let signups = await readFileJSON(FILE_PATH);
  if (!signups) signups = [];

	const signup = {
		id: uuidv4(),
		product,
		proxyGroup,
		mode: mode as SignupMode,
		status: "not_started" as SignupStatus,
	};

	signups = [...signups, signup];
  await writeFileJSON(FILE_PATH, signups);
	signupsStore.signups = signups;
};

export const deleteSignup = async (id: string) => {
  let signups = await readFileJSON(FILE_PATH);
  if (!signups || !Array.isArray(signups)) {
    throw new Error("Unable to load signups data");
  }

  const signupIndex = signups.findIndex((signup: Signup) => signup.id === id);
  if (signupIndex === -1) {
    throw new Error(`Signup with ID "${id}" does not exist`);
  }

  const signupBackup = signups[signupIndex];

  try {
    signups.splice(signupIndex, 1);
    await writeFileJSON(FILE_PATH, signups);
    signupsStore.signups = signups;
  } catch {
    if (signupBackup) {
      signups.splice(signupIndex, 0, signupBackup);
      await writeFileJSON(FILE_PATH, signups);
    }
    throw new Error("Failed to delete signup. Rollback attempted.");
  }
};
