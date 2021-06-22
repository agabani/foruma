export interface BaseIconProps {
  height: number | string;
  width: number | string;
  iconName: BaseIconPropsIconName;
}

export const baseIconPropsIconName = [
  "placeholder",
  "checkmark",
  "chrome",
  "info",
  "edge",
  "firefox",
  "safari",
  "trash",
  "warning",
] as const;
export type BaseIconPropsIconName = typeof baseIconPropsIconName[number];
