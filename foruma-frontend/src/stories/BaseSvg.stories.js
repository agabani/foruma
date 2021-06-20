import BaseSvg from "./BaseSvg.vue";
import PlaceholderIcon from "./icons/PlaceholderIcon.vue";

export default {
  title: "Components/BaseSvg",
  component: BaseSvg,
  argTypes: {
    onSubmit: {},
  },
};

const Template = (args) => ({
  components: { BaseSvg, PlaceholderIcon },
  setup() {
    return { args };
  },
  template: `<BaseSvg v-bind="args"><PlaceholderIcon /></BaseSvg>`,
});

export const Default = Template.bind({});
Default.args = {
  viewBox: "0 0 48 48",
};

export const Size24x24 = Template.bind({});
Size24x24.args = {
  ...Default.args,
  height: 24,
  width: 24,
};

export const Size96x96 = Template.bind({});
Size96x96.args = {
  ...Default.args,
  height: 96,
  width: 96,
};

export const ViewBox = Template.bind({});
ViewBox.args = {
  ...Default.args,
  viewBox: "24 24 24 24",
};
