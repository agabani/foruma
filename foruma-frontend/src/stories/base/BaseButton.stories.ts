import { Meta, Story } from "@storybook/vue3";
import BaseButton from "./BaseButton.vue";
import {
  baseButtonEvents,
  BaseButtonProps,
  baseButtonPropSizes,
  baseButtonPropsType,
} from "./BaseButton.types";
import metaEvents from "../helpers/meta-events";

export default {
  title: "Base/BaseButton",
  component: BaseButton,
  argTypes: {
    size: {
      control: { type: "select", options: baseButtonPropSizes },
    },
    type: {
      control: { type: "select", options: baseButtonPropsType },
    },
    ...metaEvents(baseButtonEvents),
  },
} as Meta;

const Template: Story<BaseButtonProps> = (args: BaseButtonProps) => ({
  components: { BaseButton },
  setup() {
    return { args };
  },
  template: '<BaseButton v-bind="args" />',
});

export const Default = Template.bind({});
Default.args = { label: "Label" };

export const Primary = Template.bind({});
Primary.args = { ...Default.args, primary: true };

export const Secondary = Template.bind({});
Secondary.args = { ...Default.args, primary: false };

export const Small = Template.bind({});
Small.args = { ...Default.args, size: "small" };

export const Medium = Template.bind({});
Medium.args = { ...Default.args, size: "medium" };

export const Large = Template.bind({});
Large.args = { ...Default.args, size: "large" };

export const FullWidth = Template.bind({});
FullWidth.args = { ...Default.args, fullWidth: true };

export const Button = Template.bind({});
Button.args = { ...Default.args, type: "button" };

export const Reset = Template.bind({});
Reset.args = { ...Default.args, type: "reset" };

export const Submit = Template.bind({});
Submit.args = { ...Default.args, type: "submit" };
