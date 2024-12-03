/**
 *
 *
 * Author: Elias Sjödin
 * Created: 2024-12-03
 */

import { readFileJSON, writeFileJSON } from "$lib/services/files-service";
import {
	signupsStore,
	type Signup,
	type SignupMode,
	type SignupStatus
} from "$lib/stores/signups-store.svelte";

export const loadSignups = async () => {
  const signups = await readFileJSON("signups.json");
	if (!signups) return;
	signupsStore.signups = [ ...signups ];
}

export const addNewSignup = async ({ product, proxyGroup, mode }: Omit<Signup, "status">) => {
	let signups = await readFileJSON("signups.json");
  if (!signups) signups = [];

	const signup = {
		product,
		proxyGroup,
		mode: mode as SignupMode,
		status: "not_started" as SignupStatus,
	};

	signups = [...signups, signup];
  await writeFileJSON("signups.json", signups);
	signupsStore.signups = signups;
};

export const deleteSignup = async () => {
}
