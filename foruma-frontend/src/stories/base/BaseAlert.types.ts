export interface BaseAlertProps {
  eventDate: Date;
  message: string;
  title: string;
  type: BaseAlertPropsType;
}

export const baseAlertPropsType = [
  "information",
  "success",
  "warning",
] as const;
export type BaseAlertPropsType = typeof baseAlertPropsType[number];

export const baseAlertEvents = ["open", "close"] as const;
export type BaseAlertEvents = typeof baseAlertEvents[number];
