import PureLoginForm from "./PureLoginForm.vue";

export default {
  title: "Components/PureLoginForm",
  component: PureLoginForm,
  argTypes: {
    onSubmit: {},
  },
};

const Template = (args) => ({
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

export const MissingFieldsAlert = Template.bind({});
MissingFieldsAlert.args = {
  ...Default.args,
  alertEventDate: new Date(),
  alertTitle: "Sorry! There was a problem with your request!",
  alertMessage: "Make sure your passwords match then try again!",
};
