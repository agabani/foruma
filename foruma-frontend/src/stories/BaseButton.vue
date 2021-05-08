<template>
  <button :class="classes" @click="onClick">{{ label }}</button>
</template>

<script lang="ts">
import { defineComponent } from "vue";

export default defineComponent({
  name: "BaseButton",
  props: {
    label: {
      type: String,
      required: true,
    },
    primary: {
      type: Boolean,
      default: false,
    },
    size: {
      default: "medium",
      type: String,
      validator: function (value: string): boolean {
        return ["small", "medium", "large"].indexOf(value) !== -1;
      },
    },
  },
  emits: ["click"],
  methods: {
    onClick() {
      this.$emit("click");
    },
  },
  computed: {
    classes(): string {
      return [
        "base-button",
        `base-button--${this.size}`,
        `base-button--${this.primary ? "primary" : "secondary"}`,
      ].join(" ");
    },
  },
});
</script>

<style lang="css" scoped>
.base-button {
  border: 0;
  border-radius: 3em;
  cursor: pointer;
  font-weight: 700;
}

.base-button--medium {
  font-size: 12px;
  padding: 10px 16px;
}

.base-button--medium {
  font-size: 14px;
  padding: 11px 20px;
}

.base-button--large {
  font-size: 16px;
  padding: 12px 24px;
}

.base-button--primary {
  background-color: #1ea7fd;
  color: white;
}

.base-button--secondary {
  background-color: white;
  color: #333;
  box-shadow: rgba(0, 0, 0, 0.15) 0 0 0 1px inset;
}
</style>
