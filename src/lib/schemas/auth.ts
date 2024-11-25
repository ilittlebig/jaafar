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
  username: z
    .string({ required_error: "Please enter an email" })
    .email("Invalid email address"),
});
