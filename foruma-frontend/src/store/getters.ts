import { AuthenticationSession, ChangedEvent, State, SubForum } from "vue";

export const authenticated = (state: State): boolean | undefined =>
  !!state.data.authentication;

export const sessions = (state: State): AuthenticationSession[] =>
  state.data.authentication?.sessions || [];

export const subForums = (state: State): SubForum[] | undefined =>
  state.data.forum?.subForums;

export const username = (state: State): string | undefined =>
  state.data.authentication?.username;

export const loginChangedEvent = (state: State): ChangedEvent | undefined =>
  state.events.loginChanged;

export const passwordChangedEvent = (state: State): ChangedEvent | undefined =>
  state.events.passwordChanged;

export const signupChangedEvent = (state: State): ChangedEvent | undefined =>
  state.events.signupChanged;
