<template>
  <button :class="classes" :type="type" @click="onClick">{{ label }}</button>
</template>

<script lang="ts">
import { defineComponent } from "vue";

export default defineComponent({
  name: "BaseButton",
  props: {
    fullWidth: {
      type: Boolean,
      default: false,
    },
    label: {
      type: String,
      required: true,
    },
    primary: {
      type: Boolean,
      default: false,
    },
    size: {
      type: String,
      default: "medium",
      validator: function (value: string): boolean {
        return ["small", "medium", "large"].indexOf(value) !== -1;
      },
    },
    type: {
      type: String,
      default: "submit",
      validator: function (value: string): boolean {
        return ["button", "reset", "submit"].indexOf(value) !== -1;
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
      const c = [
        "base-button",
        `base-button--${this.size}`,
        `base-button--${this.primary ? "primary" : "secondary"}`,
      ];

      if (this.fullWidth) {
        c.push("base-button--full-width");
      }

      return c.join(" ");
    },
  },
});
</script>

<style lang="css" scoped>
.base-button {
  margin: 1em 1em 1em 0;

  border: 0;
  border-radius: 3em;
  cursor: pointer;
  font-weight: 700;
}

.base-button--small {
  padding: 10px 16px;

  font-size: 12px;
}

.base-button--medium {
  padding: 11px 20px;

  font-size: 14px;
}

.base-button--large {
  padding: 12px 24px;

  font-size: 16px;
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

.base-button--full-width {
  width: 100%;
}
</style>
