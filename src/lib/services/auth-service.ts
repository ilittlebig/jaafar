/**
 *
 *
 * Author: Elias Sjödin
 * Created: 2024-11-25
 */

import {
	PUBLIC_USER_POOL_ID,
	PUBLIC_USER_POOL_CLIENT_ID,
} from "$env/static/public";
import {
	EasyAuth,
	signIn,
} from "@ilittlebig/easy-auth";
import { resetPasswordVerificationDialog } from "$lib/components/dialogs/auth/reset-password-verification-dialog.svelte";

export const handleNextStep = (nextStep: string = "", result?: any) => {
  console.log(nextStep);
  switch (nextStep) {
		/*
    case "CONTINUE_SIGN_IN_WITH_TOTP_SETUP":
      const setupDetails = result?.nextStep?.totpSetupDetails;
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
		*/
    case "RESET_PASSWORD":
    case "CONFIRM_RESET_PASSWORD_WITH_CODE":
      resetPasswordVerificationDialog.open = true;
      break;
    case "DONE":
			/*
      mfaSetupDialogOpen.set(false);
      mfaCodeDialogOpen.set(false);
      forgotPasswordStep2Open.set(false);
      newPasswordRequiredDialogOpen.set(false);
			*/
      break;
    default:
      // TODO: should never happen
      break;
  }
}

export const handleSignIn = async (username: string, password: string) => {
  const result = await signIn({ username, password });
	console.log(result);
  handleNextStep(result?.nextStep?.signInStep, result);
}

export const configureEasyAuth = () => {
	EasyAuth.configure({
		Auth: {
			Cognito: {
				userPoolId: PUBLIC_USER_POOL_ID,
				userPoolClientId: PUBLIC_USER_POOL_CLIENT_ID,
			}
		}
	});
}
