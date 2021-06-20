<template>
  <BasePanel>
    <template v-slot:header>
      <span>Sessions</span>
    </template>
    <template v-slot>
      <PureSessionCard
        v-for="session in sessions"
        :key="session.id"
        :browser="session.browser"
        :operatingSystem="session.operatingSystem"
        :lastActiveDate="session.lastActiveDate"
        @deleteClicked="onDelete(session.id)"
      />
    </template>
  </BasePanel>
</template>

<script lang="ts">
import { defineComponent, PropType } from "vue";
import BasePanel from "../../base/BasePanel.vue";
import PureSessionCard from "./PureSessionCard.vue";
import {
  PureSessionsPanelEventDeleteClicked,
  pureSessionsPanelEvents,
  PureSessionsPanelProps,
} from "./PureSessionsPanel.types";

export default defineComponent({
  name: "PureSessionsPanel",
  components: {
    BasePanel,
    PureSessionCard,
  },
  props: {
    sessions: {
      type: Array as PropType<PureSessionsPanelProps[]>,
      required: true,
    },
  },
  emits: [...pureSessionsPanelEvents],
  methods: {
    onDelete(id: string) {
      const event: PureSessionsPanelEventDeleteClicked = {
        id: id,
      };
      this.$emit("deleteClicked", event);
    },
  },
});
</script>
