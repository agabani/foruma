import BaseButton from "./BaseButton.vue";

export default {
  title: "Components/BaseButton",
  component: BaseButton,
  argTypes: {
    size: {
      control: { type: "select", options: ["small", "medium", "large"] },
    },
    type: {
      control: { type: "select", options: ["button", "reset", "submit"] },
    },
    onClick: {},
  },
};

const Template = (args) => ({
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
