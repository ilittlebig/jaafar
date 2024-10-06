/**
 * Wrapper for Tauri's HTTP API
 * Checks if the user's session is valid automatically before doing any requests
 */

import { fetch } from "@tauri-apps/plugin-http";
//import { getCurrentSession } from "auth-buddy";
//import { getEnvironmentVariables } from "$lib/utils/environment";

const request = async (method, path, body) => {
  //const session = await getCurrentSession();
  //const accessToken = session.tokens.accessToken.toString();

  const { API_URL } = getEnvironmentVariables();
  const response = await fetch(API_URL + path, {
    method,
    headers: { Authorizationtoken: accessToken },
    body: JSON.stringify(body)
  });
  return await response.json();
}

export const GET = async path => await request("GET", path);
export const POST = async (path, body) => await request("POST", path, body);
export const DELETE = async path => await request("DELETE", path);;
export const PATCH = async (path, body) => await request("PATCH", path, body);;
