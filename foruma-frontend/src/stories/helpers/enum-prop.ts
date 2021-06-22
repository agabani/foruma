import { PropType } from "vue";

export default function enumProp<D extends T, T extends string | number>(
  d: D,
  e: readonly T[]
): { default: T; validator: (v: T) => boolean; type: PropType<T> } {
  return {
    default: d,
    validator: (v: T) => e.includes(v),
    type:
      typeof d === "string" || d instanceof String
        ? (String as unknown as PropType<T>)
        : (Number as unknown as PropType<T>),
  };
}
