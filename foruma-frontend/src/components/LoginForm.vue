<template>
  <BaseLoginForm @submit="authenticate" />
</template>
<script lang="ts">
import { useStore } from "@/store";
import { AuthenticatePayload } from "@/store/types";
import { defineComponent } from "vue";
import BaseLoginForm from "@/stories/BaseLoginForm.vue";

export default defineComponent({
  name: "LoginForm",
  components: {
    BaseLoginForm,
  },
  setup() {
    const store = useStore();

    return {
      authenticate: (payload: AuthenticatePayload) =>
        store.dispatch("authenticate", payload),
    };
  },
});
</script>
