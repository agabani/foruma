<template>
  <div>
    <ContainerChangePasswordPanelContainer />
    <DeleteYourAccountPanelContainer />
  </div>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import DeleteYourAccountPanelContainer from "@/components/DeleteYourAccountPanelContainer.vue";
import ContainerChangePasswordPanelContainer from "@/components/ContainerChangePasswordPanelContainer.vue";

export default defineComponent({
  name: "AccountSettingsPage",
  components: {
    ContainerChangePasswordPanelContainer,
    DeleteYourAccountPanelContainer,
  },
});
</script>
