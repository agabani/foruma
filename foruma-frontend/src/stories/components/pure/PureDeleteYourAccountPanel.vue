<template>
  <div class="pure-delete-your-account-panel">
    <BasePanel>
      <template v-slot:header>
        <span>Delete your account</span>
      </template>
      <template v-slot>
        <p>
          If you delete your account all your information will be gone forever.
          This step cannot be undone, so please, be sure!
        </p>
        <BaseButton label="Delete account" primary @click="onClick" />
      </template>
    </BasePanel>
  </div>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import BaseButton from "../../base/BaseButton.vue";
import BasePanel from "../../base/BasePanel.vue";
import { pureDeleteYourAccountPanelEvents } from "./PureDeleteYourAccountPanel.types";

export default defineComponent({
  name: "PureDeleteYourAccountPanel",
  components: {
    BaseButton,
    BasePanel,
  },
  emits: [...pureDeleteYourAccountPanelEvents],
  methods: {
    onClick() {
      this.$emit("clicked");
    },
  },
});
</script>

<style lang="css" scoped>
.pure-delete-your-account-panel {
}
</style>
