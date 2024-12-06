/**
 *
 *
 * Author: Elias Sjödin
 * Created: 2024-11-28
 */

export type Settings = { [key: string]: Record<string, any> };

export let settingsStore: Settings = $state({
	integration: {
		captcha_solver: undefined,
		captcha_solver_api_key: undefined,
		sms_verifier: undefined,
		sms_verifier_api_key: undefined,
		request_delay: 3000,
		max_request_retries: 3,
		entry_limit: 10,
		imap_email: undefined,
		imap_password: undefined,
		webhook: undefined,
	},
});

export let settingsLoaded = $state({ value: false });
