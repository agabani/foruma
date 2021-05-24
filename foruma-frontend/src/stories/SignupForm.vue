<template>
  <div class="signup-form">
    <BaseSplashPanel>
      <template v-slot:splash>
        <div class="signup-form--splash">
          <h1>WEL</h1>
          <h1>COME</h1>
          <h1>_</h1>
        </div>
      </template>
      <template v-slot>
        <div>
          <form @submit="onSubmit">
            <h1 class="signup-form--header">Create your account</h1>
            <BaseAlert
              type="warning"
              :title="alertTitle"
              :message="alertMessage"
              :eventDate="alertEventDate"
            ></BaseAlert>
            <div class="signup-form--label">Username</div>
            <BaseTextField
              :fullWidth="true"
              :value="dataUsername"
              @change="updateUsername"
            />
            <div class="signup-form--label">Password</div>
            <BasePasswordField :fullWidth="true" @change="updatePassword" />
            <div class="signup-form--label">Confirm Password</div>
            <BasePasswordField
              :fullWidth="true"
              @change="updateConfirmPassword"
            />
            <div class="signup-form--button">
              <BaseButton label="Signup" :fullWidth="true" :primary="true" />
            </div>
          </form>
        </div>
      </template>
    </BaseSplashPanel>
  </div>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import BaseAlert from "./BaseAlert.vue";
import BaseButton from "./BaseButton.vue";
import BasePasswordField from "./BasePasswordField.vue";
import BaseSplashPanel from "./BaseSplashPanel.vue";
import BaseTextField from "./BaseTextField.vue";
import { Change as BasePasswordFieldChange } from "./BasePasswordField.vue";
import { Change as BaseTextFieldChange } from "./BaseTextField.vue";

export default defineComponent({
  name: "SignupForm",
  components: {
    BaseAlert,
    BaseButton,
    BasePasswordField,
    BaseSplashPanel,
    BaseTextField,
  },
  props: {
    username: {
      type: String,
      default: "",
    },
  },
  data() {
    return {
      dataUsername: this.username,
      dataPassword: "",
      dataConfirmPassword: "",
      alertEventDate: undefined as Date | undefined,
      alertMessage: "Sorry! There was a problem with your request!",
      alertTitle: "Uh oh, something went wrong",
    };
  },
  methods: {
    onSubmit(event: { preventDefault: () => void }): void {
      event.preventDefault();
      if (
        !this.dataUsername ||
        !this.dataPassword ||
        !this.dataConfirmPassword
      ) {
        this.alertTitle = "Sorry! There was a problem with your request!";
        this.alertMessage =
          "All fields needs to be filled in order to create an account!";
        this.alertEventDate = new Date();
        return;
      }

      if (this.dataPassword !== this.dataConfirmPassword) {
        this.alertTitle = "Sorry! There was a problem with your request!";
        this.alertMessage = "Make sure your passwords match then try again!";
        this.alertEventDate = new Date();
        return;
      }

      if (
        this.dataUsername &&
        this.dataPassword &&
        this.dataConfirmPassword &&
        this.dataPassword === this.dataConfirmPassword
      ) {
        this.$emit("submit", {
          username: this.dataUsername,
          password: this.dataPassword,
        });
      }
    },
    updateUsername(value: BaseTextFieldChange): void {
      this.dataUsername = value.newValue;
    },
    updatePassword(value: BasePasswordFieldChange): void {
      this.dataPassword = value.newValue;
    },
    updateConfirmPassword(value: BasePasswordFieldChange): void {
      this.dataConfirmPassword = value.newValue;
    },
  },
});
</script>

<style lang="css" scoped>
.signup-form {
}

.signup-form--splash h1 {
  display: block;

  color: white;
  font-size: 4em;
}

.signup-form--header {
  display: block;

  margin-bottom: 2em;
}

.signup-form--label {
  margin: 0em 1em;

  font-size: 14px;
  font-weight: 700;
}

.signup-form--button {
  margin-top: 4em;
}
</style>
