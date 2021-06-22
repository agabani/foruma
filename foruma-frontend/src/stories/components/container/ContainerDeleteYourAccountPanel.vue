<template>
  <PureDeleteYourAccountPanel @clicked="onClick" />
</template>

<script lang="ts">
import { useStore } from "../../../store/use";
import { defineComponent } from "vue";
import PureDeleteYourAccountPanel from "../pure/PureDeleteYourAccountPanel.vue";

export default defineComponent({
  name: "ContainerDeleteYourAccountPanel",
  components: {
    PureDeleteYourAccountPanel,
  },
  setup() {
    const store = useStore();

    return {
      deleteOwnAccount: () => store.dispatch("deleteOwnAccount"),
    };
  },
  methods: {
    onClick() {
      this.deleteOwnAccount();
    },
  },
});
</script>
