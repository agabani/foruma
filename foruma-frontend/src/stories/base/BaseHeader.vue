<template>
  <header class="base-header">
    <div class="base-header--item">
      <slot name="brand"></slot>
    </div>
    <div class="base-header--item">
      <slot></slot>
    </div>
  </header>
</template>

<script lang="ts">
import { defineComponent } from "vue";
export default defineComponent({
  name: "BaseHeader",
});
</script>

<style lang="css" scoped>
.base-header {
  display: flex;
  justify-content: space-between;

  padding: 1em 1.5em;
  margin-bottom: 2em;

  border-bottom: 1px solid rgba(0, 0, 0, 0.1);
}

.base-header--item {
  align-self: center;
}
</style>
