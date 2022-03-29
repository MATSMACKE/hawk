/* tslint:disable */
/* eslint-disable */
/**
*/
export class Runner {
  free(): void;
/**
* @returns {Runner}
*/
  static new(): Runner;
/**
*/
  clear(): void;
/**
* @param {string} code
* @param {boolean} in_repl
*/
  run(code: string, in_repl: boolean): void;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_runner_free: (a: number) => void;
  readonly runner_new: () => number;
  readonly runner_clear: (a: number) => void;
  readonly runner_run: (a: number, b: number, c: number, d: number) => void;
  readonly __wbindgen_malloc: (a: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number) => number;
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
