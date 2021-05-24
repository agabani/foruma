<template>
  <ChangePasswordPanel
    @submit="changePassword"
    :displayAlert="displayAlert"
    :alertTitle="alertTitle"
    :alertMessage="alertMessage"
    :alertType="alertType"
  />
</template>

<script lang="ts">
import { computed, defineComponent } from "vue";
import { useStore } from "@/store";
import ChangePasswordPanel from "@/stories/ChangePasswordPanel.vue";
import { ChangePasswordPayload } from "@/store/types";

export default defineComponent({
  name: "ChangePasswordPanelContainer",
  components: {
    ChangePasswordPanel,
  },
  setup() {
    const store = useStore();

    return {
      changePassword: (payload: ChangePasswordPayload) =>
        store.dispatch("changeOwnPassword", payload),
      displayAlert: computed(() => store.getters.passwordChanged?.when),
      alertTitle: computed(() =>
        store.getters.passwordChanged?.success
          ? "Your password has been changed"
          : "Uh oh, something went wrong"
      ),
      alertType: computed(() =>
        store.getters.passwordChanged?.success ? "success" : "warning"
      ),
      alertMessage: computed(() => store.getters.passwordChanged?.message),
    };
  },
});
</script>
