<template>
  <BaseHeader
    :authenticated="authenticated"
    :username="username"
    @login="navigateToLogin"
    @logout="unauthenticate"
    @signup="navigateToSignup"
  />
</template>
/
<script lang="ts">
import { computed, defineComponent } from "vue";
import BaseHeader from "@/stories/BaseHeader.vue";
import { useStore } from "@/store";
import router from "@/router";
export default defineComponent({
  name: "Header",
  components: {
    BaseHeader,
  },
  methods: {
    navigateToLogin() {
      router.push("/login");
    },
    navigateToSignup() {
      router.push("/signup");
    },
  },
  setup() {
    const store = useStore();

    return {
      authenticated: computed(() => store.getters.authenticated),
      username: computed(() => store.getters.username || ""),
      unauthenticate: () => store.dispatch("unauthenticate"),
    };
  },
});
</script>
