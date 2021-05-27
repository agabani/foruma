import PureDeleteYourAccountPanel from "./PureDeleteYourAccountPanel.vue";

export default {
  title: "Components/PureDeleteYourAccountPanel",
  component: PureDeleteYourAccountPanel,
  argTypes: {
    onClick: {},
  },
};

const Template = (args) => ({
  components: { PureDeleteYourAccountPanel },
  setup() {
    return { args };
  },
  template: '<PureDeleteYourAccountPanel v-bind="args" />',
});

export const Default = Template.bind({});
Default.args = {};
