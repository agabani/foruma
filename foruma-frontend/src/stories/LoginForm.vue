<template>
  <div class="login-form">
    <div class="login-form--splash">
      <h1 class="login-form--greeting">WEL</h1>
      <h1 class="login-form--greeting">COME</h1>
      <h1 class="login-form--greeting">_</h1>
    </div>
    <div class="login-form--panel">
      <h1 class="login-form--header">Login to your Account</h1>
      <form @submit="onSubmit">
        <div class="login-form--label">Username</div>
        <div class="login-form--field">
          <BaseTextField
            :fullWidth="true"
            @change="updateUsername"
            :initialValue="username"
          />
        </div>
        <div class="login-form--label">Password</div>
        <div class="login-form--field">
          <BasePasswordField :fullWidth="true" @change="updatePassword" />
        </div>
        <div class="login-form--button">
          <BaseButton label="Login" :primary="true" :fullWidth="true" />
        </div>
      </form>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import BaseButton from "./BaseButton.vue";
import BasePasswordField from "./BasePasswordField.vue";
import BaseTextField from "./BaseTextField.vue";

export default defineComponent({
  name: "LoginForm",
  components: {
    BaseButton,
    BasePasswordField,
    BaseTextField,
  },
  props: {
    initialUsername: {
      type: String,
      default: "",
    },
  },
  emits: ["submit"],
  data() {
    return {
      username: this.initialUsername,
      password: "",
    };
  },
  methods: {
    onSubmit(event): void {
      event.preventDefault();
      this.$emit("submit", {
        username: this.username,
        password: this.password,
      });
    },
    updateUsername(value: { newValue: string; oldValue: string }): void {
      this.username = value.newValue;
    },
    updatePassword(value: { newValue: string; oldValue: string }): void {
      this.password = value.newValue;
    },
  },
});
</script>

<style lang="css" scoped>
.login-form {
  display: flex;
  justify-content: flex-end;
  align-items: center;

  max-width: 960px;

  font-family: Helvetica, Verdana, Arial;
}

