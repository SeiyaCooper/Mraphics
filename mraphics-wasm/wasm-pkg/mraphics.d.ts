/* tslint:disable */
/* eslint-disable */

export class Canvas {
  free(): void;
  [Symbol.dispose](): void;
  enable_orbit_control(): void;
  constructor(canvas_id: string);
  run(): void;
  playhead: number;
}

export class Color {
  free(): void;
  [Symbol.dispose](): void;
  static from_hex_str(hex_str: string): Color;
  constructor(r: number, g: number, b: number, a: number);
}

export function set_up(): void;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_color_free: (a: number, b: number) => void;
  readonly color_from_hex_str: (a: number, b: number) => number;
  readonly color_new: (a: number, b: number, c: number, d: number) => number;
  readonly __wbg_canvas_free: (a: number, b: number) => void;
  readonly __wbg_get_canvas_playhead: (a: number) => number;
  readonly __wbg_set_canvas_playhead: (a: number, b: number) => void;
  readonly canvas_enable_orbit_control: (a: number) => void;
  readonly canvas_new: (a: number, b: number) => number;
  readonly canvas_run: (a: number) => void;
  readonly set_up: () => void;
  readonly wasm_bindgen__convert__closures_____invoke__hdb80f1ed7129c4f1: (a: number, b: number) => void;
  readonly wasm_bindgen__closure__destroy__h6c14e0bbaa33f71e: (a: number, b: number) => void;
  readonly wasm_bindgen__convert__closures_____invoke__h5997bd2378143005: (a: number, b: number, c: any) => void;
  readonly wasm_bindgen__closure__destroy__h7530440220bcb8bf: (a: number, b: number) => void;
  readonly wasm_bindgen__convert__closures_____invoke__h7b85597c6c141fa5: (a: number, b: number, c: any, d: any) => void;
  readonly wasm_bindgen__closure__destroy__h4bc839e0a199cb13: (a: number, b: number) => void;
  readonly wasm_bindgen__convert__closures_____invoke__hd7f2cee65b407601: (a: number, b: number, c: any) => void;
  readonly wasm_bindgen__closure__destroy__ha031fb354715f687: (a: number, b: number) => void;
  readonly wasm_bindgen__convert__closures_____invoke__h72e1e188793d6dc6: (a: number, b: number, c: any) => void;
  readonly wasm_bindgen__closure__destroy__h8b8db0507dfe8f07: (a: number, b: number) => void;
  readonly wasm_bindgen__convert__closures_____invoke__h9233806efcfd5643: (a: number, b: number, c: any) => void;
  readonly wasm_bindgen__closure__destroy__hba469e99f58e502a: (a: number, b: number) => void;
  readonly wasm_bindgen__convert__closures_____invoke__hfce13e71b68b9f78: (a: number, b: number, c: any) => void;
  readonly wasm_bindgen__closure__destroy__h5ba5449a24dc04df: (a: number, b: number) => void;
  readonly wasm_bindgen__convert__closures_____invoke__hda95989a6850e8ae: (a: number, b: number, c: any) => void;
  readonly wasm_bindgen__closure__destroy__h527b93f4e1d2d1da: (a: number, b: number) => void;
  readonly wasm_bindgen__convert__closures_____invoke__hb5c2a4e092461377: (a: number, b: number, c: any) => void;
  readonly wasm_bindgen__closure__destroy__h81b0014b7004c6d6: (a: number, b: number) => void;
  readonly wasm_bindgen__convert__closures_____invoke__h952098ab27c9525c: (a: number, b: number, c: any) => void;
  readonly wasm_bindgen__closure__destroy__hb99f7865e774358d: (a: number, b: number) => void;
  readonly wasm_bindgen__convert__closures_____invoke__h5bdae1983e88d88e: (a: number, b: number, c: any) => void;
  readonly wasm_bindgen__closure__destroy__h3786c7b017e19f4d: (a: number, b: number) => void;
  readonly wasm_bindgen__convert__closures_____invoke__hd9dfa106a4aaa8b7: (a: number, b: number) => number;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __externref_table_alloc: () => number;
  readonly __wbindgen_externrefs: WebAssembly.Table;
  readonly __wbindgen_exn_store: (a: number) => void;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
  readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;

/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
*
* @returns {InitOutput}
*/
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
