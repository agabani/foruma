<template>
  <PureDeleteYourAccountPanel @click="terminateOwnAccount" />
</template>

<script lang="ts">
import { defineComponent } from "vue";
import { useStore } from "@/store";
import PureDeleteYourAccountPanel from "@/stories/PureDeleteYourAccountPanel.vue";

export default defineComponent({
  name: "ContainerDeleteYourAccountPanel",
  components: {
    PureDeleteYourAccountPanel,
  },
  setup() {
    const store = useStore();

    return {
      terminateOwnAccount: () => store.dispatch("terminateOwnAccount"),
    };
  },
});
</script>
