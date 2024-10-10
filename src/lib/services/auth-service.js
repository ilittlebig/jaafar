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
  confirmSignIn,
} from "auth-buddy";
import {
  PUBLIC_USER_POOL_ID,
  PUBLIC_USER_POOL_CLIENT_ID,
} from "$env/static/public";
import { totpSetupDetails } from "$lib/stores/auth-store";
import { mfaSetupDialogOpen } from "$lib/components/dialogs/auth-dialogs/mfa-setup-dialog";
import { forgotPasswordStep2Open } from "$lib/components/dialogs/auth-dialogs/forgot-password-dialog";
import { mfaCodeDialogOpen } from "$lib/components/dialogs/auth-dialogs/mfa-code-dialog";
import { newPasswordRequiredDialogOpen } from "$lib/components/dialogs/auth-dialogs/new-password-required-dialog";

/**
 *
 */

export const handleNextStep = (nextStep, result) => {
  console.log("next step:", nextStep, result); // TODO: remove this line
  switch (nextStep) {
    case "CONTINUE_SIGN_IN_WITH_TOTP_SETUP":
      const { totpSetupDetails: setupDetails } = result?.nextStep;
      totpSetupDetails.set(setupDetails);
      mfaSetupDialogOpen.set(true);
      break;
    case "CONTINUE_SIGN_IN_WITH_MFA_SELECTION": break;
    case "CONFIRM_SIGN_IN_WITH_NEW_PASSWORD_REQUIRED":
      newPasswordRequiredDialogOpen.set(true);
      break;
    case "CONFIRM_SIGN_IN_WITH_TOTP_CODE":
      mfaCodeDialogOpen.set(true);
      break;
    case "RESET_PASSWORD":
    case "CONFIRM_RESET_PASSWORD_WITH_CODE":
      forgotPasswordStep2Open.set(true);
      break;
    case "DONE":
      mfaSetupDialogOpen.set(false);
      newPasswordRequiredDialogOpen.set(false);
      mfaCodeDialogOpen.set(false);
      forgotPasswordStep2Open.set(false);
      break;
  }
}

/**
 *
 */

export const handleChallengeResponse = async ({ response }) => {
  const result = await confirmSignIn({ challengeResponse: response });
  handleNextStep(result?.nextStep?.signInStep, result);
}

/**
 *
 */

export const handleSignIn = async ({ username, password }) => {
  const result = await signIn({ username, password });
  handleNextStep(result?.nextStep?.signInStep, result);
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

export const handleConfirmResetPassword = async ({ username, confirmationCode, newPassword }) => {
  await confirmResetPassword({
    username,
    confirmationCode,
    newPassword,
  });
  handleNextStep("DONE");
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
