import { configureAuthBuddy } from "$lib/services/auth-service";
import "../app.css";

export const prerender = true;
export const ssr = false;

configureAuthBuddy();
