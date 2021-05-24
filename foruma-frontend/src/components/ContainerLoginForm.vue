<template>
  <PureLoginForm
    @submit="onSubmit"
    :alertEventDate="alertEventDate"
    :alertMessage="alertMessage"
    :alertTitle="alertTitle"
  />
</template>

<script lang="ts">
import { computed, defineComponent, Login } from "vue";
import { useStore } from "@/store";
import { LoginPayload } from "@/store/types";
import PureLoginForm, { Sumbit } from "@/stories/PureLoginForm.vue";

export default defineComponent({
  name: "ContainerLoginForm",
  components: {
    PureLoginForm,
  },
  setup() {
    const store = useStore();

    return {
      authenticate: (payload: LoginPayload) => {
        store.dispatch("login", payload);
      },
      state: computed(() => store.getters.login),
    };
  },
  data() {
    return {
      alertTitle: "",
      alertMessage: "",
      alertEventDate: undefined as Date | undefined,
    };
  },
  watch: {
    state(newState: Login) {
      if (newState.error) {
        this.alertEventDate = newState.eventDate;
        this.alertMessage = newState.error.message;
        this.alertTitle = newState.error.title;
      }
    },
  },
  methods: {
    onSubmit(payload: Sumbit) {
      if (!payload.username || !payload.password) {
        this.alertTitle = "Sorry! There was a problem with your request!";
        this.alertMessage =
          "All fields needs to be filled in order to login to an account!";
        this.alertEventDate = new Date();
        return;
      }

      this.authenticate({
        username: payload.username,
        password: payload.password,
      });
    },
  },
});
</script>
