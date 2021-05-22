<template>
  <div class="delete-your-account-panel">
    <BasePanel>
      <template v-slot:header>
        <span>Delete your account</span>
      </template>
      <template v-slot>
        <p>
          If you delete your account all your information will be gone forever.
          This step cannot be undone, so please, be sure!
        </p>
        <BaseButton label="Delete account" primary @click="$emit('click')" />
      </template>
    </BasePanel>
  </div>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import BaseButton from "./BaseButton.vue";
import BasePanel from "./BasePanel.vue";

export default defineComponent({
  name: "DeleteYourAccountPanel",
  components: {
    BaseButton,
    BasePanel,
  },
  emits: ["click"],
});
</script>

<style lang="css" scoped>
.delete-your-account-panel {
}
</style>
