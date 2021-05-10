import { State } from "@vue/runtime-core";

export const authenticate = (state: State, username: string): void => {
  state.authentication = {
    authenticated: true,
    username,
  };
};

export const unauthenticate = (state: State): void => {
  state.authentication = {
    authenticated: false,
    username: undefined,
  };
};
