/**
 * Validation schemas for authentication
 *
 * Author: Elias Sjödin
 * Created: 2024-10-09
 */

import { z } from "zod";

/**
 * Password Reset Schemas
 */

const passwordSchema = z
  .string({ required_error: "New password is required" })
  .min(8, "Password must be at least 8 characters long")
  .regex(/[a-z]/, "Password must contain at least one lowercase letter")
  .regex(/[A-Z]/, "Password must contain at least one uppercase letter")
  .regex(/\d/, "Password must contain at least one number")
  .regex(/[\W_]/, "Password must contain at least one special character");

export const forgotPasswordSchema = z.object({
  confirmationCode: z
    .string({ required_error: "Code is required" })
    .regex(/^\d{6}$/, "Code must be exactly 6 digits"),
  newPassword: passwordSchema,
  confirmPassword: z.string({ required_error: "Confirm password is required" }),
})
.refine(data => data.newPassword === data.confirmPassword, {
  message: "Passwords do not match",
  path: ["confirmPassword"],
});

export const newPasswordRequiredSchema = z.object({
  newPassword: passwordSchema,
  confirmPassword: z.string({ required_error: "Confirm password is required" }),
})
.refine(data => data.newPassword === data.confirmPassword, {
  message: "Passwords do not match",
  path: ["confirmPassword"],
});

/**
 * Other Schemas
 */

export const usernameSchema = z.object({
  email: z
    .string({ required_error: "Please enter an email" })
    .email("Invalid email address"),
});

export const loginSchema = z.object({
  email: z
    .string({ required_error: "Please enter an email" })
    .email("Invalid email address"),
  password: z
    .string({ required_error: "Please enter a password" })
    .refine(val => val !== undefined && val !== null && val.length > 0, {
      message: "Please enter a password",
    }),
});

export const mfaCodeSchema = z.object({
  code: z
    .string({ required_error: "Code is required" })
    .regex(/^\d{6}$/, "Code must be exactly 6 digits"),
});
