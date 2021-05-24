<template>
  <input
    :class="classes"
    type="password"
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
  name: "BasePasswordField",
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
      const c = [
        "base-password-field",
        `base-password-field--${this.dataSize}`,
      ];

      if (this.dataFullWidth) {
        c.push("base-password-field--full-width");
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
.base-password-field {
  margin: 1em 1em 1em 0;

  border: 0;
  border-radius: 3em;
  cursor: text;
}

.base-password-field:focus {
  outline: none;
}

.base-password-field--small {
  padding: 10px 16px;

  font-size: 12px;
}

.base-password-field--medium {
  padding: 11px 20px;

  font-size: 14px;
}

.base-password-field--large {
  padding: 12px 24px;

  font-size: 16px;
}

.base-password-field--full-width {
  width: 100%;
  box-sizing: border-box;
}

.base-password-field {
  background-color: white;
  box-shadow: rgba(0, 0, 0, 0.15) 0 0 0 1px inset;
}
</style>
