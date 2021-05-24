<template>
  <input
    :class="classes"
    type="text"
    :placeholder="dataPlaceholder"
    v-model="dataValue"
  />
</template>

<script lang="ts">
import { defineComponent } from "vue";

export interface Change {
  newValue: string;
  oldValue: string;
}

export default defineComponent({
  name: "BaseTextField",
  props: {
    fullWidth: {
      type: Boolean,
      default: false,
    },
    placeholder: {
      type: String,
      default: "",
    },
    size: {
      type: String,
      default: "medium",
      validator: function (value: string): boolean {
        return ["small", "medium", "large"].indexOf(value) !== -1;
      },
    },
    value: {
      type: String,
      default: "",
    },
  },
  emits: ["change"],
  data() {
    return {
      dataFullWidth: this.fullWidth,
      dataPlaceholder: this.placeholder,
      dataSize: this.size,
      dataValue: this.value,
    };
  },
  computed: {
    classes(): string {
      const c = ["base-text-field", `base-text-field--${this.dataSize}`];

      if (this.dataFullWidth) {
        c.push("base-text-field--full-width");
      }

      return c.join(" ");
    },
  },
  watch: {
    dataValue(newValue, oldValue) {
      const change: Change = {
        newValue,
        oldValue,
      };
      this.$emit("change", change);
    },
  },
});
</script>

<style lang="css" scoped>
.base-text-field {
  margin: 1em 1em 1em 0;

  border: 0;
  border-radius: 3em;
  cursor: text;
}

.base-text-field:focus {
  outline: none;
}

.base-text-field--small {
  padding: 10px 16px;

  font-size: 12px;
}

.base-text-field--medium {
  padding: 11px 20px;

  font-size: 14px;
}

.base-text-field--large {
  padding: 12px 24px;

  font-size: 16px;
}

.base-text-field--full-width {
  width: 100%;
  box-sizing: border-box;
}

.base-text-field {
  background-color: white;
  box-shadow: rgba(0, 0, 0, 0.15) 0 0 0 1px inset;
}
</style>
