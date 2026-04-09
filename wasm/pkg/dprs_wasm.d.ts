/* tslint:disable */
/* eslint-disable */

export class Dims {
    free(): void;
    [Symbol.dispose](): void;
    constructor();
    n_x: number;
    n_y: number;
    n_z: number;
}

export class Parameters {
    free(): void;
    [Symbol.dispose](): void;
    /**
     * Create a new [Parameters]
     */
    constructor();
    dims: Dims;
    params: Params;
    probabilities: Probabilities;
    topo_bc_x: TopoBc;
    set topo_bc_y(value: TopoBc);
    set topo_bc_z(value: TopoBc);
}

export class Params {
    free(): void;
    [Symbol.dispose](): void;
    constructor();
    n_iterations: number;
    random_seed: number;
    sample_period: number;
}

export class Probabilities {
    free(): void;
    [Symbol.dispose](): void;
    constructor();
    p_1: number;
    p_2: number;
    p_initial: number;
}

export class Simulation {
    free(): void;
    [Symbol.dispose](): void;
    /**
     * Create a new [Parameters]
     */
    constructor(parameters: Parameters);
    result(index: number): Uint8Array | undefined;
    simulate(): void;
    parameters: Parameters;
}

export class TopoBc {
    free(): void;
    [Symbol.dispose](): void;
    constructor();
    fix_max: boolean;
    fix_min: boolean;
    fix_value: boolean;
    periodic: boolean;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
    readonly memory: WebAssembly.Memory;
    readonly __wbg_dims_free: (a: number, b: number) => void;
    readonly __wbg_get_dims_n_x: (a: number) => number;
    readonly __wbg_get_dims_n_y: (a: number) => number;
    readonly __wbg_get_dims_n_z: (a: number) => number;
    readonly __wbg_get_probabilities_p_1: (a: number) => number;
    readonly __wbg_get_probabilities_p_2: (a: number) => number;
    readonly __wbg_get_probabilities_p_initial: (a: number) => number;
    readonly __wbg_get_topobc_fix_max: (a: number) => number;
    readonly __wbg_get_topobc_fix_min: (a: number) => number;
    readonly __wbg_get_topobc_fix_value: (a: number) => number;
    readonly __wbg_get_topobc_periodic: (a: number) => number;
    readonly __wbg_parameters_free: (a: number, b: number) => void;
    readonly __wbg_params_free: (a: number, b: number) => void;
    readonly __wbg_probabilities_free: (a: number, b: number) => void;
    readonly __wbg_set_dims_n_x: (a: number, b: number) => void;
    readonly __wbg_set_dims_n_y: (a: number, b: number) => void;
    readonly __wbg_set_dims_n_z: (a: number, b: number) => void;
    readonly __wbg_set_probabilities_p_1: (a: number, b: number) => void;
    readonly __wbg_set_probabilities_p_2: (a: number, b: number) => void;
    readonly __wbg_set_probabilities_p_initial: (a: number, b: number) => void;
    readonly __wbg_set_topobc_fix_max: (a: number, b: number) => void;
    readonly __wbg_set_topobc_fix_min: (a: number, b: number) => void;
    readonly __wbg_set_topobc_fix_value: (a: number, b: number) => void;
    readonly __wbg_set_topobc_periodic: (a: number, b: number) => void;
    readonly __wbg_simulation_free: (a: number, b: number) => void;
    readonly __wbg_topobc_free: (a: number, b: number) => void;
    readonly dims_new: () => number;
    readonly parameters_dims: (a: number) => number;
    readonly parameters_new: () => number;
    readonly parameters_params: (a: number) => number;
    readonly parameters_probabilities: (a: number) => number;
    readonly parameters_set_dims: (a: number, b: number) => void;
    readonly parameters_set_params: (a: number, b: number) => void;
    readonly parameters_set_probabilities: (a: number, b: number) => void;
    readonly parameters_set_topo_bc_x: (a: number, b: number) => void;
    readonly parameters_set_topo_bc_y: (a: number, b: number) => void;
    readonly parameters_set_topo_bc_z: (a: number, b: number) => void;
    readonly parameters_topo_bc_x: (a: number) => number;
    readonly probabilities_new: () => number;
    readonly simulation_new: (a: number) => number;
    readonly simulation_parameters: (a: number) => number;
    readonly simulation_result: (a: number, b: number) => [number, number];
    readonly simulation_set_parameters: (a: number, b: number) => void;
    readonly simulation_simulate: (a: number) => void;
    readonly topobc_new: () => number;
    readonly __wbg_get_params_n_iterations: (a: number) => number;
    readonly __wbg_get_params_random_seed: (a: number) => number;
    readonly __wbg_get_params_sample_period: (a: number) => number;
    readonly __wbg_set_params_n_iterations: (a: number, b: number) => void;
    readonly __wbg_set_params_random_seed: (a: number, b: number) => void;
    readonly __wbg_set_params_sample_period: (a: number, b: number) => void;
    readonly params_new: () => number;
    readonly __wbindgen_externrefs: WebAssembly.Table;
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
