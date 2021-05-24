<template>
  <div>
    <ChangePasswordPanelContainer />
    <DeleteYourAccountPanelContainer />
  </div>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import ChangePasswordPanelContainer from "@/components/ChangePasswordPanelContainer.vue";
import DeleteYourAccountPanelContainer from "@/components/DeleteYourAccountPanelContainer.vue";

export default defineComponent({
  name: "AccountSettingsPage",
  components: {
    ChangePasswordPanelContainer,
    DeleteYourAccountPanelContainer,
  },
});
</script>
