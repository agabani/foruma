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

export const initialize = async ({
  commit,
}: {
  commit: Commit;
}): Promise<void> => {
  const whoamiResponse = await api.get("/api/authentication/whoami");

  switch (whoamiResponse.status) {
    case 200:
      {
        commit("login", whoamiResponse.data.username);
      }
      break;
    case 401:
      {
        commit("logout");
      }
      return;
    default:
      throw new Error("unexpected response");
  }
};

export const login = async (
  { commit }: { commit: Commit },
  payload: LoginPayload
): Promise<void> => {
  const loginResponse = await api.post("/api/authentication/login", {
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

  const whoamiResponse = await api.get("/api/authentication/whoami");

  switch (whoamiResponse.status) {
    case 200:
      {
        commit("login", whoamiResponse.data.username);
      }
      break;
    default:
      throw new Error("unexpected response");
  }
};

export const logout = async ({ commit }: { commit: Commit }): Promise<void> => {
  const logoutResponse = await api.post("/api/authentication/logout");

  switch (logoutResponse.status) {
    case 200:
      {
        commit("logout");
      }
      break;
    default:
      throw new Error("unexpected response");
  }
};

export const signup = async (
  { commit }: { commit: Commit },
  payload: SignupPayload
): Promise<void> => {
  const signupResponse = await api.post("/api/authentication/signup", {
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

  const whoamiResponse = await api.get("/api/authentication/whoami");

  switch (whoamiResponse.status) {
    case 200:
      {
        commit("login", whoamiResponse.data.username);
      }
      break;
    default:
      throw new Error("unexpected response");
  }
};

export const terminateOwnAccount = async ({
  commit,
}: {
  commit: Commit;
}): Promise<void> => {
  const terminateResponse = await api.post("/api/account/terminate");

  switch (terminateResponse.status) {
    case 200:
      {
        commit("logout", terminateResponse.data.username);
      }
      break;
    default:
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
      {
        commit("passwordChangedEvent", event);
      }
      break;
    case 400:
      event.error = {
        title: "Uh oh, something went wrong",
        message: "Your current password is incorrect!",
      };
      commit("passwordChangedEvent", event);
      break;
    case 401:
      event.error = {
        title: "Uh oh, something went wrong",
        message: "You are somehow not logged in... ¯\\_(ツ)_/¯",
      };
      commit("passwordChangedEvent", event);
      break;
    default:
      throw new Error("unexpected response");
  }
};
