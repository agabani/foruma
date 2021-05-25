import BaseHeader from "./BaseHeader.vue";

export default {
  title: "Components/BaseHeader",
  component: BaseHeader,
  argTypes: {
    onLogin: {},
    onLogout: {},
    onSignup: {},
  },
};

const Template = (args) => ({
  components: { BaseHeader },
  setup() {
    return { args };
  },
  template: '<BaseHeader v-bind="args" />',
});

export const Default = Template.bind({});
Default.args = {
  username: "",
};

export const LoggedIn = Template.bind({});
LoggedIn.args = {
  ...Default.args,
  authenticated: true,
  username: "Username",
};

export const LoggedOut = Template.bind({});
LoggedOut.args = {
  ...Default.args,
  authenticated: false,
};
