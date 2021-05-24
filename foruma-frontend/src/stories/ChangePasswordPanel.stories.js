import ChangePasswordPanel from "./ChangePasswordPanel.vue";

export default {
  title: "Components/ChangePasswordPanel",
  component: ChangePasswordPanel,
  argTypes: {
    onSubmit: {},
  },
};

const Template = (args) => ({
  components: { ChangePasswordPanel },
  setup() {
    return { args };
  },
  template: '<ChangePasswordPanel v-bind="args" />',
});

export const Default = Template.bind({});
Default.args = {};

export const SuccessAlert = Template.bind({});
SuccessAlert.args = {
  ...Default.args,
  displayAlert: true,
  alertType: "success",
  alertTitle: "Your password has been changed",
};

export const WarningAlertMessage = Template.bind({});
WarningAlertMessage.args = {
  ...Default.args,
  displayAlert: true,
  alertType: "warning",
  alertTitle: "Uh oh, something went wrong",
  alertMessage: "Sorry! There was a problem with your request!",
};
