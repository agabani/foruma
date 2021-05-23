<template>
  <ChangePasswordPanel @submit="changePassword" />
</template>

<script lang="ts">
import { defineComponent } from "vue";
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
    };
  },
});
</script>
