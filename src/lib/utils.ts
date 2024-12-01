import { type ClassValue, clsx } from "clsx";
import { twMerge } from "tailwind-merge";
import Papa from "papaparse";

type Obj = { [key: string]: Record<string, any> };

export const cn = (...inputs: ClassValue[]) => {
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

export const csvToJson = async (csvContent: string): Promise<any> => {
	const result = Papa.parse(csvContent, {
		header: true,
		skipEmptyLines: true,
	});

	if (result.errors.length > 0) {
		throw new Error("Error parsing CSV: " + JSON.stringify(result.errors));
	}
	return result.data;
};
