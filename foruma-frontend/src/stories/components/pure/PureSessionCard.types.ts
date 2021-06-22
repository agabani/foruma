export interface PureSessionCardProps {
  browser: PureSessionPropsBrowser;
  operatingSystem: PureSessionPropsOperatingSystem;
  lastActiveDate: Date;
}

export const pureSessionPropsBrowser = [
  null,
  "Chrome",
  "Edge",
  "Firefox",
  "Safari",
] as const;
export type PureSessionPropsBrowser = typeof pureSessionPropsBrowser[number];

export const pureSessionPropsOperatingSystem = [
  null,
  "Linux",
  "Mac OS",
  "Windows",
] as const;
export type PureSessionPropsOperatingSystem =
  typeof pureSessionPropsOperatingSystem[number];

export const pureSessionCardEvents = ["deleteClicked"] as const;
export type PureSessionCardEvents = typeof pureSessionCardEvents[number];
