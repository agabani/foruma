import { Story } from "@storybook/vue3";
import PureAccountSettingsScreen from "./PureAccountSettingsScreen.vue";
import { appUseStore } from "../../helpers/store";

appUseStore();

export default {
  title: "Screens/Pure/PureAccountSettingsScreen",
  component: PureAccountSettingsScreen,
  argTypes: {},
};

const Template: Story = (args) => ({
  components: { PureAccountSettingsScreen },
  setup() {
    return { args };
  },
  template: `<PureAccountSettingsScreen v-bind="args" />`,
});

export const Default = Template.bind({});
Default.args = {};
