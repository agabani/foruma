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
  alertEventDate: new Date(),
  alertTitle: "Your password has been changed",
  alertType: "success",
};

export const WarningAlertMessage = Template.bind({});
WarningAlertMessage.args = {
  ...Default.args,
  displayAlert: true,
  alertEventDate: new Date(),
  alertMessage: "Sorry! There was a problem with your request!",
  alertTitle: "Uh oh, something went wrong",
  alertType: "warning",
};
