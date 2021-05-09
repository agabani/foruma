<template>
  <input
    :class="classes"
    type="password"
    :placeholder="placeholder"
    v-model="value"
  />
</template>

<script lang="ts">
import { defineComponent } from "vue";

export default defineComponent({
  name: "BasePasswordField",
  props: {
    initialValue: {
      type: String,
      default: "",
    },
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
  },
  emits: ["change"],
  data() {
    return {
      value: this.initialValue,
    };
  },
  computed: {
    classes(): string {
      const c = ["base-password-field", `base-password-field--${this.size}`];

      if (this.fullWidth) {
        c.push("base-password-field--full-width");
      }

      return c.join(" ");
    },
  },
  watch: {
    value(newValue, oldValue) {
      this.$emit("change", { newValue, oldValue });
    },
  },
});
</script>

<style lang="css" scoped>
.base-password-field {
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
