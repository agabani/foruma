<template>
  <div class="pure-header">
    <BaseHeader>
      <template v-slot:brand>
        <span class="pure-header--brand" @click="onBrand">Foruma</span>
      </template>
      <template v-slot>
        <BaseButton
          v-if="authenticated"
          size="small"
          :label="username"
          primary
          @click="onUsername"
        />
        <BaseButton
          v-if="authenticated"
          size="small"
          label="Log out"
          @click="onLogout"
        />
        <BaseButton
          v-if="!authenticated"
          size="small"
          label="Log in"
          @click="onLogin"
        />
        <BaseButton
          v-if="!authenticated"
          size="small"
          label="Sign up"
          primary
          @click="onSignup"
        />
      </template>
    </BaseHeader>
  </div>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import { pureHeaderEvents } from "./PureHeader.types";
import BaseButton from "../../base/BaseButton.vue";
import BaseHeader from "../../base/BaseHeader.vue";

export default defineComponent({
  name: "PureHeader",
  components: {
    BaseButton,
    BaseHeader,
  },
  props: {
    username: {
      type: String,
    },
  },
  emits: [...pureHeaderEvents],
  computed: {
    authenticated(): boolean {
      return this.username ? true : false;
    },
  },
  methods: {
    onBrand() {
      this.$emit("brandClicked");
    },
    onLogin() {
      this.$emit("loginClicked");
    },
    onLogout() {
      this.$emit("logoutClicked");
    },
    onSignup() {
      this.$emit("signupClicked");
    },
    onUsername() {
      this.$emit("usernameClicked");
    },
  },
});
</script>

<style lang="css" scoped>
.pure-header .base-button + .base-button {
  margin-left: 10px;
}

.pure-header .pure-header--brand {
  cursor: pointer;

  font-weight: 900;
  font-size: 24px;
  margin: 6px 0 6px 10px;
}
</style>
