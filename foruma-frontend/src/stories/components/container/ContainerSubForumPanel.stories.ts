import { Story } from "@storybook/vue3";
import { appUseStore } from "../../helpers/store";
import ContainerSubForumPanel from "./ContainerSubForumPanel.vue";

appUseStore();

export default {
  title: "Components/Container/ContainerSubForumPanel",
  component: ContainerSubForumPanel,
  argTypes: {},
};

const Template: Story = (args) => ({
  components: { ContainerSubForumPanel },
  setup() {
    return { args };
  },
  template: '<ContainerSubForumPanel v-bind="args" />',
});

export const Default = Template.bind({});
Default.args = {};
