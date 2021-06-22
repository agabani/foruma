import { PureSessionCardProps } from "./PureSessionCard.types";

export interface PureSessionsPanelProps {
  sessions: ({ id: string } & PureSessionCardProps)[];
}

export const pureSessionsPanelEvents = ["deleteClicked"] as const;
export type PureSessionsPanelEvents = typeof pureSessionsPanelEvents[number];

export interface PureSessionsPanelEventDeleteClicked {
  id: string;
}
