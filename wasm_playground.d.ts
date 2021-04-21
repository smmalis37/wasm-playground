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
