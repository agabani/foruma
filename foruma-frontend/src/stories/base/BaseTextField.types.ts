export interface BaseTextFieldProps {
  fullWidth: boolean;
  placeholder: string;
  size: BaseTextFieldPropsSize;
  value: string;
}

export const baseTextFieldPropsSizes = ["small", "medium", "large"] as const;
export type BaseTextFieldPropsSize = typeof baseTextFieldPropsSizes[number];

export const baseTextFieldEvents = ["change"] as const;
export type BaseTextFieldEvents = typeof baseTextFieldEvents[number];

export interface BaseTextFieldEventChange {
  newValue: string;
  oldValue: string;
}
