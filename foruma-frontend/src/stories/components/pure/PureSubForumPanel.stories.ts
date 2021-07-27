import { Story } from "@storybook/vue3";
import metaEvents from "../../helpers/meta-events";
import {
  pureSubForumPanelEvents,
  PureSubForumPanelProps,
} from "./PureSubForumPanel.types";
import PureSubForumPanel from "./PureSubForumPanel.vue";

export default {
  title: "Components/Pure/PureSubForumPanel",
  component: PureSubForumPanel,
  argTypes: {
    ...metaEvents(pureSubForumPanelEvents),
  },
};

const Template: Story<PureSubForumPanelProps> = (args) => ({
  components: { PureSubForumPanel },
  setup() {
    return { args };
  },
  template: `
    <PureSubForumPanel v-bind="args" />
    `,
});

export const Default = Template.bind({});
Default.args = {
  subForums: [],
};

export const Example = Template.bind({});
Example.args = {
  ...Default.args,
  subForums: [
    {
      id: "1",
      name: "News & Announcements",
    },
    {
      id: "2",
      name: "General Discussion",
    },
    {
      id: "3",
      name: "Home Cinema & Hi-Fi",
    },
    {
      id: "4",
      name: "TV, Film & Radio",
    },
    {
      id: "5",
      name: "Computer & Gadgets",
    },
    {
      id: "6",
      name: "Off Topic",
    },
  ],
};
