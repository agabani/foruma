<template>
  <div>
    <ContainerSubForumPanel />
  </div>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import ContainerSubForumPanel from "../../components/container/ContainerSubForumPanel.vue";

export default defineComponent({
  name: "PureForumScreen",
  components: {
    ContainerSubForumPanel,
  },
});
</script>
