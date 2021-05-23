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
