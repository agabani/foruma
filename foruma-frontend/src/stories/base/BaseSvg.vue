<template>
  <svg
    xmlns="http://www.w3.org/2000/svg"
    :width="width"
    :height="height"
    :viewBox="viewBox"
    :aria-labelledby="iconName"
    role="presentation"
  >
    <title :id="iconName" lang="en">{{ iconName }}</title>
    <g>
      <slot></slot>
    </g>
  </svg>
</template>

<script lang="ts">
import { defineComponent } from "vue";

export default defineComponent({
  name: "BaseSvg",
  props: {
    iconName: {
      type: String,
      default: "",
    },
    height: {
      type: [Number, String],
      default: 48,
    },
    width: {
      type: [Number, String],
      default: 48,
    },
    viewBox: {
      type: String,
      required: true,
    },
  },
});
</script>
