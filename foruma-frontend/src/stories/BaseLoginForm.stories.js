import LoginForm from "./BaseLoginForm.vue";

export default {
  title: "Components/BaseLoginForm",
  component: LoginForm,
  argTypes: {
    onSubmit: {},
  },
};

const Template = (args) => ({
  components: { LoginForm },
  setup() {
    return { args };
  },
  template: '<LoginForm v-bind="args" />',
});

export const Default = Template.bind({});
Default.args = {};

export const Username = Template.bind({});
Username.args = { ...Default.args, username: "Username" };
