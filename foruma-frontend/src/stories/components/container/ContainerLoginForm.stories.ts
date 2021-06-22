import { Story } from "@storybook/vue3";
import ContainerLoginForm from "./ContainerLoginForm.vue";
import { appUseStore } from "../../helpers/store";

appUseStore();

export default {
  title: "Components/Container/ContainerLoginForm",
  component: ContainerLoginForm,
  argTypes: {},
};

const Template: Story = (args) => ({
  components: { ContainerLoginForm },
  setup() {
    return { args };
  },
  template: '<ContainerLoginForm v-bind="args" />',
});

export const Default = Template.bind({});
Default.args = {};
