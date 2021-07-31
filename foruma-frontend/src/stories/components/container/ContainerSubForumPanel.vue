<template>
  <PureSubForumPanel :subForums="subForums" />
</template>

<script lang="ts">
import { computed, defineComponent } from "vue";
import { useStore } from "../../../store/use";
import PureSubForumPanel from "../pure/PureSubForumPanel.vue";

export default defineComponent({
  name: "ContainerSubForumPanel",
  components: {
    PureSubForumPanel,
  },
  setup() {
    const store = useStore();
    return {
      subForums: computed(() => store.getters.subForums),
      getSubForums: () => {
        store.dispatch("getSubForums");
      },
    };
  },
  created: function () {
    this.getSubForums();
  },
});
</script>
