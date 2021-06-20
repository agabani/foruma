<template>
  <div>
    <ContainerHeader />
    <ContainerLoginForm />
  </div>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import ContainerHeader from "../../components/container/ContainerHeader.vue";
import ContainerLoginForm from "../../components/container/ContainerLoginForm.vue";

export default defineComponent({
  name: "PureLoginScreen",
  components: {
    ContainerHeader,
    ContainerLoginForm,
  },
});
</script>
