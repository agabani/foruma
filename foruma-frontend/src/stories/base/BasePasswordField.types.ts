export interface BasePasswordFieldProps {
  fullWidth: boolean;
  placeholder: string;
  size: BasePasswordFieldPropsSize;
  value: string;
}

export const basePasswordFieldPropsSizes = [
  "small",
  "medium",
  "large",
] as const;
export type BasePasswordFieldPropsSize =
  typeof basePasswordFieldPropsSizes[number];

export const basePasswordFieldEvents = ["change"] as const;
export type BasePasswordFieldEvents = typeof basePasswordFieldEvents[number];

export interface BasePasswordFieldEventChange {
  newValue: string;
  oldValue: string;
}
