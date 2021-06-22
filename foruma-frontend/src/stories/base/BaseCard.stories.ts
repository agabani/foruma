import { Story } from "@storybook/vue3";
import { BaseCardProp } from "./BaseCard.types";
import BaseCard from "./BaseCard.vue";

export default {
  title: "Base/BaseCard",
  component: BaseCard,
  argTypes: {},
};

const Template: Story<BaseCardProp> = (args: BaseCardProp) => ({
  components: { BaseCard },
  setup() {
    return { args };
  },
  template: `
      <BaseCard v-bind="args">
          <template v-if="${!!args.default}" v-slot>
              ${args.default}
          </template>
      </BaseCard>
      `,
});

export const Default = Template.bind({});
Default.args = { default: "Default" };
