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
	resetPassword,
	confirmResetPassword,
	type SignInInput,
	type ResetPasswordInput,
	type ConfirmResetPasswordInput,
} from "@ilittlebig/easy-auth";
import { usernameStore } from "$lib/stores/auth-store.svelte";
import { resetPasswordVerificationDialog } from "$lib/components/dialogs/auth/reset-password-verification-dialog.svelte";
import { resetPasswordDialog } from "$lib/components/dialogs/auth/reset-password-dialog.svelte";
import { totpCodeDialog } from "$lib/components/dialogs/auth/totp-code-dialog.svelte";

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
		*/
    case "CONFIRM_SIGN_IN_WITH_TOTP_CODE":
      totpCodeDialog.open = true;
      break;
    case "RESET_PASSWORD":
    case "CONFIRM_RESET_PASSWORD_WITH_CODE":
			resetPasswordDialog.open = false;
      resetPasswordVerificationDialog.open = true;
      break;
    case "DONE":
			/*
      mfaSetupDialogOpen.set(false);
      newPasswordRequiredDialogOpen.set(false);
			*/
			resetPasswordVerificationDialog.open = false;
      totpCodeDialog.open = false;
      break;
    default:
      // TODO: should never happen
      break;
  }
}

export const handleSignIn = async ({ username, password }: SignInInput) => {
  const result = await signIn({ username, password });
	usernameStore.value = username;
  handleNextStep(result?.nextStep?.signInStep, result);
}

export const handleResetPassword = async ({ username }: ResetPasswordInput) => {
  const result = await resetPassword({ username });
	usernameStore.value = username;
  handleNextStep(result?.nextStep?.resetPasswordStep);
}

export const handleConfirmResetPassword = async ({
  confirmationCode,
  newPassword
}: ConfirmResetPasswordInput) => {
  await confirmResetPassword({
    username: usernameStore.value,
    confirmationCode,
    newPassword,
  });
  handleNextStep("DONE");
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
