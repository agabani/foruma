export interface PureSignupFormProps {
  username: string;
  alertEventDate: Date;
  alertMessage: string;
  alertTitle: string;
}

export const pureSignupFormEvents = ["submitted"] as const;
export type PureSignupFormEvents = typeof pureSignupFormEvents[number];

export interface PureSignupFormEventSubmit {
  username: string;
  password: string;
  confirmPassword: string;
}
