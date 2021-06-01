<template>
  <BaseCard>
    <div class="pure-session-card">
      <div class="pure-session-card--icon">
        <PureIcon :iconName="browser.toLowerCase()" />
      </div>
      <div class="pure-session-card--contents">
        <div class="pure-session-card--title">
          {{ browser }} ({{ operatingSystem }})
        </div>
        <div class="pure-session-card--details">
          Last active {{ lastActiveDateText }}
        </div>
      </div>
    </div>
  </BaseCard>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import BaseCard from "./BaseCard.vue";
import PureIcon from "./PureIcon.vue";

export default defineComponent({
  name: "PureSessionCard",
  components: {
    BaseCard,
    PureIcon,
  },
  props: {
    browser: {
      type: String,
    },
    operatingSystem: {
      type: String,
    },
    lastActiveDate: {
      type: Date,
    },
  },
  computed: {
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
</style>
