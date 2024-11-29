/**
 *
 *
 * Author: Elias Sjödin
 * Created: 2024-11-28
 */

import { z } from "zod";

const integrationSchema = z.object({
	captcha_solver: z.string().optional(),
	captcha_solver_api_key: z.string().optional(),
	request_delay: z.coerce
		.number({ invalid_type_error: "Request delay must be a valid number" })
		.min(1000, { message: "Request delay must be at least 1000ms" }),
	entry_limit: z.coerce
		.number({ invalid_type_error: "Entry limit must be a valid number" })
		.min(1, { message: "Entry limit must be at least 1" }),
	imap_email: z
		.string()
		.email({ message: "Invalid email format" })
		.optional(),
	imap_password: z.string().optional(),
	webhook: z
		.string()
		.url({ message: "Webhook must be a valid URL" })
		.optional(),
});

export const settingsSchema = z.object({
  integration: integrationSchema,
});

export type SettingsSchema = z.infer<typeof settingsSchema>;
