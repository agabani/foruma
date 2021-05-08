import LoginForm from "./LoginForm.vue";

export default {
  title: "Components/LoginForm",
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

export const InitialUsername = Template.bind({});
InitialUsername.args = { ...Default.args, initialUsername: "Initial Username" };
