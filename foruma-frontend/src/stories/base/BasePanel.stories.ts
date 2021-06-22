import { Story } from "@storybook/vue3";
import { BasePanelProps } from "./BasePanel.types";
import BasePanel from "./BasePanel.vue";

export default {
  title: "Base/BasePanel",
  component: BasePanel,
  argTypes: {
    header: {
      control: "text",
      type: String,
    },
    default: {
      control: "text",
      type: String,
    },
    footer: {
      control: "text",
      type: String,
    },
  },
};

const Template: Story<BasePanelProps> = (args) => ({
  components: { BasePanel },
  setup() {
    return { args };
  },
  template: `
    <BasePanel v-bind="args">
        <template v-if="${!!args.header}" v-slot:header>
            ${args.header}
        </template>
        <template v-if="${!!args.default}" v-slot>
            ${args.default}
        </template>
        <template v-if="${!!args.footer}" v-slot:footer>
            ${args.footer}
        </template>
    </BasePanel>
    `,
});

export const Default = Template.bind({});
Default.args = { default: "Default" };

export const Empty = Template.bind({});
Empty.args = { ...Default.args, default: undefined };

export const Header = Template.bind({});
Header.args = { ...Default.args, header: "Header" };

export const Footer = Template.bind({});
Footer.args = { ...Default.args, footer: "Footer" };

export const Full = Template.bind({});
Full.args = {
  ...Default.args,
  header: "Header",
  default: "Default",
  footer: "Footer",
};
