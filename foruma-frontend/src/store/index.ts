import { State } from "vue";
import { createStore } from "vuex";
import * as actions from "./actions";
import * as getters from "./getters";
import * as mutations from "./mutations";

const store = createStore<State>({
  state: {
    data: {
      authentication: undefined,
    },
    events: {
      passwordChanged: undefined,
      loginChanged: undefined,
      signupChanged: undefined,
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

export default store;
