import { Story } from "@storybook/vue3";
import metaEvents from "../helpers/meta-events";
import {
  baseTextFieldEvents,
  BaseTextFieldProps,
  baseTextFieldPropsSizes,
} from "./BaseTextField.types";
import BaseTextField from "./BaseTextField.vue";

export default {
  title: "Base/BaseTextField",
  component: BaseTextField,
  argTypes: {
    size: {
      control: { type: "select", options: baseTextFieldPropsSizes },
    },
    ...metaEvents(baseTextFieldEvents),
  },
};

const Template: Story<BaseTextFieldProps> = (args) => ({
  components: { BaseTextField },
  setup() {
    return { args };
  },
  template: '<BaseTextField v-bind="args" />',
});

export const Default = Template.bind({});
Default.args = {};

export const Placeholder = Template.bind({});
Placeholder.args = { ...Default.args, placeholder: "Placeholder" };

export const Value = Template.bind({});
Value.args = { ...Default.args, value: "Value" };

export const Small = Template.bind({});
Small.args = { ...Default.args, size: "small" };

export const Medium = Template.bind({});
Medium.args = { ...Default.args, size: "medium" };

export const Large = Template.bind({});
Large.args = { ...Default.args, size: "large" };

export const FullWidth = Template.bind({});
FullWidth.args = { ...Default.args, fullWidth: true };
