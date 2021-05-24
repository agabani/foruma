import { State, PasswordChanged } from "@vue/runtime-core";

export const authenticate = (state: State, username: string): void => {
  state.authentication.authenticated = true;
  state.authentication.username = username;
};

export const unauthenticate = (state: State): void => {
  state.authentication.authenticated = false;
  state.authentication.username = undefined;
};

export const passwordChanged = (
  state: State,
  passwordChanged: PasswordChanged
): void => {
  state.authentication.passwordChanged = passwordChanged;
};
