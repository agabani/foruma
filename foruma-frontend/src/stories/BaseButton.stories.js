import BaseButton from "./BaseButton.vue";

export default {
  title: "Components/BaseButton",
  component: BaseButton,
  argTypes: {
    size: {
      control: { type: "select", options: ["small", "medium", "large"] },
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
Primary.args = {
  ...Default.args,
  primary: true,
};

export const Secondary = Template.bind({});
Secondary.args = {
  ...Default.args,
  primary: false,
};

export const Small = Template.bind({});
Small.args = {
  ...Default.args,
  size: "small",
};

export const Medium = Template.bind({});
Medium.args = {
  ...Default.args,
  size: "medium",
};

export const Large = Template.bind({});
Large.args = {
  ...Default.args,
  size: "large",
};
