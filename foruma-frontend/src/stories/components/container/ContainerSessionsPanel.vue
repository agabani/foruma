<template>
  <PureSessionsPanel :sessions="sessions" @deleteClicked="onDelete" />
</template>

<script lang="ts">
import { useStore } from "../../../store/use";
import { computed, defineComponent } from "vue";
import PureSessionsPanel from "../pure/PureSessionsPanel.vue";
import { DeleteSessionPayload } from "@/store/types";
import { PureSessionsPanelEventDeleteClicked } from "../pure/PureSessionsPanel.types";

export default defineComponent({
  name: "ContainerSessionsPanel",
  components: {
    PureSessionsPanel,
  },
  setup() {
    const store = useStore();
    return {
      sessions: computed(() => store.getters.sessions),
      deleteSession: (payload: DeleteSessionPayload) =>
        store.dispatch("deleteSession", payload),
    };
  },
  methods: {
    onDelete(payload: PureSessionsPanelEventDeleteClicked) {
      this.deleteSession(payload);
    },
  },
});
</script>
