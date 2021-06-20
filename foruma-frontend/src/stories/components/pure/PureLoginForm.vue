<template>
  <div class="pure-login-form">
    <BaseSplashPanel>
      <template v-slot:splash>
        <div class="pure-login-form--splash">
          <h1>WEL</h1>
          <h1>COME</h1>
          <h1>_</h1>
        </div>
      </template>
      <template v-slot>
        <div>
          <form @submit="onSubmit">
            <h1 class="pure-login-form--header">Login to your Account</h1>
            <BaseAlert
              type="warning"
              :title="alertTitle"
              :message="alertMessage"
              :eventDate="alertEventDate"
            ></BaseAlert>
            <div class="pure-login-form--label">Username</div>
            <BaseTextField
              :fullWidth="true"
              :value="dataUsername"
              @change="updateUsername"
            />
            <div class="pure-login-form--label">Password</div>
            <BasePasswordField :fullWidth="true" @change="updatePassword" />
            <div class="pure-login-form--button">
              <BaseButton label="Login" :fullWidth="true" :primary="true" />
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
  pureLoginFormEvents,
  PureLoginFormEventSubmit,
} from "./PureLoginForm.types";

export default defineComponent({
  name: "PureLoginForm",
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
    };
  },
  emits: [...pureLoginFormEvents],
  methods: {
    onSubmit(event: { preventDefault: () => void }): void {
      event.preventDefault();
      const submit: PureLoginFormEventSubmit = {
        username: this.dataUsername,
        password: this.dataPassword,
      };

      this.$emit("submitted", submit);
    },
    updateUsername(value: BasePasswordFieldEventChange): void {
      this.dataUsername = value.newValue;
    },
    updatePassword(value: BaseTextFieldEventChange): void {
      this.dataPassword = value.newValue;
    },
  },
});
</script>

<style lang="css" scoped>
.pure-login-form {
}

.pure-login-form--splash h1 {
  display: block;

  color: white;
  font-size: 4em;
}

.pure-login-form--header {
  display: block;

  margin-bottom: 2em;
}

.pure-login-form--label {
  margin: 0em 1em;

  font-size: 14px;
  font-weight: 700;
}

.pure-login-form--button {
  margin-top: 4em;
}
</style>
