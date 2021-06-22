import {
  baseAlertPropsType,
  BaseAlertPropsType,
} from "../../base/BaseAlert.types";

export interface PureChangePasswordPanelProps {
  alertEventDate: Date;
  alertMessage: string;
  alertTitle: string;
  alertType: PureChangePasswordPanelPropsType;
}

export const pureChangePasswordPanelPropsTypes = baseAlertPropsType;
export type PureChangePasswordPanelPropsType = BaseAlertPropsType;

export const pureChangePasswordPanelEvents = ["submitted"] as const;
export type PureChangePasswordPanelEvents =
  typeof pureChangePasswordPanelEvents[number];

export interface PureChangePasswordPanelEventSubmit {
  currentPassword: string;
  newPassword: string;
  confirmNewPassword: string;
}
