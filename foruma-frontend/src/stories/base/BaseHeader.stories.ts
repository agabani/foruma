import { Story } from "@storybook/vue3";
import { BaseHeaderProps } from "./BaseHeader.types";
import BaseHeader from "./BaseHeader.vue";

export default {
  title: "Base/BaseHeader",
  component: BaseHeader,
  argTypes: {
    brand: {
      control: "text",
      type: String,
    },
    default: {
      control: "text",
      type: String,
    },
  },
};

const Template: Story<BaseHeaderProps> = (args) => ({
  components: { BaseHeader },
  setup() {
    return { args };
  },
  template: `
      <BaseHeader v-bind="args">
        <template v-if="${!!args.brand}" v-slot:brand>
            ${args.brand}
        </template>
          <template v-if="${!!args.default}" v-slot>
              ${args.default}
          </template>
      </BaseHeader>
      `,
});

export const Default = Template.bind({});
Default.args = { default: "Default" };

export const Brand = Template.bind({});
Brand.args = { ...Default.args, brand: "Brand" };
