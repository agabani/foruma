<template>
  <PureSignupForm
    @submitted="onSubmit"
    :alertEventDate="alertEventDate"
    :alertMessage="alertMessage"
    :alertTitle="alertTitle"
  />
</template>

<script lang="ts">
import { useStore } from "../../../store/use";
import { ChangedEvent, computed, defineComponent } from "vue";
import { PureSignupFormEventSubmit } from "../pure/PureSignupForm.types";
import PureSignupForm from "../pure/PureSignupForm.vue";
import { SignupPayload } from "@/store/types";

export default defineComponent({
  name: "ContainerSignupForm",
  components: {
    PureSignupForm,
  },
  setup() {
    const store = useStore();
    return {
      signup: (payload: SignupPayload) => {
        store.dispatch("signup", payload);
      },
      signupChangedEvent: computed(() => store.getters.signupChangedEvent),
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
    signupChangedEvent(event: ChangedEvent) {
      if (event.error) {
        this.alertEventDate = event.eventDate;
        this.alertMessage = event.error.message;
        this.alertTitle = event.error.title;
      }
    },
  },
  methods: {
    onSubmit(payload: PureSignupFormEventSubmit) {
      if (!payload.username || !payload.password || !payload.confirmPassword) {
        this.alertTitle = "Uh oh, something went wrong";
        this.alertMessage =
          "All fields needs to be filled in order to create an account!";
        this.alertEventDate = new Date();
        return;
      }

      if (payload.password !== payload.confirmPassword) {
        this.alertTitle = "Uh oh, something went wrong";
        this.alertMessage = "Make sure your passwords match then try again!";
        this.alertEventDate = new Date();
        return;
      }

      this.signup({
        username: payload.username,
        password: payload.password,
      });
    },
  },
});
</script>
