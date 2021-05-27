<template>
  <div>
    <ContainerChangePasswordPanel />
    <ContainerDeleteYourAccountPanel />
  </div>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import ContainerChangePasswordPanel from "@/components/ContainerChangePasswordPanel.vue";
import ContainerDeleteYourAccountPanel from "@/components/ContainerDeleteYourAccountPanel.vue";

export default defineComponent({
  name: "AccountSettingsPage",
  components: {
    ContainerChangePasswordPanel,
    ContainerDeleteYourAccountPanel,
  },
});
</script>
