import {
  State,
  LoginChangedEvent,
  PasswordChangedEvent,
  SignupChangedEvent,
} from "@vue/runtime-core";

export const authenticated = (state: State): boolean | undefined =>
  !!state.data.authentication;

export const username = (state: State): string | undefined =>
  state.data.authentication?.username;

export const loginChangedEvent = (
  state: State
): LoginChangedEvent | undefined => state.events.loginChanged;

export const passwordChangedEvent = (
  state: State
): PasswordChangedEvent | undefined => state.events.passwordChanged;

export const signupChangedEvent = (
  state: State
): SignupChangedEvent | undefined => state.events.signupChanged;
