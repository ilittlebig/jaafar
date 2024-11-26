/**
 *
 *
 * Author: Elias Sjödin
 * Created: 2024-11-26
 */

import type { AuthTOTPSetupDetails } from "@ilittlebig/easy-auth";

type TotpSetupDetails = { value: AuthTOTPSetupDetails | undefined };

export let usernameStore = $state({ value: "" });
export let totpSetupDetailsStore: TotpSetupDetails = $state({ value: undefined });
