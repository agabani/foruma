import { Story } from "@storybook/vue3";
import metaEvents from "../../helpers/meta-events";
import { pureHeaderEvents, PureHeaderProps } from "./PureHeader.types";
import PureHeader from "./PureHeader.vue";

export default {
  title: "Components/Pure/PureHeader",
  component: PureHeader,
  argTypes: {
    ...metaEvents(pureHeaderEvents),
  },
};

const Template: Story<PureHeaderProps> = (args) => ({
  components: { PureHeader },
  setup() {
    return { args };
  },
  template: '<PureHeader v-bind="args" />',
});

export const Default = Template.bind({});
Default.args = {};

export const LoggedIn = Template.bind({});
LoggedIn.args = {
  ...Default.args,
  username: "Username",
};

export const LoggedOut = Template.bind({});
LoggedOut.args = {
  ...Default.args,
};
