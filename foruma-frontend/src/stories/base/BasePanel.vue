<template>
  <div class="base-panel">
    <div class="base-panel--header" v-if="!!$slots.header">
      <slot name="header"></slot>
    </div>
    <div class="base-panel--default" v-if="!!$slots.default">
      <slot></slot>
    </div>
    <div class="base-panel--footer" v-if="!!$slots.footer">
      <slot name="footer"></slot>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent } from "vue";
export default defineComponent({
  name: "BasePanel",
});
</script>

<style lang="css" scoped>
.base-panel {
  margin: 1.5em 0 2.5em;
  padding: 2em 1.5em;

  border: 1px solid rgba(0, 0, 0, 0.1);
  border-radius: 4px;
  box-shadow: rgba(0, 0, 0, 0.1) 0 1px 3px 0;
}

.base-panel--header {
  font-size: 24px;
  font-weight: 500;

  margin-bottom: 1em;
  padding-bottom: 0.5em;

  border-bottom: 1px solid rgba(0, 0, 0, 0.1);
}

.base-panel--default {
}

.base-panel--footer {
  margin-top: 1em;
  padding-top: 0.5em;

  border-top: 1px solid rgba(0, 0, 0, 0.1);
}
</style>
