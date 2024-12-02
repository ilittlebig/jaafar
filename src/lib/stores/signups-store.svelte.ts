/**
 *
 *
 * Author: Elias Sjödin
 * Created: 2024-12-02
 */

export interface Signup {
	status: string;
	product: string;
	proxyGroup: string;
	amountOfAccounts: number;
	mode: "auto_confirm" | "manual_confirm";
}

export interface SignupsStore {
	signups: Signup[];
}

export let signupsStore: SignupsStore = $state({
	signups: [],
});
