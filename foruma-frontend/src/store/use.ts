import { InjectionKey, State } from "vue";
import { Store, useStore as baseUseStore } from "vuex";

export const key: InjectionKey<Store<State>> = Symbol();

export function useStore(): Store<State> {
  return baseUseStore(key);
}
