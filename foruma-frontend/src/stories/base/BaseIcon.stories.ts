import { Story } from "@storybook/vue3";
import { BaseIconProps, baseIconPropsIconName } from "./BaseIcon.types";
import BaseIcon from "./BaseIcon.vue";

export default {
  title: "Base/BaseIcon",
  component: BaseIcon,
  argTypes: {
    iconName: {
      control: { type: "select", options: baseIconPropsIconName },
    },
  },
};

const Template: Story<BaseIconProps> = (args) => ({
  components: { BaseIcon },
  setup() {
    return { args };
  },
  template: `<BaseIcon v-bind="args" />`,
});

export const Default = Template.bind({});
Default.args = {};

export const Size24x24 = Template.bind({});
Size24x24.args = { ...Default.args, height: 24, width: 24 };

export const Size96x96 = Template.bind({});
Size96x96.args = { ...Default.args, height: 96, width: 96 };

export const Checkmark = Template.bind({});
Checkmark.args = { ...Default.args, iconName: "checkmark" };

export const Chrome = Template.bind({});
Chrome.args = { ...Default.args, iconName: "chrome" };

export const Edge = Template.bind({});
Edge.args = { ...Default.args, iconName: "edge" };

export const Firefox = Template.bind({});
Firefox.args = { ...Default.args, iconName: "firefox" };

export const Info = Template.bind({});
Info.args = { ...Default.args, iconName: "info" };

export const Placeholder = Template.bind({});
Placeholder.args = { ...Default.args, iconName: "placeholder" };

export const Safari = Template.bind({});
Safari.args = { ...Default.args, iconName: "safari" };

export const Trash = Template.bind({});
Trash.args = { ...Default.args, iconName: "trash" };

export const Warning = Template.bind({});
Warning.args = { ...Default.args, iconName: "warning" };
