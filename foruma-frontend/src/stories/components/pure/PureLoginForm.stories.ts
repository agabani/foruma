import { Story } from "@storybook/vue3";
import metaEvents from "../../helpers/meta-events";
import { pureLoginFormEvents, PureLoginFormProps } from "./PureLoginForm.types";
import PureLoginForm from "./PureLoginForm.vue";

export default {
  title: "Components/Pure/PureLoginForm",
  component: PureLoginForm,
  argTypes: {
    ...metaEvents(pureLoginFormEvents),
  },
};

const Template: Story<PureLoginFormProps> = (args) => ({
  components: { PureLoginForm },
  setup() {
    return { args };
  },
  template: `
    <PureLoginForm v-bind="args" />
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
