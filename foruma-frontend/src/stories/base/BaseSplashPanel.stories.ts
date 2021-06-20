import { Story } from "@storybook/vue3";
import { BaseSplashPanelProps } from "./BaseSplashPanel.types";
import BaseSplashPanel from "./BaseSplashPanel.vue";

export default {
  title: "Base/BaseSplashPanel",
  components: BaseSplashPanel,
  argTypes: {
    splash: {
      control: "text",
      type: String,
    },
    default: {
      control: "text",
      type: String,
    },
  },
};

const Template: Story<BaseSplashPanelProps> = (args) => ({
  components: { BaseSplashPanel },
  setup() {
    return { args };
  },
  template: `
    <BaseSplashPanel v-bind="args">
        <template v-if="${!!args.splash}" v-slot:splash>
            ${args.splash}
        </template>
        <template v-if="${!!args.default}" v-slot>
            ${args.default}
        </template>
    </BaseSplashPanel>
    `,
});

export const Default = Template.bind({});
Default.args = { default: "Default" };

export const Empty = Template.bind({});
Empty.args = { ...Default.args, default: undefined };

export const Splash = Template.bind({});
Splash.args = { ...Default.args, splash: "Splash" };
