import DeleteYourAccountPanel from "./DeleteYourAccountPanel.vue";

export default {
  title: "Components/DeleteYourAccountPanel",
  component: DeleteYourAccountPanel,
  argTypes: {
    onClick: {},
  },
};

const Template = (args) => ({
  components: { DeleteYourAccountPanel },
  setup() {
    return { args };
  },
  template: '<DeleteYourAccountPanel v-bind="args" />',
});

export const Default = Template.bind({});
Default.args = {};
