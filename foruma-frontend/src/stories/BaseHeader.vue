<template>
  <header class="base-header">
    <div class="item">
      <span class="brand">Foruma</span>
    </div>
    <div class="item">
      <base-button
        v-if="authenticated"
        size="small"
        :label="username"
        primary
        @click="$emit('accountSettings')"
      />
      <base-button
        v-if="authenticated"
        size="small"
        label="Log out"
        @click="$emit('logout')"
      />
      <base-button
        v-if="!authenticated"
        size="small"
        label="Log in"
        @click="$emit('login')"
      />
      <base-button
        v-if="!authenticated"
        size="small"
        label="Sign up"
        primary
        @click="$emit('signup')"
      />
    </div>
  </header>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import BaseButton from "./BaseButton.vue";

export default defineComponent({
  name: "BaseHeader",
  components: {
    BaseButton,
  },
  props: {
    authenticated: {
      type: Boolean,
      required: true,
    },
    username: {
      type: String,
      required: true,
    },
  },
  emits: ["accountSettings", "login", "logout", "signup"],
});
</script>

<style lang="css" scoped>
.base-header {
  display: flex;
  justify-content: space-between;

  padding: 16px 24px;

  border-bottom: 1px solid rgba(0, 0, 0, 0.1);
}

.base-header .base-button + .base-button {
  margin-left: 10px;
}

.base-header .brand {
  font-weight: 900;
  font-size: 24px;
  margin: 6px 0 6px 10px;
}
</style>
