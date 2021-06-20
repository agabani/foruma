<template>
  <PureChangePasswordPanel
    @submitted="onSubmit"
    :alertEventDate="alertEventDate"
    :alertMessage="alertMessage"
    :alertTitle="alertTitle"
    :alertType="alertType"
  />
</template>

<script lang="ts">
import { ChangePasswordPayload } from "@/store/types";
import { ChangedEvent, computed, defineComponent } from "vue";
import { useStore } from "../../../store/use";
import { PureChangePasswordPanelEventSubmit } from "../pure/PureChangePasswordPanel.types";
import PureChangePasswordPanel from "../pure/PureChangePasswordPanel.vue";

export default defineComponent({
  name: "ContainerChangeYourPassword",
  components: {
    PureChangePasswordPanel,
  },
  setup() {
    const store = useStore();

    return {
      changePassword: (payload: ChangePasswordPayload) =>
        store.dispatch("changeOwnPassword", payload),
      passwordChangedEvent: computed(() => store.getters.passwordChangedEvent),
    };
  },
  data() {
    return {
      alertType: "warning",
      alertTitle: "",
      alertMessage: "",
      alertEventDate: undefined as Date | undefined,
    };
  },
  watch: {
    passwordChangedEvent(event: ChangedEvent) {
      if (event.error) {
        (this.alertType = "warning"), (this.alertMessage = event.error.message);
        this.alertTitle = event.error.title;
      } else {
        (this.alertType = "success"), (this.alertMessage = "");
        this.alertTitle = "Your password has been changed";
      }

      this.alertEventDate = event.eventDate;
    },
  },
  methods: {
    onSubmit(payload: PureChangePasswordPanelEventSubmit) {
      if (
        !payload.currentPassword ||
        !payload.newPassword ||
        !payload.confirmNewPassword
      ) {
        this.alertType = "warning";
        this.alertTitle = "Uh oh, something went wrong";
        this.alertMessage =
          "All fields needs to be filled in order to change your password!";
        this.alertEventDate = new Date();
        return;
      }

      if (payload.newPassword !== payload.confirmNewPassword) {
        this.alertType = "warning";
        this.alertTitle = "Uh oh, something went wrong";
        this.alertMessage = "Make sure your passwords match then try again!";
        this.alertEventDate = new Date();
        return;
      }

      this.changePassword({
        currentPassword: payload.currentPassword,
        newPassword: payload.newPassword,
      });
    },
  },
});
</script>
