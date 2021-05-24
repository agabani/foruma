<template>
  <ContainerLoginForm />
</template>

<script lang="ts">
import { defineComponent } from "vue";
import ContainerLoginForm from "../components/ContainerLoginForm.vue";

export default defineComponent({
  name: "Login",
  components: {
    ContainerLoginForm,
  },
});
</script>
