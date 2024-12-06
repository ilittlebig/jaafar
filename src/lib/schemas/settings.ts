/**
 *
 *
 * Author: Elias Sjödin
 * Created: 2024-11-28
 */

import { z } from "zod";

const optionalString = (schema: z.ZodString) => {
  return z.preprocess(value => {
    if (typeof value === "string" && value.trim() === "") {
      return undefined;
    }
    return value;
  }, schema.optional());
}

const integrationSchema = z.object({
	captcha_solver: optionalString(z.string()),
	captcha_solver_api_key: optionalString(z.string()),
	sms_verifier: optionalString(z.string()),
	sms_verifier_api_key: optionalString(z.string()),
	request_delay: z.coerce
		.number({ invalid_type_error: "Request delay must be a valid number" })
		.min(1000, { message: "Request delay must be at least 1000ms" }),
	max_request_retries: z.coerce
		.number({ invalid_type_error: "Max request retries must be a valid number" })
		.min(0, { message: "Max request retries must be at least 0" }),
	entry_limit: z.coerce
		.number({ invalid_type_error: "Entry limit must be a valid number" })
		.min(1, { message: "Entry limit must be at least 1" }),
	imap_email: optionalString(z.string().email({ message: "Invalid email format" })),
	imap_password: optionalString(z.string()),
  webhook: optionalString(z.string().url({ message: "Webhook must be a valid URL" })),
})

export const settingsSchema = z.object({
  integration: integrationSchema,
});

export type SettingsSchema = z.infer<typeof settingsSchema>;
