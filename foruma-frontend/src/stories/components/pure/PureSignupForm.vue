<template>
  <div class="pure-signup-form">
    <BaseSplashPanel>
      <template v-slot:splash>
        <div class="pure-signup-form--splash">
          <h1>WEL</h1>
          <h1>COME</h1>
          <h1>_</h1>
        </div>
      </template>
      <template v-slot>
        <div>
          <form @submit="onSubmit">
            <h1 class="pure-signup-form--header">Create your account</h1>
            <BaseAlert
              type="warning"
              :title="alertTitle"
              :message="alertMessage"
              :eventDate="alertEventDate"
            ></BaseAlert>
            <div class="pure-signup-form--label">Username</div>
            <BaseTextField
              :fullWidth="true"
              :value="dataUsername"
              @change="updateUsername"
            />
            <div class="pure-signup-form--label">Password</div>
            <BasePasswordField :fullWidth="true" @change="updatePassword" />
            <div class="pure-signup-form--label">Confirm Password</div>
            <BasePasswordField
              :fullWidth="true"
              @change="updateConfirmPassword"
            />
            <div class="pure-signup-form--button">
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
import BaseAlert from "../../base/BaseAlert.vue";
import BaseButton from "../../base/BaseButton.vue";
import BasePasswordField from "../../base/BasePasswordField.vue";
import BaseSplashPanel from "../../base/BaseSplashPanel.vue";
import BaseTextField from "../../base/BaseTextField.vue";
import { BasePasswordFieldEventChange } from "../../base/BasePasswordField.types";
import { BaseTextFieldEventChange } from "../../base/BaseTextField.types";
import {
  pureSignupFormEvents,
  PureSignupFormEventSubmit,
} from "./PureSignupForm.types";

export default defineComponent({
  name: "PureSignupForm",
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
    alertEventDate: {
      type: Date,
    },
    alertMessage: {
      type: String,
      default: "",
    },
    alertTitle: {
      type: String,
      default: "",
    },
  },
  data() {
    return {
      dataUsername: this.username,
      dataPassword: "",
      dataConfirmPassword: "",
    };
  },
  emits: [...pureSignupFormEvents],
  methods: {
    onSubmit(event: { preventDefault: () => void }): void {
      event.preventDefault();
      this.$emit("submitted", {
        username: this.dataUsername,
        password: this.dataPassword,
        confirmPassword: this.dataConfirmPassword,
      } as PureSignupFormEventSubmit);
    },
    updateUsername(value: BasePasswordFieldEventChange): void {
      this.dataUsername = value.newValue;
    },
    updatePassword(value: BaseTextFieldEventChange): void {
      this.dataPassword = value.newValue;
    },
    updateConfirmPassword(value: BaseTextFieldEventChange): void {
      this.dataConfirmPassword = value.newValue;
    },
  },
});
</script>

<style lang="css" scoped>
.pure-signup-form {
}

.pure-signup-form--splash h1 {
  display: block;

  color: white;
  font-size: 4em;
}

.pure-signup-form--header {
  display: block;

  margin-bottom: 2em;
}

.pure-signup-form--label {
  margin: 0em 1em;

  font-size: 14px;
  font-weight: 700;
}

.pure-signup-form--button {
  margin-top: 4em;
}
</style>
