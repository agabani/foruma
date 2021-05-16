<template>
  <BaseSignupForm @submit="createAccount" />
</template>

<script lang="ts">
import { defineComponent } from "vue";
import BaseSignupForm from "@/stories/BaseSignupForm.vue";
import { useStore } from "@/store";
import type { SignupPayload } from "@/store/types";

export default defineComponent({
  name: "SignupForm",
  components: {
    BaseSignupForm,
  },
  setup() {
    const store = useStore();

    return {
      createAccount: (payload: SignupPayload) => {
        store.dispatch("signup", payload);
      },
    };
  },
});
</script>
