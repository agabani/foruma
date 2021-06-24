import { app } from "@storybook/vue3";
import { action } from "@storybook/addon-actions";
import { ChangedEvent, State } from "vue";
import { createStore, Store } from "vuex";
import { key } from "../../store/use";

export function appUseStore(configure?: (store: Store<State>) => void): void {
  const store = createStore<State>({
    state: {
      data: {
        authentication: {
          username: "John Smith",
          sessions: [
            {
              id: "1",
              isCurrentSession: true,
              browser: "Chrome",
              operatingSystem: "Linux",
              lastActiveDate: new Date("2021-06-19T22:55:55"),
            },
            {
              id: "2",
              isCurrentSession: false,
              browser: "Safari",
              operatingSystem: "Mac OS",
              lastActiveDate: new Date("2021-06-19T22:56:09"),
            },
            {
              id: "3",
              isCurrentSession: false,
              browser: "Edge",
              operatingSystem: "Windows",
              lastActiveDate: new Date("2021-06-19T22:56:17"),
            },
          ],
        },
      },
      events: {
        passwordChanged: undefined,
        loginChanged: undefined,
        signupChanged: undefined,
      },
    },
    actions: {
      changeOwnPassword(context, payload) {
        const event: ChangedEvent = {
          eventDate: new Date(),
          error: undefined,
        };

        context.commit("passwordChanged", event);
        action("changeOwnPassword")(payload);
      },
      deleteOwnAccount(context, payload) {
        action("deleteOwnAccount")(payload);
      },
      deleteSession(context, payload) {
        context.commit("sessionDeleted", payload);
        action("deleteSession")(payload);
      },
      login(context, payload) {
        const event: ChangedEvent = {
          eventDate: new Date(),
          error: {
            title: "Storybook",
            message: "The login request was accepted by the mock store!",
          },
        };

        context.commit("loginChanged", event);
        action("login")(payload);
      },
      logout(context, payload) {
        action("logout")(payload);
      },
      navigateToAccountSettings(context, payload) {
        action("navigateToAccountSettings")(payload);
      },
      navigateToHome(context, payload) {
        action("navigateToHome")(payload);
      },
      navigateToLogin(context, payload) {
        action("navigateToLogin")(payload);
      },
      navigateToSignup(context, payload) {
        action("navigateToSignup")(payload);
      },
      signup(context, payload) {
        const event: ChangedEvent = {
          eventDate: new Date(),
          error: {
            title: "Storybook",
            message: "The signup request was accepted by the mock store!",
          },
        };

        context.commit("signupChanged", event);
        action("signup")(payload);
      },
    },
    getters: {
      username: (state: State) => state.data.authentication?.username,
      sessions: (state: State) => state.data.authentication?.sessions,
      loginChangedEvent: (state: State) => state.events.loginChanged,
      passwordChangedEvent: (state: State) => state.events.passwordChanged,
      signupChangedEvent: (state: State) => state.events.signupChanged,
    },
    mutations: {
      loginChanged(state, payload: ChangedEvent) {
        state.events.loginChanged = payload;
      },
      passwordChanged(state, payload: ChangedEvent) {
        state.events.passwordChanged = payload;
      },
      sessionDeleted(state, payload: { id: string }) {
        if (state.data.authentication) {
          state.data.authentication.sessions =
            state.data.authentication.sessions.filter(
              (session) => session.id !== payload.id
            );
        }
      },
      signupChanged(state, payload: ChangedEvent) {
        state.events.signupChanged = payload;
      },
    },
  });

  if (configure) {
    configure(store);
  }

  app.use(store, key);
}