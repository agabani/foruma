import BaseSignupForm from "./BaseSignupForm.vue";

export default {
  title: "Components/BaseSignupForm",
  components: BaseSignupForm,
  argTypes: {
    username: {
      control: {
        type: "text",
      },
      table: {
        category: "props",
      },
    },
    submit: {
      table: {
        category: "events",
      },
    },
    onSubmit: {},
  },
};

const Template = (args) => ({
  components: { BaseSignupForm },
  setup() {
    return { args };
  },
  template: '<BaseSignupForm v-bind="args" />',
});

export const Default = Template.bind({});
Default.args = {};

export const Username = Template.bind({});
Username.args = { ...Default.args, username: "Username" };
