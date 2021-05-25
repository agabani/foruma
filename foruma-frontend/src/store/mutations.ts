import {
  State,
  LoginChangedEvent,
  SignupChangedEvent,
  PasswordChangedEvent,
} from "@vue/runtime-core";

export const authenticate = (state: State, username: string): void => {
  state.data.authentication = { username };
};

export const unauthenticate = (state: State): void => {
  state.data.authentication = undefined;
};

export const login = (state: State, login: LoginChangedEvent): void => {
  state.events.loginChanged = login;
};

export const signup = (state: State, signup: SignupChangedEvent): void => {
  state.events.signupChanged = signup;
};

export const passwordChanged = (
  state: State,
  passwordChanged: PasswordChangedEvent
): void => {
  state.events.passwordChanged = passwordChanged;
};
