<template>
  <PureLoginForm
    @submitted="onSubmit"
    :alertEventDate="alertEventDate"
    :alertMessage="alertMessage"
    :alertTitle="alertTitle"
  />
</template>

<script lang="ts">
import { useStore } from "../../../store/use";
import { ChangedEvent, computed, defineComponent } from "vue";
import PureLoginForm from "../pure/PureLoginForm.vue";
import { PureLoginFormEventSubmit } from "../pure/PureLoginForm.types";
import { LoginPayload } from "@/store/types";

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
    onSubmit(payload: PureLoginFormEventSubmit) {
      if (!payload.username || !payload.password) {
        this.alertTitle = "Uh oh, something went wrong";
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
