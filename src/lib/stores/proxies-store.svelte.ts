/**
 *
 *
 * Author: Elias Sjödin
 * Created: 2024-12-01
 */

export interface ProxyGroup {
	name: string;
	file: string;
	originalFilePath: string;
	amount: number;
}

export interface ProxyStore {
	groups: ProxyGroup[];
}

export let proxiesStore: ProxyStore = $state({
	groups: [],
});
