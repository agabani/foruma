<template>
  <DeleteYourAccountPanelContainer />
</template>

<script lang="ts">
import { defineComponent } from "vue";
import DeleteYourAccountPanelContainer from "@/components/DeleteYourAccountPanelContainer.vue";

export default defineComponent({
  name: "AccountSettingsPage",
  components: {
    DeleteYourAccountPanelContainer,
  },
});
</script>
