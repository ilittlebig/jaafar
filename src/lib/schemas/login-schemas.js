import { z } from "zod";

export const loginSchema = z.object({
  email: z
    .string({ required_error: "Please enter an email" })
    .email("Invalid email address"),
  password: z
    .string({ required_error: "Please enter a password" })
    .refine(val => {
      return val !== undefined && val !== null && val.length > 0;
    }, { message: "Please enter a password" }),
})
