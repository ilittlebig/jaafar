import { type ClassValue, clsx } from "clsx";
import { twMerge } from "tailwind-merge";

type Obj = { [key: string]: Record<string, any> };

export function cn(...inputs: ClassValue[]) {
	return twMerge(clsx(inputs));
}

export const deepMerge = (target: Obj, source: Obj) => {
	for (const key in source) {
		if (source[key] && typeof source[key] === "object" && !Array.isArray(source[key])) {
			if (!target[key] || typeof target[key] !== "object") {
				target[key] = {};
			}
			deepMerge(target[key], source[key]);
		} else {
			target[key] = source[key];
		}
	}
	return target;
}
