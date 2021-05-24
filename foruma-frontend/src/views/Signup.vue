<template>
  <ContainerSignupForm />
</template>

<script lang="ts">
import { defineComponent } from "vue";
import ContainerSignupForm from "@/components/ContainerSignupForm.vue";

export default defineComponent({
  name: "Signup",
  components: {
    ContainerSignupForm,
  },
});
</script>
