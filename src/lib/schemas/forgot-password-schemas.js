import { z } from "zod";

export const usernameSchema = z.object({
  email: z
    .string({ required_error: "Please enter an email" })
    .email("Invalid email address"),
})
