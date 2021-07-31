export interface PureSubForumCardProps {
  name: string;
}

export const pureSubForumCardEvents = ["clicked"] as const;
export type PureSubForumCardEvents = typeof pureSubForumCardEvents[number];
