/* tslint:disable */
/* eslint-disable */
/**
*/
export enum ColorMode {
  Red,
  YellowGreen,
  BlueGreen,
  Blue,
  Purple,
  Pink,
}
/**
*/
export class Fire {
  free(): void;
/**
* @param {number} width
* @param {number} height
* @returns {Fire}
*/
  static new(width: number, height: number): Fire;
/**
* @param {number} spread_factor
* @param {number} height_factor
* @param {number} color
*/
  tick(spread_factor: number, height_factor: number, color: number): void;
/**
* @returns {number}
*/
  texture(): number;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_fire_free: (a: number) => void;
  readonly fire_new: (a: number, b: number) => number;
  readonly fire_tick: (a: number, b: number, c: number, d: number) => void;
  readonly fire_texture: (a: number) => number;
  readonly __wbindgen_exn_store: (a: number) => void;
}

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
