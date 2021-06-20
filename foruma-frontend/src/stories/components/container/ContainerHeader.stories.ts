import { Story } from "@storybook/vue3";
import ContainerHeader from "./ContainerHeader.vue";
import { appUseStore } from "../../helpers/store";

appUseStore();

export default {
  title: "Components/Container/ContainerHeader",
  component: ContainerHeader,
  argTypes: {},
};

const Template: Story = (args) => ({
  components: { ContainerHeader },
  setup() {
    return { args };
  },
  template: '<ContainerHeader v-bind="args" />',
});

export const Default = Template.bind({});
Default.args = {};
