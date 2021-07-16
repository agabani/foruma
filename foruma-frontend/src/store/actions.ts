import router from "@/router";
import axios from "axios";
import type { ChangedEvent } from "@vue/runtime-core";
import { Commit } from "vuex";
import { SessionChangedEventPayload } from "./mutations";
import type {
  LoginPayload,
  ChangePasswordPayload,
  SignupPayload,
  DeleteSessionPayload,
} from "./types";
import {
  mutationChangeAccountAuthenticationPassword,
  mutationDeleteAccountAuthenticationSession,
  queryCurrentAccount,
  queryCurrentAccountAuthenticationSessions,
} from "@/graphql";

const api = axios.create({
  baseURL: process.env.VUE_APP_API_BASE_URL,
  validateStatus: null,
  withCredentials: true,
});

export const initialize = async ({
  commit,
}: {
  commit: Commit;
}): Promise<void> => {
  const currentAccount = await queryCurrentAccount();

  if (currentAccount) {
    commit("login", currentAccount.username);
  } else {
    commit("logout");
  }
};

export const changeOwnPassword = async (
  { commit }: { commit: Commit },
  payload: ChangePasswordPayload
): Promise<void> => {
  const response = await mutationChangeAccountAuthenticationPassword(payload);

  const event: ChangedEvent = {
    eventDate: new Date(),
    error: undefined,
  };

  if (response.success) {
    commit("passwordChangedEvent", event);
  } else {
    switch (response.errorCode) {
      case "incorrect_password":
        event.error = {
          title: "Uh oh, something went wrong",
          message: "Your current password is incorrect!",
        };
        commit("passwordChangedEvent", event);
        break;
      case "unauthenticated":
        event.error = {
          title: "Uh oh, something went wrong",
          message: "You are somehow not logged in... ¯\\_(ツ)_/¯",
        };
        commit("passwordChangedEvent", event);
        break;
      default:
        throw new Error("unexpected response");
    }
  }
};

export const deleteOwnAccount = async ({
  commit,
}: {
  commit: Commit;
}): Promise<void> => {
  const terminateResponse = await api.post("/api/v1/account/terminate");

  switch (terminateResponse.status) {
    case 200:
      {
        commit("logout", terminateResponse.data.username);
        router.push("/");
      }
      break;
    default:
      throw new Error("unexpected response");
  }
};

export const deleteSession = async (
  { commit }: { commit: Commit },
  payload: DeleteSessionPayload
): Promise<void> => {
  const response = await mutationDeleteAccountAuthenticationSession(payload.id);

  if (response) {
    await getSessions({ commit });
  } else {
    throw new Error("unexpected response");
  }
};

export const getSessions = async ({
  commit,
}: {
  commit: Commit;
}): Promise<void> => {
  const response = await queryCurrentAccountAuthenticationSessions();

  if (response) {
    const payload: SessionChangedEventPayload = {
      event: {
        eventDate: new Date(),
        error: undefined,
      },
      data: response.map((s) => {
        return {
          browser: s.browser,
          id: s.id,
          isCurrentSession: s.isCurrentSession,
          lastActiveDate: new Date(s.lastActiveDate),
          location: s.location,
          operatingSystem: s.operatingSystem,
        };
      }),
    };

    commit("sessionChangedEvent", payload);
  } else {
    commit("logout");
  }
};

export const login = async (
  { commit }: { commit: Commit },
  payload: LoginPayload
): Promise<void> => {
  const loginResponse = await api.post("/api/v1/authentication/login", {
    username: payload.username,
    password: payload.password,
  });

  switch (loginResponse.status) {
    case 200:
      break;
    case 401:
      {
        const event: ChangedEvent = {
          eventDate: new Date(),
          error: {
            title: "Uh oh, something went wrong",
            message: "Sorry! There was a problem with your request!",
          },
        };
        commit("loginChangedEvent", event);
      }
      return;
    default:
      throw new Error("unexpected response");
  }

  const currentAccount = await queryCurrentAccount();

  if (currentAccount) {
    commit("login", currentAccount.username);
  }
};

export const logout = async ({ commit }: { commit: Commit }): Promise<void> => {
  const logoutResponse = await api.post("/api/v1/authentication/logout");

  switch (logoutResponse.status) {
    case 200:
      {
        commit("logout");
        router.push("/");
      }
      break;
    default:
      throw new Error("unexpected response");
  }
};

export const navigateToAccountSettings = async (): Promise<void> => {
  router.push("/account-settings");
};

export const navigateToHome = async (): Promise<void> => {
  router.push("/");
};

export const navigateToLogin = async (): Promise<void> => {
  router.push("/login");
};

export const navigateToSignup = async (): Promise<void> => {
  router.push("/signup");
};

export const signup = async (
  { commit }: { commit: Commit },
  payload: SignupPayload
): Promise<void> => {
  const signupResponse = await api.post("/api/v1/authentication/signup", {
    username: payload.username,
    password: payload.password,
  });

  switch (signupResponse.status) {
    case 200:
      {
        commit("login", signupResponse.data.username);
      }
      break;
    case 401:
      {
        const event: ChangedEvent = {
          eventDate: new Date(),
          error: {
            title: "Uh oh, something went wrong",
            message: "Sorry! There was a problem with your request!",
          },
        };
        commit("signupChangedEvent", event);
      }
      return;
    default:
      throw new Error("unexpected response");
  }

  const currentAccount = await queryCurrentAccount();

  if (currentAccount) {
    commit("login", currentAccount.username);
  }
};
