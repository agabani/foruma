export interface BaseButtonProps {
  fullWidth: boolean;
  label: string;
  primary: boolean;
  size: BaseButtonPropsSize;
  type: BaseButtonPropsType;
}

export const baseButtonPropSizes = ["small", "medium", "large"] as const;
export type BaseButtonPropsSize = typeof baseButtonPropSizes[number];

export const baseButtonPropsType = ["button", "reset", "submit"] as const;
export type BaseButtonPropsType = typeof baseButtonPropsType[number];

export const baseButtonEvents = ["click"] as const;
export type BaseButtonEvents = typeof baseButtonEvents[number];
