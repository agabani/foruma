<template>
  <div class="change-password-panel">
    <BasePanel>
      <template v-slot:header>
        <span>Change your password</span>
      </template>
      <template v-slot>
        <form @submit="onSubmit">
          <div>
            <BasePasswordField
              placeholder="Old password"
              @change="updateOldPassword"
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
import BaseButton from "./BaseButton.vue";
import BasePanel from "./BasePanel.vue";
import BasePasswordField from "./BasePasswordField.vue";
import type { Change as BasePasswordFieldChange } from "./BasePasswordField.vue";

export default defineComponent({
  name: "ChangePasswordPanel",
  components: {
    BaseButton,
    BasePanel,
    BasePasswordField,
  },
  emits: ["submit"],
  data() {
    return {
      oldPassword: "",
      newPassword: "",
      confirmNewPassword: "",
    };
  },
  methods: {
    onSubmit(event: { preventDefault: () => void }) {
      event.preventDefault();
      if (
        this.oldPassword &&
        this.newPassword &&
        this.confirmNewPassword &&
        this.newPassword === this.confirmNewPassword
      ) {
        this.$emit("submit", {
          oldPassword: this.oldPassword,
          newPassword: this.newPassword,
        });
      }
    },
    updateOldPassword(value: BasePasswordFieldChange): void {
      this.oldPassword = value.newValue;
    },
    updateNewPassword(value: BasePasswordFieldChange): void {
      this.newPassword = value.newValue;
    },
    updateConfirmNewPassword(value: BasePasswordFieldChange): void {
      this.confirmNewPassword = value.newValue;
    },
  },
});
</script>

<style lang="css" scoped>
.change-password-panel {
}
</style>
