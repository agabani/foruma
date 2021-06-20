import PureAccountSettingsPage from "./PureAccountSettingsPage.vue";
import { Sessions } from "../PureSessionsPanel.stories";

export default {
  title: "Pages/PureAccountSettingsPage",
  component: PureAccountSettingsPage,
  argTypes: {},
};

const Template = (args) => ({
  components: { PureAccountSettingsPage },
  setup() {
    return { args };
  },
  template: `<PureAccountSettingsPage v-bind="args" />`,
});

export const Default = Template.bind({});
Default.args = {
  sessions: [],
};

export const Example = Template.bind({});
Example.args = {
  sessions: Sessions.args.sessions,
};
