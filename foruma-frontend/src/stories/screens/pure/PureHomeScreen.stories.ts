import { Story } from "@storybook/vue3";
import PureHomeScreen from "./PureHomeScreen.vue";
import { appUseStore } from "../../helpers/store";

appUseStore();

export default {
  title: "Screens/Pure/PureHomeScreen",
  component: PureHomeScreen,
  argTypes: {},
};

const Template: Story = (args) => ({
  components: { PureHomeScreen },
  setup() {
    return { args };
  },
  template: `<PureHomeScreen v-bind="args" />`,
});

export const Default = Template.bind({});
Default.args = {};
