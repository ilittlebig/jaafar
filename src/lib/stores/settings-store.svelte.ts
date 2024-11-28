/**
 *
 *
 * Author: Elias Sjödin
 * Created: 2024-11-28
 */

export let settingsStore: { [key: string]: Record<string, any> } = $state({
	integration: {
		captcha_solver: undefined,
		captcha_solver_api_key: undefined,
		request_delay: 3000,
		entry_limit: 10,
		imap_email: undefined,
		imap_password: undefined,
		webhook: undefined,
	},
});
