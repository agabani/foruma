<template>
  <div>
    <PureSessionsPanel :sessions="sessions" />
  </div>
</template>

<script lang="ts">
import { defineComponent, PropType } from "vue";
import PureSessionsPanel, { Session } from "../PureSessionsPanel.vue";

export default defineComponent({
  name: "PureAccountSettingsPage",
  components: {
    PureSessionsPanel,
  },
  props: {
    sessions: {
      type: Array as PropType<Session[]>,
      required: true,
    },
  },
});
</script>
