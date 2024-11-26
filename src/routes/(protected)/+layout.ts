import { redirect } from "@sveltejs/kit";
import { getCurrentSession } from "@ilittlebig/easy-auth";

const isAuthenticated = async () => {
	try {
		await getCurrentSession();
		return true;
	} catch {
		return false;
	}
}

export const load = async () => {
  const authenticated = await isAuthenticated();
	if (!authenticated) throw redirect(307, "/login");
}
