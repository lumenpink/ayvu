/* tslint:disable */
/* eslint-disable */

export function get_backdrop_svg(name: string, width: number, height: number): string | undefined;

export function get_character_avatar_svg(id: string): string | undefined;

export function get_character_detail_json(id: string): string | undefined;

export function get_character_pose_svg(id: string, emotion: string): string | undefined;

export function list_backdrops_json(): string;

export function list_characters_json(): string;

export function render_comic(script: string, title?: string | null, layout?: string | null): string;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
    readonly memory: WebAssembly.Memory;
    readonly get_backdrop_svg: (a: number, b: number, c: number, d: number, e: number) => void;
    readonly get_character_avatar_svg: (a: number, b: number, c: number) => void;
    readonly get_character_detail_json: (a: number, b: number, c: number) => void;
    readonly get_character_pose_svg: (a: number, b: number, c: number, d: number, e: number) => void;
    readonly list_backdrops_json: (a: number) => void;
    readonly list_characters_json: (a: number) => void;
    readonly render_comic: (a: number, b: number, c: number, d: number, e: number, f: number, g: number) => void;
    readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
    readonly __wbindgen_export: (a: number, b: number) => number;
    readonly __wbindgen_export2: (a: number, b: number, c: number, d: number) => number;
    readonly __wbindgen_export3: (a: number, b: number, c: number) => void;
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
