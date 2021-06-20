export interface PureHeaderProps {
  username: string | undefined;
}

export const pureHeaderEvents = [
  "brandClicked",
  "loginClicked",
  "logoutClicked",
  "signupClicked",
  "usernameClicked",
] as const;
export type PureHeaderEvents = typeof pureHeaderEvents[number];
