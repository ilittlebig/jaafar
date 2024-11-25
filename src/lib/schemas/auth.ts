/**
 *
 *
 * Author: Elias Sjödin
 * Created: 2024-11-25
 */

import { z } from "zod";

export const loginFormSchema = z.object({
	username: z.string().min(2).max(50),
	password: z.string().min(2).max(50),
});

export const resetPasswordFormSchema = z.object({
	username: z.string(),
});

export type LoginFormSchema = typeof loginFormSchema;
export type ResetPasswordFormSchema = typeof resetPasswordFormSchema;
