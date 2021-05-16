<template>
  <SignupForm />
</template>

<script lang="ts">
import { defineComponent } from "vue";
import SignupForm from "@/components/SignupForm.vue";

export default defineComponent({
  name: "Signup",
  components: {
    SignupForm,
  },
});
</script>
