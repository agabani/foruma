import axios from "axios";

const api = axios.create({
  baseURL: process.env.VUE_APP_API_BASE_URL,
  validateStatus: null,
  withCredentials: true,
});

const GraphQL = "/api/graphql/";

export async function queryCurrentAccount(): Promise<{
  id: string;
  username: string;
} | null> {
  const response = await api.post(GraphQL, {
    query: `
{
    currentAccount {
        id
        username
    }
}`,
  });

  if (response.status !== 200) {
    throw new Error("Unexpected status code");
  }

  return response.data.data.currentAccount
    ? response.data.data.currentAccount
    : null;
}

export async function queryCurrentAccountAuthenticationSessions(): Promise<
  | [
      {
        id: string;
        isCurrentSession: boolean;
        browser: string | null;
        operatingSystem: string | null;
        location: string | null;
        lastActiveDate: string;
      }
    ]
  | null
> {
  const response = await api.post(GraphQL, {
    query: `
{
  currentAccount {
    id
    username
    authentication {
      sessions {
        browser
        id
        isCurrentSession
        lastActiveDate
        location
        operatingSystem
      }
    }
  }
}
`,
  });

  if (response.status !== 200) {
    throw new Error("Unexpected status code");
  }

  return response.data.data.currentAccount
    ? response.data.data.currentAccount.authentication.sessions
    : null;
}
