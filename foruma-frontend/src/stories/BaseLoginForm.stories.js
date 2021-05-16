import BaseLoginForm from "./BaseLoginForm.vue";

export default {
  title: "Components/BaseLoginForm",
  component: BaseLoginForm,
  argTypes: {
    onSubmit: {},
  },
};

const Template = (args) => ({
  components: { BaseLoginForm },
  setup() {
    return { args };
  },
  template: '<BaseLoginForm v-bind="args" />',
});

export const Default = Template.bind({});
Default.args = {};

export const Username = Template.bind({});
Username.args = { ...Default.args, username: "Username" };
