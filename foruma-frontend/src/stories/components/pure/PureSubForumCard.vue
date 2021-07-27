<template>
  <BaseCard>
    <div class="pure-sub-forum-card" @click="onClick">{{ name }}</div>
  </BaseCard>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import BaseCard from "../../base/BaseCard.vue";
import { pureSubForumCardEvents } from "./PureSubForumCard.types";

export default defineComponent({
  name: "PureSubForumCard",
  components: {
    BaseCard,
  },
  props: {
    name: {
      type: String,
      required: true,
    },
  },
  emits: [...pureSubForumCardEvents],
  methods: {
    onClick() {
      this.$emit("clicked");
    },
  },
});
</script>

<style lang="css" scoped>
.pure-sub-forum-card {
  cursor: pointer;
}
</style>
