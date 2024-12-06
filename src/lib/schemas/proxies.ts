/**
 *
 *
 * Author: Elias Sjödin
 * Created: 2024-12-02
 */

import { z } from "zod";

const isValidFilePath = (path: string): boolean => {
	return path.startsWith("/") || /^[a-zA-Z]:\\/.test(path);
};

export const addProxyGroupFormSchema = z.object({
	name: z
		.string({ required_error: "Name is required" })
		.min(3, "Name has to be at least 3 characters"),
	file: z
		.string({ required_error: "File is required" })
		.refine(path => isValidFilePath(path), {
			message: "Invalid file path",
		}),
});
