import { PureSubForumCardProps } from "./PureSubForumCard.types";

export interface PureSubForumPanelProps {
  subForums: PureSubForumPanelPropsSubForum[];
}

export type PureSubForumPanelPropsSubForum = {
  id: string;
} & PureSubForumCardProps;

export const pureSubForumPanelEvents = ["clicked"] as const;
export type PureSubForumPanelEvents = typeof pureSubForumPanelEvents[number];

export interface PureSubForumPanelEventClicked {
  id: string;
}
