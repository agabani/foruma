import { Story } from "@storybook/vue3";
import { appUseStore } from "../../helpers/store";
import ContainerSessionsPanel from "./ContainerSessionsPanel.vue";

appUseStore();

export default {
  title: "Components/Container/ContainerSessionsPanel",
  component: ContainerSessionsPanel,
  argTypes: {},
};

const Template: Story = (args) => ({
  components: { ContainerSessionsPanel },
  setup() {
    return { args };
  },
  template: '<ContainerSessionsPanel v-bind="args" />',
});

export const Default = Template.bind({});
Default.args = {};
