// eslint-disable-next-line @typescript-eslint/no-unused-vars
import { ComponentCustomProperties } from "vue";
import { Store } from "vuex";

declare module "@vue/runtime-core" {
  // declare your own store states
  interface State {
    authentication: {
      authenticated: boolean | undefined;
      username: string | undefined;
    };
  }

  // provide typings for `this.$store`
  interface ComponentCustomProperties {
    $store: Store<State>;
  }
}
