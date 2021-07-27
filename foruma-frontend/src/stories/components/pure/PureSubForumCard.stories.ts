import { Story } from "@storybook/vue3";
import metaEvents from "../../helpers/meta-events";
import {
  pureSubForumCardEvents,
  PureSubForumCardProps,
} from "./PureSubForumCard.types";
import PureSubForumCard from "./PureSubForumCard.vue";

export default {
  title: "Components/Pure/PureSubForumCard",
  component: PureSubForumCard,
  argTypes: {
    ...metaEvents(pureSubForumCardEvents),
  },
};

const Template: Story<PureSubForumCardProps> = (args) => ({
  components: { PureSubForumCard },
  setup() {
    return { args };
  },
  template: `
    <PureSubForumCard v-bind="args" />
    `,
});

export const Default = Template.bind({});
Default.args = {
  name: "Name",
};
