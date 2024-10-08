/**
 * Handles user authentication
 *
 * Author: Elias Sjödin
 * Created: 2024-10-06
 */

import {
  AuthBuddy,
  signIn,
  resetPassword,
  confirmResetPassword,
} from "auth-buddy";
import {
  PUBLIC_USER_POOL_ID,
  PUBLIC_USER_POOL_CLIENT_ID,
} from "$env/static/public";

/**
 *
 */

export const handleSignIn = async ({ username, password }) => {
  const result = await signIn({ username, password });
  return result?.nextStep?.signInStep;
}

/**
 *
 */

export const handleResetPassword = async ({ username }) => {
  const result = await resetPassword({ username });
  return result?.nextStep?.resetPasswordStep;
}

/**
 *
 */

export const handleConfirmResetPassword = async ({ username, confirmationCode, newPassword }) => {
  await confirmResetPassword({
    username,
    confirmationCode,
    newPassword,
  });
  return "DONE";
}

/**
 *
 */

export const configureAuthBuddy = () => {
  AuthBuddy.configure({
    Auth: {
      Cognito: {
        userPoolId: PUBLIC_USER_POOL_ID,
        userPoolClientId: PUBLIC_USER_POOL_CLIENT_ID
      },
    }
  });
}
