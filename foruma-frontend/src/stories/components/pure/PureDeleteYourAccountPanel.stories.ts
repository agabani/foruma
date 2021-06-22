import { Story } from "@storybook/vue3";
import metaEvents from "../../helpers/meta-events";
import { pureDeleteYourAccountPanelEvents } from "./PureDeleteYourAccountPanel.types";
import PureDeleteYourAccountPanel from "./PureDeleteYourAccountPanel.vue";

export default {
  title: "Components/Pure/PureDeleteYourAccountPanel",
  component: PureDeleteYourAccountPanel,
  argTypes: {
    ...metaEvents(pureDeleteYourAccountPanelEvents),
  },
};

const Template: Story = (args) => ({
  components: { PureDeleteYourAccountPanel },
  setup() {
    return { args };
  },
  template: '<PureDeleteYourAccountPanel v-bind="args" />',
});

export const Default = Template.bind({});
Default.args = {};
