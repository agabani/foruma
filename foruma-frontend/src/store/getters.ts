import { State, PasswordChanged, Signup } from "@vue/runtime-core";

export const authenticated = (state: State): boolean | undefined =>
  state.authentication.authenticated;

export const passwordChanged = (state: State): PasswordChanged | undefined =>
  state.authentication.passwordChanged;

export const signup = (state: State): Signup | undefined =>
  state.authentication.signup;

export const username = (state: State): string | undefined =>
  state.authentication.username;
