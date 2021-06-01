import PureSessionsPanel from "./PureSessionsPanel.vue";

export default {
  title: "Components/PureSessionsPanel",
  component: PureSessionsPanel,
  argTypes: {},
};

const Template = (args) => ({
  components: { PureSessionsPanel },
  setup() {
    return { args };
  },
  template: `<PureSessionsPanel v-bind="args" />`,
});

export const Default = Template.bind({});
Default.args = { sessions: [] };

export const Sessions = Template.bind({});
Sessions.args = {
  ...Default.args,
  sessions: [
    {
      id: "1",
      browser: "Chrome",
      operatingSystem: "Linux",
      lastActiveDate: new Date("2021-06-19T22:55:55"),
    },
    {
      id: "2",
      browser: "Safari",
      operatingSystem: "Mac OS",
      lastActiveDate: new Date("2021-06-19T22:56:09"),
    },
    {
      id: "3",
      browser: "Edge",
      operatingSystem: "Windows",
      lastActiveDate: new Date("2021-06-19T22:56:17"),
    },
  ],
};
