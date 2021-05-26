<template>
  <div class="pure-change-password-panel">
    <BasePanel>
      <template v-slot:header>
        <span>Change your password</span>
      </template>
      <template v-slot>
        <BaseAlert
          :eventDate="alertEventDate"
          :type="alertType"
          :title="alertTitle"
          :message="alertMessage"
        ></BaseAlert>
        <form @submit="onSubmit">
          <div>
            <BasePasswordField
              placeholder="Current password"
              @change="updateCurrentPassword"
            />
          </div>
          <div>
            <BasePasswordField
              placeholder="New password"
              @change="updateNewPassword"
            />
          </div>
          <div>
            <BasePasswordField
              placeholder="Confirm new password"
              @change="updateConfirmNewPassword"
            />
          </div>
          <div>
            <BaseButton label="Change password" primary />
          </div>
        </form>
      </template>
    </BasePanel>
  </div>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import BaseAlert from "./BaseAlert.vue";
import BaseButton from "./BaseButton.vue";
import BasePanel from "./BasePanel.vue";
import BasePasswordField, { Change } from "./BasePasswordField.vue";

export interface Submit {
  currentPassword: string;
  newPassword: string;
  confirmNewPassword: string;
}

export default defineComponent({
  name: "PureChangePasswordPanel",
  components: {
    BaseAlert,
    BaseButton,
    BasePanel,
    BasePasswordField,
  },
  props: {
    alertEventDate: {
      type: Date,
    },
    alertMessage: {
      type: String,
      default: "",
    },
    alertTitle: {
      type: String,
      default: "",
    },
    alertType: {
      type: String,
      default: "warning",
    },
  },
  emits: ["submit"],
  data() {
    return {
      currentPassword: "",
      newPassword: "",
      confirmNewPassword: "",
    };
  },
  methods: {
    onSubmit(event: { preventDefault: () => void }) {
      event.preventDefault();

      const submit: Submit = {
        currentPassword: this.currentPassword,
        newPassword: this.newPassword,
        confirmNewPassword: this.confirmNewPassword,
      };

      this.$emit("submit", submit);
    },
    updateCurrentPassword(value: Change): void {
      this.currentPassword = value.newValue;
    },
    updateNewPassword(value: Change): void {
      this.newPassword = value.newValue;
    },
    updateConfirmNewPassword(value: Change): void {
      this.confirmNewPassword = value.newValue;
    },
  },
});
</script>

<style lang="css" scoped>
.pure-change-password-panel {
}
</style>
