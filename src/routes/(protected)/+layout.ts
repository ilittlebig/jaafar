import { redirect } from "@sveltejs/kit";

const isAuthenticated = async () => {
  return false;
}

export const load = async () => {
  const authenticated = await isAuthenticated();
	if (!authenticated) redirect(307, "/login");
}
