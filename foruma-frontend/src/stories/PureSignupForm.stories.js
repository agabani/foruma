import PureSignupForm from "./PureSignupForm.vue";

export default {
  title: "Components/PureSignupForm",
  component: PureSignupForm,
  argTypes: {
    onSubmit: {},
  },
};

const Template = (args) => ({
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
