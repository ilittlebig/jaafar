/**
 * Stores related to authentication
 *
 * Author: Elias Sjödin
 * Created: 2024-10-10
 */

import { writable } from "svelte/store";

export const username = writable(undefined);
export const totpSetupDetails = writable(undefined);
