<template>
  <router-view />
</template>

<script lang="ts">
import { defineComponent } from "vue";
import { useStore } from "./store/use";

export default defineComponent({
  name: "App",
  components: {},
  created: function () {
    if (sessionStorage.spaFallbackRedirect) {
      const redirect = sessionStorage.spaFallbackRedirect;
      delete sessionStorage.spaFallbackRedirect;
      delete sessionStorage.spaFallbackSegment;
      this.$router.push(redirect);
    }

    const store = useStore();

    store.dispatch("initialize");
  },
});
</script>

<style lang="scss">
#app {
  font-family: Helvetica, Verdana, Arial;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  // text-align: center;
  // color: #2c3e50;
}

#nav {
  padding: 30px;

  a {
    font-weight: bold;
    color: #2c3e50;

    &.router-link-exact-active {
      color: #42b983;
    }
  }
}
</style>
