import BaseCard from "./BaseCard.vue";

export default {
  title: "Components/BaseCard",
  component: BaseCard,
  argTypes: {},
};

const Template = (args) => ({
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
