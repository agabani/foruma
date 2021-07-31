import { Story } from "@storybook/vue3";
import PureForumScreen from "./PureForumScreen.vue";
import { appUseStore } from "../../helpers/store";

appUseStore();

export default {
  title: "Screens/Pure/PureForumScreen",
  component: PureForumScreen,
  argTypes: {},
};

const Template: Story = (args) => ({
  components: { PureForumScreen },
  setup() {
    return { args };
  },
  template: `<PureForumScreen v-bind="args" />`,
});

export const Default = Template.bind({});
Default.args = {};
