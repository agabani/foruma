import PureSessionCard from "./PureSessionCard.vue";

export default {
  title: "Components/PureSessionCard",
  component: PureSessionCard,
  argTypes: {
    browser: {
      control: {
        type: "select",
        options: ["Chrome", "Edge", "Firefox", "Safari"],
      },
    },
    operatingSystem: {
      control: {
        type: "select",
        options: ["Linux", "Mac OS", "Windows"],
      },
    },
  },
};

const Template = (args) => ({
  components: { PureSessionCard },
  setup() {
    return { args };
  },
  template: `<PureSessionCard v-bind="args" />`,
});

export const Default = Template.bind({});
Default.args = {
  browser: "Browser",
  operatingSystem: "Operating System",
  lastActiveDate: new Date("2021-06-19T22:57:50"),
};

export const Chrome = Template.bind({});
Chrome.args = {
  ...Default.args,
  browser: "Chrome",
};

export const Edge = Template.bind({});
Edge.args = {
  ...Default.args,
  browser: "Edge",
};

export const Firefox = Template.bind({});
Firefox.args = {
  ...Default.args,
  browser: "Firefox",
};

export const Safari = Template.bind({});
Safari.args = {
  ...Default.args,
  browser: "Safari",
};

export const Linux = Template.bind({});
Linux.args = {
  ...Default.args,
  operatingSystem: "Linux",
};

export const MacOS = Template.bind({});
MacOS.args = {
  ...Default.args,
  operatingSystem: "Mac OS",
};

export const Windows = Template.bind({});
Windows.args = {
  ...Default.args,
  operatingSystem: "Windows",
};
