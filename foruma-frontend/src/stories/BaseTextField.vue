<template>
  <input
    :class="classes"
    type="text"
    :placeholder="placeholder"
    v-model="value"
  />
</template>

<script lang="ts">
import { defineComponent } from "vue";
export default defineComponent({
  name: "BaseTextField",
  props: {
    initialValue: {
      type: String,
      default: "",
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
      return ["base-text-field", `base-text-field--${this.size}`].join(" ");
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
.base-text-field {
  border: 0;
  border-radius: 3em;
  cursor: text;
}

.base-text-field:focus {
  outline: none;
}

.base-text-field--small {
  font-size: 12px;
  padding: 10px 16px;
}

.base-text-field--medium {
  font-size: 14px;
  padding: 11px 20px;
}

.base-text-field--large {
  font-size: 16px;
  padding: 12px 24px;
}

.base-text-field {
  background-color: white;
  box-shadow: rgba(0, 0, 0, 0.15) 0 0 0 1px inset;
}
</style>
