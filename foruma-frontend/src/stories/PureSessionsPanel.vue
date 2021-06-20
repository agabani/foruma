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
      />
    </template>
  </BasePanel>
</template>

<script lang="ts">
import { defineComponent, PropType } from "vue";
import BasePanel from "./BasePanel.vue";
import PureSessionCard from "./PureSessionCard.vue";

export interface Session {
  id: string;
  browser: string;
  operatingSystem: string;
  lastActiveDate: Date;
}

export default defineComponent({
  name: "PureSessionsPanel",
  components: {
    BasePanel,
    PureSessionCard,
  },
  props: {
    sessions: {
      type: Array as PropType<Session[]>,
      required: true,
    },
  },
});
</script>
