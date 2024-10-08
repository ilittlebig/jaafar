import { z } from "zod";

export const usernameSchema = z.object({
  email: z
    .string({ required_error: "Please enter an email" })
    .email("Invalid email address"),
});

export const confirmPasswordSchema = z
  .object({
    confirmationCode: z
      .string({ required_error: "Code is required" })
      .regex(/^\d{6}$/, "Code must be exactly 6 digits"),
      newPassword: z
        .string({ required_error: "New password is required" })
        .min(8, "Password must be at least 8 characters long")
        .regex(/[a-z]/, "Password must contain at least one lowercase letter")
        .regex(/[A-Z]/, "Password must contain at least one uppercase letter")
        .regex(/\d/, "Password must contain at least one number")
        .regex(/[\W_]/, "Password must contain at least one special character"),
    confirmPassword: z
      .string({ required_error: "Confirm password is required" }),
  })
  .refine((data) => data.newPassword === data.confirmPassword, {
    message: "Passwords do not match",
    path: ["confirmPassword"],
  });
