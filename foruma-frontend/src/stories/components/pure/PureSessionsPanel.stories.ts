import metaEvents from "../../helpers/meta-events";
import { Story } from "@storybook/vue3";
import {
  pureSessionsPanelEvents,
  PureSessionsPanelProps,
} from "./PureSessionsPanel.types";
import PureSessionsPanel from "./PureSessionsPanel.vue";

export default {
  title: "Components/Pure/PureSessionsPanel",
  component: PureSessionsPanel,
  argTypes: {
    ...metaEvents(pureSessionsPanelEvents),
  },
};

const Template: Story<PureSessionsPanelProps> = (args) => ({
  components: { PureSessionsPanel },
  setup() {
    return { args };
  },
  template: `<PureSessionsPanel v-bind="args" />`,
});

export const Default = Template.bind({});
Default.args = { sessions: [] };

export const Example = Template.bind({});
Example.args = {
  ...Default.args,
  sessions: [
    {
      id: "1",
      isCurrentSession: true,
      browser: "Chrome",
      operatingSystem: "Linux",
      lastActiveDate: new Date("2021-06-19T22:55:55"),
      location: "United Kingdom",
    },
    {
      id: "2",
      isCurrentSession: false,
      browser: "Safari",
      operatingSystem: "Mac OS",
      lastActiveDate: new Date("2021-06-19T22:56:09"),
      location: "United States of America",
    },
    {
      id: "3",
      isCurrentSession: false,
      browser: "Edge",
      operatingSystem: "Windows",
      lastActiveDate: new Date("2021-06-19T22:56:17"),
      location: null,
    },
  ],
};
