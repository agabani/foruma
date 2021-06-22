<template>
  <BaseSvg
    :width="width"
    :height="height"
    :viewBox="viewBox"
    :iconName="iconName"
  >
    <CheckmarkIcon v-if="iconName === 'checkmark'" />
    <ChromeIcon v-else-if="iconName === 'chrome'" />
    <EdgeIcon v-else-if="iconName === 'edge'" />
    <FirefoxIcon v-else-if="iconName === 'firefox'" />
    <InfoIcon v-else-if="iconName === 'info'" />
    <SafariIcon v-else-if="iconName === 'safari'" />
    <TrashIcon v-else-if="iconName === 'trash'" />
    <WarningIcon v-else-if="iconName === 'warning'" />
    <PlaceholderIcon v-else />
  </BaseSvg>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import { baseIconPropsIconName } from "./BaseIcon.types";

import BaseSvg from "./BaseSvg.vue";
import enumProp from "../helpers/enum-prop";
import CheckmarkIcon from "../icons/CheckmarkIcon.vue";
import ChromeIcon from "../icons/ChromeIcon.vue";
import EdgeIcon from "../icons/EdgeIcon.vue";
import FirefoxIcon from "../icons/FirefoxIcon.vue";
import InfoIcon from "../icons/InfoIcon.vue";
import PlaceholderIcon from "../icons/PlaceholderIcon.vue";
import SafariIcon from "../icons/SafariIcon.vue";
import TrashIcon from "../icons/TrashIcon.vue";
import WarningIcon from "../icons/WarningIcon.vue";

export default defineComponent({
  name: "BaseIcon",
  components: {
    BaseSvg,
    CheckmarkIcon,
    ChromeIcon,
    EdgeIcon,
    FirefoxIcon,
    InfoIcon,
    PlaceholderIcon,
    SafariIcon,
    TrashIcon,
    WarningIcon,
  },
  props: {
    iconName: {
      ...enumProp("placeholder", baseIconPropsIconName),
      default: "placeholder",
    },
    height: {
      type: [Number, String],
      default: 48,
    },
    width: {
      type: [Number, String],
      default: 48,
    },
  },
  computed: {
    viewBox(): string {
      switch (this.iconName) {
        case "checkmark":
          return "0 0 512 512";
        case "firefox":
          return "0 0 953.37 984";
        default:
          return "0 0 48 48";
      }
    },
  },
});
</script>
