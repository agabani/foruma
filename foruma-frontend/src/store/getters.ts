import { State } from "@vue/runtime-core";

export const authenticated = (state: State): boolean | undefined =>
  state.authentication.authenticated;

export const username = (state: State): string | undefined =>
  state.authentication.username;
