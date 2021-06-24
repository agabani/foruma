import type {
  AuthenticationSession,
  ChangedEvent,
  State,
} from "@vue/runtime-core";

export const login = (state: State, username: string): void => {
  state.data.authentication = { username, sessions: [] };
};

export const logout = (state: State): void => {
  state.data.authentication = undefined;
};

export const loginChangedEvent = (state: State, login: ChangedEvent): void => {
  state.events.loginChanged = login;
};

export const sessionChangedEvent = (
  state: State,
  payload: SessionChangedEventPayload
): void => {
  if (state.data.authentication) {
    state.data.authentication.sessions = payload.data;
  } else {
    throw new Error("unexpected state");
  }
};

export interface SessionChangedEventPayload {
  data: AuthenticationSession[];
  event: ChangedEvent;
}

export const signupChangedEvent = (
  state: State,
  signup: ChangedEvent
): void => {
  state.events.signupChanged = signup;
};

export const passwordChangedEvent = (
  state: State,
  passwordChanged: ChangedEvent
): void => {
  state.events.passwordChanged = passwordChanged;
};
