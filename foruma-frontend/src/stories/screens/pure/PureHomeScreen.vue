<template>
  <div>
    <ContainerHeader />
    <h1>Welcome to Your Vue.js + TypeScript App</h1>
  </div>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import ContainerHeader from "../../components/container/ContainerHeader.vue";

export default defineComponent({
  name: "PureHomeScreen",
  components: {
    ContainerHeader,
  },
});
</script>
