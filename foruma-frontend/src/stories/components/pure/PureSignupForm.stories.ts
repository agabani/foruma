import { Story } from "@storybook/vue3";
import metaEvents from "../../helpers/meta-events";
import {
  pureSignupFormEvents,
  PureSignupFormProps,
} from "./PureSignupForm.types";
import PureSignupForm from "./PureSignupForm.vue";

export default {
  title: "Components/Pure/PureSignupForm",
  component: PureSignupForm,
  argTypes: {
    ...metaEvents(pureSignupFormEvents),
  },
};

const Template: Story<PureSignupFormProps> = (args) => ({
  components: { PureSignupForm },
  setup() {
    return { args };
  },
  template: `
    <PureSignupForm v-bind="args" />
    `,
});

export const Default = Template.bind({});
Default.args = {};

export const Username = Template.bind({});
Username.args = { ...Default.args, username: "Username" };

export const WarningAlert = Template.bind({});
WarningAlert.args = {
  ...Default.args,
  alertEventDate: new Date(),
  alertTitle: "Uh oh, something went wrong",
  alertMessage: "Sorry! There was a problem with your request!",
};
