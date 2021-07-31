import router from "@/router";
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
  mutationLogin,
  mutationLogoutCurrentAccount,
  mutationSignup,
  mutationTerminateCurrentAccount,
  queryCurrentAccount,
  queryCurrentAccountAuthenticationSessions,
} from "@/graphql";

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
  const response = await mutationTerminateCurrentAccount();

  if (response.success) {
    commit("logout");
    router.push("/");
  } else {
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

export const getSubForums = async (): Promise<void> => {
  // TODO: retrieve sub forums from graphql api.
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
  const response = await mutationLogin(payload);

  if (!response.success) {
    const event: ChangedEvent = {
      eventDate: new Date(),
      error: {
        title: "Uh oh, something went wrong",
        message: "Sorry! There was a problem with your request!",
      },
    };
    commit("loginChangedEvent", event);
    return;
  }

  const currentAccount = await queryCurrentAccount();

  if (currentAccount) {
    commit("login", currentAccount.username);
  }
};

export const logout = async ({ commit }: { commit: Commit }): Promise<void> => {
  const response = await mutationLogoutCurrentAccount();

  if (response.success) {
    commit("logout");
    router.push("/");
  } else {
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
  const response = await mutationSignup(payload);

  if (!response.success) {
    const event: ChangedEvent = {
      eventDate: new Date(),
      error: {
        title: "Uh oh, something went wrong",
        message: "Sorry! There was a problem with your request!",
      },
    };
    commit("signupChangedEvent", event);
    return;
  }

  const currentAccount = await queryCurrentAccount();

  if (currentAccount) {
    commit("login", currentAccount.username);
  }
};
