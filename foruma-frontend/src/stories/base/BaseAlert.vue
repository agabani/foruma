<template>
  <div :class="classes">
    <div class="base-alert--container">
      <div class="base-alert--icon">
        <BaseIcon :iconName="iconName" height="24" width="24" />
      </div>
      <div class="base-alert--content">
        <div class="base-alert--title">{{ title }}</div>
        <div v-if="message" class="base-alert--message">{{ message }}</div>
      </div>
      <div class="base-alert--close" @click="onClose">
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
          <path
            d="M13.414,12,23.707,1.707A1,1,0,0,0,22.293.293L12,10.586,1.707.293A1,1,0,0,0,.293,1.707L10.586,12,.293,22.293a1,1,0,0,0,0,1.414h0a1,1,0,0,0,1.414,0L12,13.414,22.293,23.707a1,1,0,0,0,1.414,0h0a1,1,0,0,0,0-1.414Z"
          />
        </svg>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import BaseIcon from "./BaseIcon.vue";
import enumProp from "../helpers/enum-prop";
import { baseAlertEvents, baseAlertPropsType } from "./BaseAlert.types";

export default defineComponent({
  name: "BaseAlert",
  components: {
    BaseIcon,
  },
  props: {
    eventDate: {
      type: Date,
    },
    message: {
      type: String,
      default: "",
    },
    title: {
      type: String,
      required: true,
    },
    type: {
      ...enumProp("success", baseAlertPropsType),
      default: "success",
    },
  },
  emits: [...baseAlertEvents],
  data() {
    return {
      isClosed: !this.eventDate,
    };
  },
  computed: {
    classes(): string {
      const c = ["base-alert", `base-alert--${this.type}`];

      if (this.isClosed) {
        c.push("base-alert--closed");
      }

      return c.join(" ");
    },
    iconName(): string {
      switch (this.type) {
        case "information":
          return "info";
        case "success":
          return "checkmark";
        default:
          return this.type;
      }
    },
  },
  methods: {
    onClose() {
      this.isClosed = true;
      this.$emit("close");
    },
  },
  watch: {
    eventDate() {
      this.isClosed = false;
      this.$emit("open");
    },
  },
});
</script>

<style lang="css" scoped>
.base-alert {
  padding: 0.7em;
  margin: 2em 1em;

  border: 1px solid rgba(0, 0, 0, 0.1);
  border-radius: 4px;
  box-shadow: rgba(0, 0, 0, 0.1) 0 1px 3px 0;
}

.base-alert--closed {
  padding: 0;
  margin: 0;

  font-size: 0;
  opacity: 0;
  visibility: hidden;

  transition: visibility 0s ease 300ms, opacity 600ms ease,
    font-size 600ms ease 600ms, padding 600ms ease 600ms,
    margin 600ms ease 600ms;
}

.base-alert--container {
  display: flex;
  align-items: center;

  height: 0;
  padding: 2em 1.2em;

  border-left-width: 4px;
  border-left-style: solid;
  border-top-left-radius: 4px;
  border-bottom-left-radius: 4px;
}

.base-alert--closed .base-alert--container {
  padding: 0;
  margin: 0;

  transition: padding 600ms ease 600ms, margin 600ms ease 600ms;
}

.base-alert--icon {
  margin-right: 1em;
}

.base-alert--contents {
}

.base-alert--close {
  margin-left: auto;
  padding-left: 1em;

  cursor: pointer;
}

.base-alert--close svg {
  max-height: 0.8em;

  fill: rgba(171, 173, 177, 1);
}

.base-alert--title {
  font-size: 1.2em;
}

.base-alert--message {
  margin-top: 0.5em;

  color: rgba(134, 136, 142, 1);
}

.base-alert--information .base-alert--container {
  border-left-color: #24aae8;
}

.base-alert--success .base-alert--container {
  border-left-color: #32bea6;
}

.base-alert--warning .base-alert--container {
  border-left-color: #f9c353;
}
</style>
