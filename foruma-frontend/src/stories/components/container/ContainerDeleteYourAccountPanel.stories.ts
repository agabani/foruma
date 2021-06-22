import { Story } from "@storybook/vue3";
import ContainerDeleteYourAccountPanel from "./ContainerDeleteYourAccountPanel.vue";
import { appUseStore } from "../../helpers/store";

appUseStore();

export default {
  title: "Components/Container/ContainerDeleteYourAccountPanel",
  component: ContainerDeleteYourAccountPanel,
  argTypes: {},
};

const Template: Story = (args) => ({
  components: { ContainerDeleteYourAccountPanel },
  setup() {
    return { args };
  },
  template: '<ContainerDeleteYourAccountPanel v-bind="args" />',
});

export const Default = Template.bind({});
Default.args = {};
