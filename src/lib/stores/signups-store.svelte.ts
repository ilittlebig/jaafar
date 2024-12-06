/**
 *
 *
 * Author: Elias Sjödin
 * Created: 2024-12-02
 */

export type SignupStatus = "not_started" | "running" | "completed";
export type SignupMode = "auto_confirm" | "manual_confirm";

export interface Signup {
	status: SignupStatus;
	product: string;
	proxyGroup: string;
	mode: SignupMode;
}

export interface SignupsStore {
	signups: Signup[];
}

export let signupsStore: SignupsStore = $state({
	signups: [],
});
