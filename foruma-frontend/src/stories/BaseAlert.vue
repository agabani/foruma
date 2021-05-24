<template>
  <div :class="classes">
    <div class="base-alert--container">
      <div class="base-alert--icon">
        <svg
          v-if="type === 'information'"
          xmlns="http://www.w3.org/2000/svg"
          viewBox="0 0 24 24"
        >
          <path
            d="M12,24A12,12,0,1,0,0,12,12.013,12.013,0,0,0,12,24ZM12,5a1.5,1.5,0,1,1-1.5,1.5A1.5,1.5,0,0,1,12,5Zm-1,5h1a2,2,0,0,1,2,2v6a1,1,0,0,1-2,0V12H11a1,1,0,0,1,0-2Z"
          />
        </svg>
        <svg
          v-if="type === 'success'"
          xmlns="http://www.w3.org/2000/svg"
          viewBox="0 0 24 24"
        >
          <path
            d="M19,0H5A5.006,5.006,0,0,0,0,5V19a5.006,5.006,0,0,0,5,5H19a5.006,5.006,0,0,0,5-5V5A5.006,5.006,0,0,0,19,0Zm1,8.079-9.254,9.254a2,2,0,0,1-2.829,0L4,13.417A1,1,0,0,1,5.417,12l3.916,3.916,9.255-9.254A1,1,0,1,1,20,8.079Z"
          />
        </svg>
        <svg
          v-if="type === 'warning'"
          xmlns="http://www.w3.org/2000/svg"
          viewBox="0 0 24 24"
        >
          <path
            d="M12,24A12,12,0,1,0,0,12,12.013,12.013,0,0,0,12,24ZM11,6a1,1,0,0,1,2,0v8a1,1,0,0,1-2,0Zm1,12a1,1,0,1,1-1,1A1,1,0,0,1,12,18Z"
          />
        </svg>
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

export default defineComponent({
  name: "BaseAlert",
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
      type: String,
      default: "success",
      // validator: function (value: string): boolean {
      //   return ["information", "success", "warning"].indexOf(value) !== -1;
      // },
    },
  },
  emits: ["open", "close"],
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

  padding: 1em 1.2em;

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

.base-alert--icon svg {
  max-height: 1.8em;
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
  border-left-color: #007bc2;
}

.base-alert--information .base-alert--icon svg {
  fill: #007bc2;
}

.base-alert--success .base-alert--container {
  border-left-color: #21a67a;
}

.base-alert--success .base-alert--icon svg {
  fill: #21a67a;
}

.base-alert--warning .base-alert--container {
  border-left-color: #f0a92e;
}

.base-alert--warning .base-alert--icon svg {
  fill: #f0a92e;
}
</style>
