<template>
  <LoginForm @submit="onSubmit" />
</template>

<script lang="ts">
import { defineComponent } from "vue";
import LoginForm from "../stories/LoginForm.vue";
import type { Submit as LoginFormSubmit } from "../stories/LoginForm.vue";

export default defineComponent({
  name: "Login",
  components: {
    LoginForm,
  },
  methods: {
    onSubmit(value: LoginFormSubmit) {
      console.log(value);
    },
  },
});
</script>
