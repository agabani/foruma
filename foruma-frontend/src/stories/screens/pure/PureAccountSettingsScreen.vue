<template>
  <div>
    <ContainerHeader />
    <ContainerChangeYourPasswordPanel />
    <ContainerSessionsPanel />
    <ContainerDeleteYourAccountPanel />
  </div>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import ContainerChangeYourPasswordPanel from "../../components/container/ContainerChangeYourPasswordPanel.vue";
import ContainerDeleteYourAccountPanel from "../../components/container/ContainerDeleteYourAccountPanel.vue";
import ContainerHeader from "../../components/container/ContainerHeader.vue";
import ContainerSessionsPanel from "../../components/container/ContainerSessionsPanel.vue";

export default defineComponent({
  name: "PureAccountSettingsScreen",
  components: {
    ContainerChangeYourPasswordPanel,
    ContainerDeleteYourAccountPanel,
    ContainerHeader,
    ContainerSessionsPanel,
  },
});
</script>
