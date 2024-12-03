/**
 *
 *
 * Author: Elias Sjödin
 * Created: 2024-12-02
 */

import { z } from "zod";

export const newSignupFormSchema = z.object({
	product: z.string().nonempty("Product is required"),
	proxyGroup: z.string().nonempty("Proxy group is required"),
	mode: z.string().nonempty("Mode is required"),
});
