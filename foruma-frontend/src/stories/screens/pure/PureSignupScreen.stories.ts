import { Story } from "@storybook/vue3";
import PureSignupScreen from "./PureSignupScreen.vue";
import { appUseStore } from "../../helpers/store";

appUseStore();

export default {
  title: "Screens/Pure/PureSignupScreen",
  component: PureSignupScreen,
  argTypes: {},
};

const Template: Story = (args) => ({
  components: { PureSignupScreen },
  setup() {
    return { args };
  },
  template: `<PureSignupScreen v-bind="args" />`,
});

export const Default = Template.bind({});
Default.args = {};
