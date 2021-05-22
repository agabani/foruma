<template>
  <DeleteYourAccountPanel @click="terminateOwnAccount" />
</template>

<script lang="ts">
import { defineComponent } from "vue";
import { useStore } from "@/store";
import DeleteYourAccountPanel from "@/stories/DeleteYourAccountPanel.vue";

export default defineComponent({
  name: "DeleteYourAccountPanelContainer",
  components: {
    DeleteYourAccountPanel,
  },
  setup() {
    const store = useStore();

    return {
      terminateOwnAccount: () => store.dispatch("terminateOwnAccount"),
    };
  },
});
</script>
