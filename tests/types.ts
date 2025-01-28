export type FlattenMethods<T> = {
  [K in keyof T]: T[K] extends (...args: any[]) => infer R ? (R extends object ? FlattenMethods<R> : R) : T[K];
};
