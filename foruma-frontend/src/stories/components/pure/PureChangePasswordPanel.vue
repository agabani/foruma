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
import BaseAlert from "../../base/BaseAlert.vue";
import BaseButton from "../../base/BaseButton.vue";
import BasePanel from "../../base/BasePanel.vue";
import BasePasswordField from "../../base/BasePasswordField.vue";
import { BasePasswordFieldEventChange } from "../../base/BasePasswordField.types";
import {
  pureChangePasswordPanelEvents,
  PureChangePasswordPanelEventSubmit,
  pureChangePasswordPanelPropsTypes,
} from "./PureChangePasswordPanel.types";
import enumProp from "../../helpers/enum-prop";

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
      ...enumProp("warning", pureChangePasswordPanelPropsTypes),
      default: "warning",
    },
  },
  emits: [...pureChangePasswordPanelEvents],
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

      const submit: PureChangePasswordPanelEventSubmit = {
        currentPassword: this.currentPassword,
        newPassword: this.newPassword,
        confirmNewPassword: this.confirmNewPassword,
      };

      this.$emit("submitted", submit);
    },
    updateCurrentPassword(value: BasePasswordFieldEventChange): void {
      this.currentPassword = value.newValue;
    },
    updateNewPassword(value: BasePasswordFieldEventChange): void {
      this.newPassword = value.newValue;
    },
    updateConfirmNewPassword(value: BasePasswordFieldEventChange): void {
      this.confirmNewPassword = value.newValue;
    },
  },
});
</script>

<style lang="css" scoped>
.pure-change-password-panel {
}
</style>
