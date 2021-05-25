<template>
  <PureLoginForm
    @submit="onSubmit"
    :alertEventDate="alertEventDate"
    :alertMessage="alertMessage"
    :alertTitle="alertTitle"
  />
</template>

<script lang="ts">
import { computed, defineComponent, ChangedEvent } from "vue";
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
      login: (payload: LoginPayload) => {
        store.dispatch("login", payload);
      },
      loginChangedEvent: computed(() => store.getters.loginChangedEvent),
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
    loginChangedEvent(event: ChangedEvent) {
      if (event.error) {
        this.alertEventDate = event.eventDate;
        this.alertMessage = event.error.message;
        this.alertTitle = event.error.title;
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

      this.login({
        username: payload.username,
        password: payload.password,
      });
    },
  },
});
</script>
