<template>
  <div class="base-card">
    <slot></slot>
  </div>
</template>

<script lang="ts">
import { defineComponent } from "vue";
export default defineComponent({
  name: "BaseCard",
});
</script>

<style lang="css" scoped>
.base-card + .base-card {
  margin-top: 1em;
}
</style>
