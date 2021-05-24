import SignupForm from "./SignupForm.vue";

export default {
  title: "Components/SignupForm",
  component: SignupForm,
  argTypes: {
    onSubmit: {},
  },
};

const Template = (args) => ({
  components: { SignupForm },
  setup() {
    return { args };
  },
  template: `
    <SignupForm v-bind="args" />
    `,
});

export const Default = Template.bind({});
Default.args = {};

export const Username = Template.bind({});
Username.args = { ...Default.args, username: "Username" };

export const MissingFieldsAlert = Template.bind({});
MissingFieldsAlert.args = { ...Default.args, username: "Username" };
