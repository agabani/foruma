import { InjectionKey, State } from "vue";
import { createStore, Store, useStore as baseUseStore } from "vuex";
import * as actions from "./actions";
import * as getters from "./getters";
import * as mutations from "./mutations";

export const key: InjectionKey<Store<State>> = Symbol();

const store = createStore<State>({
  state: {
    authentication: {
      authenticated: false,
      username: undefined,
    },
  },
  actions,
  getters,
  mutations,
  modules: {},
});

if (module.hot) {
  module.hot.accept(["./actions", "./getters", "./mutations"], () => {
    store.hotUpdate({
      actions: require("./actions"),
      getters: require("./getters"),
      mutations: require("./mutations"),
    });
  });
}

export function useStore(): Store<State> {
  return baseUseStore(key);
}

export default store;
