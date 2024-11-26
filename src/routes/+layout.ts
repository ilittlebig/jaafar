import { goto } from "$app/navigation";
import { Hub } from "@ilittlebig/easy-auth";
import { configureEasyAuth } from "$lib/services/auth-service";
import "../app.css";

export const prerender = true;
export const ssr = false;

configureEasyAuth();

Hub.listen("auth", ({ payload }) => {
	switch (payload.event) {
		case "signedIn":
			goto("/");
			break;
		case "signedOut":
			goto("/login");
			break;
	}
});
