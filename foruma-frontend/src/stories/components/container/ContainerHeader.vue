<template>
  <PureHeader
    :username="username"
    @brandClicked="onBrand"
    @loginClicked="onLogin"
    @logoutClicked="onLogout"
    @signupClicked="onSignup"
    @usernameClicked="onUsername"
  />
</template>

<script lang="ts">
import { useStore } from "../../../store/use";
import { computed, defineComponent } from "vue";
import PureHeader from "../pure/PureHeader.vue";

export default defineComponent({
  name: "ContainerHeader",
  components: {
    PureHeader,
  },
  setup() {
    const store = useStore();

    return {
      username: computed(() => store.getters.username),
      logout: () => store.dispatch("logout"),
      navigateToAccountSettings: () =>
        store.dispatch("navigateToAccountSettings"),
      navigateToHome: () => store.dispatch("navigateToHome"),
      navigateToLogin: () => store.dispatch("navigateToLogin"),
      navigateToSignup: () => store.dispatch("navigateToSignup"),
    };
  },
  methods: {
    onBrand() {
      this.navigateToHome();
    },
    onLogin() {
      this.navigateToLogin();
    },
    onLogout() {
      this.logout();
    },
    onSignup() {
      this.navigateToSignup();
    },
    onUsername() {
      this.navigateToAccountSettings();
    },
  },
});
</script>
