import BaseAlert from "./BaseAlert.vue";

export default {
  title: "Components/BaseAlert",
  component: BaseAlert,
  argTypes: {
    type: {
      control: {
        type: "select",
        options: ["information", "success", "warning"],
      },
    },
    onClose: {},
    onOpen: {},
  },
};

const Template = (args) => ({
  components: { BaseAlert },
  setup() {
    return { args };
  },
  template: '<BaseAlert v-bind="args" />',
});

export const Default = Template.bind({});
Default.args = {
  eventDate: new Date(),
  title: "",
  message: "",
  type: "information",
};

export const Information = Template.bind({});
Information.args = {
  ...Default.args,
  title: "Did you know?",
  type: "information",
};

export const InformationMessage = Template.bind({});
InformationMessage.args = {
  ...Information.args,
  message: "Here is something that you might like to know.",
};

export const Warning = Template.bind({});
Warning.args = {
  ...Default.args,
  title: "Uh oh, something went wrong",
  type: "warning",
};

export const WarningMessage = Template.bind({});
WarningMessage.args = {
  ...Warning.args,
  message: "Sorry! There was a problem with your request!",
};

export const Success = Template.bind({});
Success.args = {
  ...Default.args,
  title: "Yay! Everything worked!",
  type: "success",
};

export const SuccessMessage = Template.bind({});
SuccessMessage.args = {
  ...Success.args,
  message: "Congrats on the internet loading your request.",
};
