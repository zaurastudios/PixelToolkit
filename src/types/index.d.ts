export type AddAdditionalType<T, U> = {
  [K in keyof T]: T[K] | U;
};
