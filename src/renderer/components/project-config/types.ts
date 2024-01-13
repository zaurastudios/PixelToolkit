type Value = number;
type Shift = number;
type Scale = number;

interface Defaults {
  value: Value;
  shift: Shift;
  scale: Scale;
}

export interface General {}

export interface Height {
  shift: Shift;
  scale: Scale;
}

export interface Occlusion extends Defaults {
  height: {
    stepLength: number;
    zBias: number;
    zScale: number;
  };
}

export interface Smoothness extends Defaults {}
export interface Roughness extends Defaults {}
export interface Porosity extends Defaults {}
export interface SSS extends Defaults {}
export interface Emissive extends Defaults {}
