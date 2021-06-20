import { Story } from "@storybook/vue3";
import ContainerSignupForm from "./ContainerSignupForm.vue";
import { appUseStore } from "../../helpers/store";

appUseStore();

export default {
  title: "Components/Container/ContainerSignupForm",
  component: ContainerSignupForm,
  argTypes: {},
};

const Template: Story = (args) => ({
  components: { ContainerSignupForm },
  setup() {
    return { args };
  },
  template: '<ContainerSignupForm v-bind="args" />',
});

export const Default = Template.bind({});
Default.args = {};
