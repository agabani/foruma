<template>
  <div>
    <ContainerHeader />
    <ContainerSignupForm />
  </div>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import ContainerHeader from "../../components/container/ContainerHeader.vue";
import ContainerSignupForm from "../../components/container/ContainerSignupForm.vue";

export default defineComponent({
  name: "PureSignupScreen",
  components: {
    ContainerHeader,
    ContainerSignupForm,
  },
});
</script>