.login-form--splash {
  flex: auto;
  align-self: stretch;

  padding: 4em 3em;

  background-color: #00b7ff;
  background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='540' height='450' viewBox='0 0 1080 900'%3E%3Cg fill-opacity='.1'%3E%3Cpolygon fill='%23444' points='90 150 0 300 180 300'/%3E%3Cpolygon points='90 150 180 0 0 0'/%3E%3Cpolygon fill='%23AAA' points='270 150 360 0 180 0'/%3E%3Cpolygon fill='%23DDD' points='450 150 360 300 540 300'/%3E%3Cpolygon fill='%23999' points='450 150 540 0 360 0'/%3E%3Cpolygon points='630 150 540 300 720 300'/%3E%3Cpolygon fill='%23DDD' points='630 150 720 0 540 0'/%3E%3Cpolygon fill='%23444' points='810 150 720 300 900 300'/%3E%3Cpolygon fill='%23FFF' points='810 150 900 0 720 0'/%3E%3Cpolygon fill='%23DDD' points='990 150 900 300 1080 300'/%3E%3Cpolygon fill='%23444' points='990 150 1080 0 900 0'/%3E%3Cpolygon fill='%23DDD' points='90 450 0 600 180 600'/%3E%3Cpolygon points='90 450 180 300 0 300'/%3E%3Cpolygon fill='%23666' points='270 450 180 600 360 600'/%3E%3Cpolygon fill='%23AAA' points='270 450 360 300 180 300'/%3E%3Cpolygon fill='%23DDD' points='450 450 360 600 540 600'/%3E%3Cpolygon fill='%23999' points='450 450 540 300 360 300'/%3E%3Cpolygon fill='%23999' points='630 450 540 600 720 600'/%3E%3Cpolygon fill='%23FFF' points='630 450 720 300 540 300'/%3E%3Cpolygon points='810 450 720 600 900 600'/%3E%3Cpolygon fill='%23DDD' points='810 450 900 300 720 300'/%3E%3Cpolygon fill='%23AAA' points='990 450 900 600 1080 600'/%3E%3Cpolygon fill='%23444' points='990 450 1080 300 900 300'/%3E%3Cpolygon fill='%23222' points='90 750 0 900 180 900'/%3E%3Cpolygon points='270 750 180 900 360 900'/%3E%3Cpolygon fill='%23DDD' points='270 750 360 600 180 600'/%3E%3Cpolygon points='450 750 540 600 360 600'/%3E%3Cpolygon points='630 750 540 900 720 900'/%3E%3Cpolygon fill='%23444' points='630 750 720 600 540 600'/%3E%3Cpolygon fill='%23AAA' points='810 750 720 900 900 900'/%3E%3Cpolygon fill='%23666' points='810 750 900 600 720 600'/%3E%3Cpolygon fill='%23999' points='990 750 900 900 1080 900'/%3E%3Cpolygon fill='%23999' points='180 0 90 150 270 150'/%3E%3Cpolygon fill='%23444' points='360 0 270 150 450 150'/%3E%3Cpolygon fill='%23FFF' points='540 0 450 150 630 150'/%3E%3Cpolygon points='900 0 810 150 990 150'/%3E%3Cpolygon fill='%23222' points='0 300 -90 450 90 450'/%3E%3Cpolygon fill='%23FFF' points='0 300 90 150 -90 150'/%3E%3Cpolygon fill='%23FFF' points='180 300 90 450 270 450'/%3E%3Cpolygon fill='%23666' points='180 300 270 150 90 150'/%3E%3Cpolygon fill='%23222' points='360 300 270 450 450 450'/%3E%3Cpolygon fill='%23FFF' points='360 300 450 150 270 150'/%3E%3Cpolygon fill='%23444' points='540 300 450 450 630 450'/%3E%3Cpolygon fill='%23222' points='540 300 630 150 450 150'/%3E%3Cpolygon fill='%23AAA' points='720 300 630 450 810 450'/%3E%3Cpolygon fill='%23666' points='720 300 810 150 630 150'/%3E%3Cpolygon fill='%23FFF' points='900 300 810 450 990 450'/%3E%3Cpolygon fill='%23999' points='900 300 990 150 810 150'/%3E%3Cpolygon points='0 600 -90 750 90 750'/%3E%3Cpolygon fill='%23666' points='0 600 90 450 -90 450'/%3E%3Cpolygon fill='%23AAA' points='180 600 90 750 270 750'/%3E%3Cpolygon fill='%23444' points='180 600 270 450 90 450'/%3E%3Cpolygon fill='%23444' points='360 600 270 750 450 750'/%3E%3Cpolygon fill='%23999' points='360 600 450 450 270 450'/%3E%3Cpolygon fill='%23666' points='540 600 630 450 450 450'/%3E%3Cpolygon fill='%23222' points='720 600 630 750 810 750'/%3E%3Cpolygon fill='%23FFF' points='900 600 810 750 990 750'/%3E%3Cpolygon fill='%23222' points='900 600 990 450 810 450'/%3E%3Cpolygon fill='%23DDD' points='0 900 90 750 -90 750'/%3E%3Cpolygon fill='%23444' points='180 900 270 750 90 750'/%3E%3Cpolygon fill='%23FFF' points='360 900 450 750 270 750'/%3E%3Cpolygon fill='%23AAA' points='540 900 630 750 450 750'/%3E%3Cpolygon fill='%23FFF' points='720 900 810 750 630 750'/%3E%3Cpolygon fill='%23222' points='900 900 990 750 810 750'/%3E%3Cpolygon fill='%23222' points='1080 300 990 450 1170 450'/%3E%3Cpolygon fill='%23FFF' points='1080 300 1170 150 990 150'/%3E%3Cpolygon points='1080 600 990 750 1170 750'/%3E%3Cpolygon fill='%23666' points='1080 600 1170 450 990 450'/%3E%3Cpolygon fill='%23DDD' points='1080 900 1170 750 990 750'/%3E%3C/g%3E%3C/svg%3E");
  border-radius: 0.5em 0 0 0.5em;
  box-shadow: rgba(0, 0, 0, 0.15) 0 0 0 1px inset;
}

.login-form--greeting {
  display: block;

  color: white;
  font-size: 4em;
}

.login-form--panel {
  padding: 4em 2em;
  max-width: 280px;

  background-color: white;
  border-radius: 0 0.5em 0.5em 0;
  box-shadow: rgba(0, 0, 0, 0.15) 0 0 0 1px inset;
}

.login-form--header {
  margin-bottom: 2em;
}

.login-form--label {
  margin: 0.5em 0;

  font-size: 14px;
  font-weight: 700;
}

.login-form--field {
  margin: 0.5em 0;
}

.login-form--button {
  margin: 4em 2em 0;
}
</style>
