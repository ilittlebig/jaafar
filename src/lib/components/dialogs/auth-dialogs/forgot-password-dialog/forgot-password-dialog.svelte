<script>
  import { writable } from "svelte/store";
  import { setError } from "sveltekit-superforms";
  import { handleResetPassword, handleConfirmResetPassword } from "$lib/services/auth-service";
  import { Button } from "$lib/components/ui/button";
  import { Input } from "$lib/components/ui/input";
  import * as Form from "$lib/components/ui/form";
  import * as Dialog from "$lib/components/ui/dialog";
  import Step1, { open as step1Open } from "./step1.svelte";
  import Step2, { open as step2Open } from "./step2.svelte";

  let username = writable(undefined);

  const handleValidFormStep1 = async form => {
    const formData = form.data;
    try {
      const result = await handleResetPassword({ username: formData.email });
      if (result === "CONFIRM_RESET_PASSWORD_WITH_CODE") {
        username.set(formData.email);
        step1Open.set(false);
        step2Open.set(true);
      }
    } catch (err) {
      setError(form, "email", err.message);
    }
  }

  const handleValidFormStep2 = async form => {
    const formData = form.data;
    try {
      const result = await handleConfirmResetPassword({
        username: $username,
        confirmationCode: formData.confirmationCode,
        newPassword: formData.newPassword,
      });
      console.log(result, $username);
      if (result === "DONE") step2Open.set(false);
    } catch (err) {
      console.log(err);
      setError(form, "email", err.message);
    }
  }
</script>

<Step1 onSubmit={handleValidFormStep1} />
<Step2 onSubmit={handleValidFormStep2} />
