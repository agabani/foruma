<template>
  <BasePanel>
    <template v-slot:header>
      <span>Sub-Forums</span>
    </template>
    <template v-slot>
      <PureSubForumCard
        v-for="subForum in subForums"
        :key="subForum.id"
        :name="subForum.name"
        @click="onClick(subForum.id)"
      />
    </template>
  </BasePanel>
</template>

<script lang="ts">
import { defineComponent, PropType } from "vue";
import BasePanel from "../../base/BasePanel.vue";
import {
  PureSubForumPanelEventClicked,
  pureSubForumPanelEvents,
  PureSubForumPanelPropsSubForum,
} from "./PureSubForumPanel.types";
import PureSubForumCard from "./PureSubForumCard.vue";

export default defineComponent({
  name: "PureSubForumPanel",
  components: {
    BasePanel,
    PureSubForumCard,
  },
  props: {
    subForums: {
      type: Array as PropType<PureSubForumPanelPropsSubForum[]>,
      required: true,
    },
  },
  emits: [...pureSubForumPanelEvents],
  methods: {
    onClick(id: string) {
      const event: PureSubForumPanelEventClicked = {
        id,
      };
      this.$emit("clicked", event);
    },
  },
});
</script>

<style lang="css" scoped>
.pure-sub-forum-panel {
  cursor: pointer;
}
</style>
