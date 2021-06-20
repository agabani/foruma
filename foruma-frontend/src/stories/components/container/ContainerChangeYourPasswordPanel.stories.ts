import { Story } from "@storybook/vue3";
import ContainerChangeYourPasswordPanel from "./ContainerChangeYourPasswordPanel.vue";
import { appUseStore } from "../../helpers/store";

appUseStore();

export default {
  title: "Components/Container/ContainerChangeYourPasswordPanel",
  component: ContainerChangeYourPasswordPanel,
  argTypes: {},
};

const Template: Story = (args) => ({
  components: { ContainerChangeYourPasswordPanel },
  setup() {
    return { args };
  },
  template: '<ContainerChangeYourPasswordPanel v-bind="args" />',
});

export const Default = Template.bind({});
Default.args = {};
