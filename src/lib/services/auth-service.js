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
} from "auth-buddy";
import {
  PUBLIC_USER_POOL_ID,
  PUBLIC_USER_POOL_CLIENT_ID,
} from "$env/static/public";

/**
 *
 */

const handleNextStep = nextStep => {
  console.log(nextStep);
  switch (nextStep) {
  }
}

/**
 *
 */

export const handleSignIn = async ({ username, password }) => {
  const result = await signIn({ username, password });
  handleNextStep(result?.nextStep?.signInStep);
}

/**
 *
 */

export const handleResetPassword = async ({ username }) => {
  const result = await resetPassword({ username });
  handleNextStep(result?.nextStep?.resetPasswordStep);
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
