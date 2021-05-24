<template>
  <PureSignupForm
    @submit="onSubmit"
    :alertEventDate="alertEventDate"
    :alertMessage="alertMessage"
    :alertTitle="alertTitle"
  />
</template>

<script lang="ts">
import { defineComponent } from "vue";
import { useStore } from "@/store";
import { SignupPayload } from "@/store/types";
import PureSignupForm, { Sumbit } from "@/stories/PureSignupForm.vue";

export default defineComponent({
  name: "ContainerSignupForm",
  components: {
    PureSignupForm,
  },
  setup() {
    const store = useStore();

    return {
      createAccount: (payload: SignupPayload) => {
        store.dispatch("signup", payload);
      },
    };
  },
  data() {
    return {
      alertTitle: "",
      alertMessage: "",
      alertEventDate: undefined as Date | undefined,
    };
  },
  methods: {
    onSubmit(payload: Sumbit) {
      if (!payload.username || !payload.password || !payload.confirmPassword) {
        this.alertTitle = "Sorry! There was a problem with your request!";
        this.alertMessage =
          "All fields needs to be filled in order to create an account!";
        this.alertEventDate = new Date();
        return;
      }

      if (payload.password !== payload.confirmPassword) {
        this.alertTitle = "Sorry! There was a problem with your request!";
        this.alertMessage = "Make sure your passwords match then try again!";
        this.alertEventDate = new Date();
        return;
      }

      this.createAccount({
        username: payload.username,
        password: payload.password,
      });
    },
  },
});
</script>
