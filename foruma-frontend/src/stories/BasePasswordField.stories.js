import BasePasswordField from "./BasePasswordField.vue";

export default {
  title: "Components/BasePasswordField",
  component: BasePasswordField,
  argTypes: {
    size: {
      control: { type: "select", options: ["small", "medium", "large"] },
    },
    onChange: {},
  },
};

const Template = (args) => ({
  components: { BasePasswordField },
  setup() {
    return { args };
  },
  template: '<BasePasswordField v-bind="args" />',
});

export const Default = Template.bind({});
Default.args = {};

export const Placeholder = Template.bind({});
Placeholder.args = { ...Default.args, placeholder: "Placeholder" };

export const InitialValue = Template.bind({});
InitialValue.args = { ...Default.args, initialValue: "Initial Value" };

export const Small = Template.bind({});
Small.args = { ...Default.args, size: "small" };

export const Medium = Template.bind({});
Medium.args = { ...Default.args, size: "medium" };

export const Large = Template.bind({});
Large.args = { ...Default.args, size: "large" };
