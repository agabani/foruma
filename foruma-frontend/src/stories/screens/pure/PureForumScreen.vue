<template>
  <div>
    <ContainerHeader />
    <ContainerSubForumPanel />
  </div>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import ContainerHeader from "../../components/container/ContainerHeader.vue";
import ContainerSubForumPanel from "../../components/container/ContainerSubForumPanel.vue";

export default defineComponent({
  name: "PureForumScreen",
  components: {
    ContainerHeader,
    ContainerSubForumPanel,
  },
});
</script>
