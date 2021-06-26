<template>
  <BaseCard>
    <div class="pure-session-card">
      <div class="pure-session-card--icon">
        <BaseIcon :iconName="iconName" />
      </div>
      <div class="pure-session-card--contents">
        <div class="pure-session-card--title">
          {{ browserText }} {{ operatingSystemText }} {{ isCurrentSessionText }}
        </div>
        <div class="pure-session-card--details">
          Last active {{ lastActiveDateText }} {{ locationText }}
        </div>
      </div>
      <div class="pure-session-card--trash">
        <BaseIcon iconName="trash" @click="onDelete" />
      </div>
    </div>
  </BaseCard>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import BaseCard from "../../base/BaseCard.vue";
import BaseIcon from "../../base/BaseIcon.vue";
import { pureSessionCardEvents } from "./PureSessionCard.types";

export default defineComponent({
  name: "PureSessionCard",
  components: {
    BaseCard,
    BaseIcon,
  },
  props: {
    isCurrentSession: {
      type: Boolean,
    },
    browser: {
      type: String,
    },
    operatingSystem: {
      type: String,
    },
    location: {
      type: String,
    },
    lastActiveDate: {
      type: Date,
    },
  },
  emits: [...pureSessionCardEvents],
  computed: {
    isCurrentSessionText(): string {
      return this.isCurrentSession ? "(current session)" : "";
    },
    lastActiveDateText() {
      const formatter = new Intl.DateTimeFormat(undefined, {
        month: "long",
        day: "2-digit",
        hour: "2-digit",
        minute: "2-digit",
      });
      const text = formatter.format(this.lastActiveDate);
      return text;
    },
    browserText(): string {
      return this.browser ? this.browser : "Unknown";
    },
    iconName(): string {
      return this.browser ? this.browser.toLowerCase() : "placeholder";
    },
    locationText(): string {
      return this.location ? `(${this.location})` : "";
    },
    operatingSystemText(): string {
      return this.operatingSystem ? `(${this.operatingSystem})` : "";
    },
  },
  methods: {
    onDelete() {
      this.$emit("deleteClicked");
    },
  },
});
</script>

<style lang="css" scoped>
.pure-session-card {
  display: flex;

  align-items: center;
}

.pure-session-card--icon {
  margin-right: 1em;
}

.pure-session-card--details {
  margin-top: 0.25em;

  color: rgba(134, 136, 142, 1);
  font-size: 0.9em;
}

.pure-session-card--trash {
  cursor: pointer;

  margin-left: auto;
}
</style>
