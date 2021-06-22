export interface PureLoginFormProps {
  username: string;
  alertEventDate: Date;
  alertMessage: string;
  alertTitle: string;
}

export const pureLoginFormEvents = ["submitted"] as const;
export type PureLoginFormEvents = typeof pureLoginFormEvents[number];

export interface PureLoginFormEventSubmit {
  username: string;
  password: string;
}
