import { Story } from "@storybook/vue3";
import PureLoginScreen from "./PureLoginScreen.vue";
import { appUseStore } from "../../helpers/store";

appUseStore();

export default {
  title: "Screens/Pure/PureLoginScreen",
  component: PureLoginScreen,
  argTypes: {},
};

const Template: Story = (args) => ({
  components: { PureLoginScreen },
  setup() {
    return { args };
  },
  template: `<PureLoginScreen v-bind="args" />`,
});

export const Default = Template.bind({});
Default.args = {};
