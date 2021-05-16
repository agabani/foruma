import axios from "axios";
import { Commit } from "vuex";
import type { AuthenticatePayload, SignupPayload } from "./types";

const api = axios.create({
  baseURL: process.env.VUE_APP_API_BASE_URL,
  validateStatus: null,
  withCredentials: true,
});

export const authenticate = async (
  { commit }: { commit: Commit },
  payload: AuthenticatePayload
): Promise<void> => {
  const loginResponse = await api.post("/api/authentication/login", {
    username: payload.username,
    password: payload.password,
  });

  if (loginResponse.status === 401) {
    return;
  } else if (loginResponse.status !== 200) {
    throw new Error("unexpected response");
  }

  const whoamiResponse = await api.get("/api/authentication/whoami");

  if (whoamiResponse.status === 200) {
    commit("authenticate", whoamiResponse.data.username);
  } else {
    throw new Error("unexpected response");
  }
};

export const unauthenticate = async ({
  commit,
}: {
  commit: Commit;
}): Promise<void> => {
  const logoutResponse = await api.post("/api/authentication/logout");

  if (logoutResponse.status !== 200) {
    throw new Error("unexpected response");
  }

  commit("unauthenticate");
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
    commit("authenticate", signupResponse.data.username);
  } else {
    throw new Error("unexpected response");
  }

  const whoamiResponse = await api.get("/api/authentication/whoami");

  if (whoamiResponse.status === 200) {
    commit("authenticate", whoamiResponse.data.username);
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
    commit("authenticate", whoamiResponse.data.username);
  } else if (whoamiResponse.status === 401) {
    commit("unauthenticate");
  } else {
    throw new Error("unexpected response");
  }
};
