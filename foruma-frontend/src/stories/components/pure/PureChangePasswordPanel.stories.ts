import { Story } from "@storybook/vue3";
import metaEvents from "../../helpers/meta-events";
import {
  pureChangePasswordPanelEvents,
  PureChangePasswordPanelProps,
  pureChangePasswordPanelPropsTypes,
} from "./PureChangePasswordPanel.types";
import PureChangePasswordPanel from "./PureChangePasswordPanel.vue";

export default {
  title: "Components/Pure/PureChangePasswordPanel",
  component: PureChangePasswordPanel,
  argTypes: {
    alertType: {
      control: { type: "select", options: pureChangePasswordPanelPropsTypes },
    },
    ...metaEvents(pureChangePasswordPanelEvents),
  },
};

const Template: Story<PureChangePasswordPanelProps> = (args) => ({
  components: { PureChangePasswordPanel },
  setup() {
    return { args };
  },
  template: `<PureChangePasswordPanel v-bind="args" />`,
});

export const Default = Template.bind({});
Default.args = {};

export const SuccessAlert = Template.bind({});
SuccessAlert.args = {
  ...Default.args,
  alertEventDate: new Date(),
  alertTitle: "Your password has been changed",
  alertType: "success",
};

export const WarningAlert = Template.bind({});
WarningAlert.args = {
  ...Default.args,
  alertEventDate: new Date(),
  alertTitle: "Uh oh, something went wrong",
  alertMessage: "Sorry! There was a problem with your request!",
};
