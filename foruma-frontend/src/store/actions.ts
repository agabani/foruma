import axios from "axios";
import { ChangedEvent } from "vue";
import { Commit } from "vuex";
import type {
  LoginPayload,
  ChangePasswordPayload,
  SignupPayload,
} from "./types";

const api = axios.create({
  baseURL: process.env.VUE_APP_API_BASE_URL,
  validateStatus: null,
  withCredentials: true,
});

export const login = async (
  { commit }: { commit: Commit },
  payload: LoginPayload
): Promise<void> => {
  const loginResponse = await api.post("/api/authentication/login", {
    username: payload.username,
    password: payload.password,
  });

  if (loginResponse.status === 401) {
    const event: ChangedEvent = {
      eventDate: new Date(),
      error: {
        title: "Uh oh, something went wrong",
        message: "Sorry! There was a problem with your request!",
      },
    };
    commit("loginChangedEvent", event);
    return;
  } else if (loginResponse.status !== 200) {
    throw new Error("unexpected response");
  }

  const whoamiResponse = await api.get("/api/authentication/whoami");

  if (whoamiResponse.status === 200) {
    commit("login", whoamiResponse.data.username);
  } else {
    throw new Error("unexpected response");
  }
};

export const logout = async ({ commit }: { commit: Commit }): Promise<void> => {
  const logoutResponse = await api.post("/api/authentication/logout");

  if (logoutResponse.status !== 200) {
    throw new Error("unexpected response");
  }

  commit("logout");
};

export const signup = async (
  { commit }: { commit: Commit },
  payload: SignupPayload
): Promise<void> => {
  const signupResponse = await api.post("/api/authentication/signup", {
    username: payload.username,
    password: payload.password,
  });

  if (signupResponse.status === 200) {
    commit("login", signupResponse.data.username);
  } else if (signupResponse.status === 401) {
    const event: ChangedEvent = {
      eventDate: new Date(),
      error: {
        title: "Uh oh, something went wrong",
        message: "Sorry! There was a problem with your request!",
      },
    };
    commit("signupChangedEvent", event);
    return;
  } else {
    throw new Error("unexpected response");
  }

  const whoamiResponse = await api.get("/api/authentication/whoami");

  if (whoamiResponse.status === 200) {
    commit("login", whoamiResponse.data.username);
  } else {
    throw new Error("unexpected response");
  }
};

export const initialize = async ({
  commit,
}: {
  commit: Commit;
}): Promise<void> => {
  const whoamiResponse = await api.get("/api/authentication/whoami");

  if (whoamiResponse.status === 200) {
    commit("login", whoamiResponse.data.username);
  } else if (whoamiResponse.status === 401) {
    commit("logout");
  } else {
    throw new Error("unexpected response");
  }
};

export const terminateOwnAccount = async ({
  commit,
}: {
  commit: Commit;
}): Promise<void> => {
  const terminateResponse = await api.post("/api/account/terminate");

  if (terminateResponse.status === 200) {
    commit("logout", terminateResponse.data.username);
  } else {
    throw new Error("unexpected response");
  }
};

export const changeOwnPassword = async (
  { commit }: { commit: Commit },
  payload: ChangePasswordPayload
): Promise<void> => {
  const changePasswordResponse = await api.post(
    "/api/authentication/change-password",
    {
      oldPassword: payload.oldPassword,
      newPassword: payload.newPassword,
    }
  );

  const event: ChangedEvent = {
    eventDate: new Date(),
    error: undefined,
  };

  switch (changePasswordResponse.status) {
    case 200:
      break;
    case 400:
      event.error = {
        title: "Uh oh, something went wrong",
        message: "Your current password is incorrect!",
      };
      break;
    case 401:
      event.error = {
        title: "Uh oh, something went wrong",
        message: "You are somehow not logged in... ¯_(ツ)_/¯",
      };
      break;
    default:
      throw new Error("unexpected response");
  }

  commit("passwordChangedEvent", event);
};
