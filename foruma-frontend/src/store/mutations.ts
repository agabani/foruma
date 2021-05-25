import { State, ChangedEvent, PasswordChangedEvent } from "@vue/runtime-core";

export const login = (state: State, username: string): void => {
  state.data.authentication = { username };
};

export const logout = (state: State): void => {
  state.data.authentication = undefined;
};

export const loginChangedEvent = (state: State, login: ChangedEvent): void => {
  state.events.loginChanged = login;
};

export const signupChangedEvent = (
  state: State,
  signup: ChangedEvent
): void => {
  state.events.signupChanged = signup;
};

export const passwordChangedEvent = (
  state: State,
  passwordChanged: PasswordChangedEvent
): void => {
  state.events.passwordChanged = passwordChanged;
};
