/* tslint:disable */
/* eslint-disable */

export class Parameters {
    free(): void;
    [Symbol.dispose](): void;
    /**
     * Create a new [Parameters]
     */
    constructor();
    sim_dimension(): number;
    initial_condition: string;
    n_iterations: number;
    n_x: number;
    n_y: number;
    n_z: number;
    p_1: number;
    p_2: number;
    p_conj: number;
    p_diag: number;
    p_initial: number;
    p_nbr: number;
    random_seed: number;
    sample_period: number;
    topology_bc_x: TopologyBc;
    set topology_bc_y(value: TopologyBc);
    set topology_bc_z(value: TopologyBc);
    u_x: number;
}

export class Simulation {
    free(): void;
    [Symbol.dispose](): void;
    /**
     * Create a new [Parameters]
     */
    constructor(parameters: Parameters);
    result(index: number): Uint8Array | undefined;
    result_sum_kernel_with_threshold(index: number, kernel_size: number, threshold: number, step: number): Uint8Array | undefined;
    simulate(model: string, scheme: string): void;
    parameters: Parameters;
}

export class TopologyBc {
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
    readonly __wbg_simulation_free: (a: number, b: number) => void;
    readonly simulation_new: (a: number) => number;
    readonly simulation_parameters: (a: number) => number;
    readonly simulation_result: (a: number, b: number) => [number, number];
    readonly simulation_result_sum_kernel_with_threshold: (a: number, b: number, c: number, d: number, e: number) => [number, number];
    readonly simulation_set_parameters: (a: number, b: number) => void;
    readonly simulation_simulate: (a: number, b: number, c: number, d: number, e: number) => [number, number];
    readonly __wbg_get_topologybc_fix_max: (a: number) => number;
    readonly __wbg_get_topologybc_fix_min: (a: number) => number;
    readonly __wbg_get_topologybc_fix_value: (a: number) => number;
    readonly __wbg_get_topologybc_periodic: (a: number) => number;
    readonly __wbg_set_topologybc_fix_max: (a: number, b: number) => void;
    readonly __wbg_set_topologybc_fix_min: (a: number, b: number) => void;
    readonly __wbg_set_topologybc_fix_value: (a: number, b: number) => void;
    readonly __wbg_set_topologybc_periodic: (a: number, b: number) => void;
    readonly __wbg_topologybc_free: (a: number, b: number) => void;
    readonly topologybc_new: () => number;
    readonly __wbg_parameters_free: (a: number, b: number) => void;
    readonly parameters_initial_condition: (a: number) => [number, number];
    readonly parameters_n_iterations: (a: number) => number;
    readonly parameters_n_x: (a: number) => number;
    readonly parameters_n_y: (a: number) => number;
    readonly parameters_n_z: (a: number) => number;
    readonly parameters_new: () => number;
    readonly parameters_p_1: (a: number) => number;
    readonly parameters_p_2: (a: number) => number;
    readonly parameters_p_conj: (a: number) => number;
    readonly parameters_p_diag: (a: number) => number;
    readonly parameters_p_initial: (a: number) => number;
    readonly parameters_p_nbr: (a: number) => number;
    readonly parameters_random_seed: (a: number) => number;
    readonly parameters_sample_period: (a: number) => number;
    readonly parameters_set_initial_condition: (a: number, b: number, c: number) => void;
    readonly parameters_set_n_iterations: (a: number, b: number) => void;
    readonly parameters_set_n_x: (a: number, b: number) => void;
    readonly parameters_set_n_y: (a: number, b: number) => void;
    readonly parameters_set_n_z: (a: number, b: number) => void;
    readonly parameters_set_p_1: (a: number, b: number) => void;
    readonly parameters_set_p_2: (a: number, b: number) => void;
    readonly parameters_set_p_conj: (a: number, b: number) => void;
    readonly parameters_set_p_diag: (a: number, b: number) => void;
    readonly parameters_set_p_initial: (a: number, b: number) => void;
    readonly parameters_set_p_nbr: (a: number, b: number) => void;
    readonly parameters_set_random_seed: (a: number, b: number) => void;
    readonly parameters_set_sample_period: (a: number, b: number) => void;
    readonly parameters_set_topology_bc_x: (a: number, b: number) => void;
    readonly parameters_set_topology_bc_y: (a: number, b: number) => void;
    readonly parameters_set_topology_bc_z: (a: number, b: number) => void;
    readonly parameters_set_u_x: (a: number, b: number) => void;
    readonly parameters_sim_dimension: (a: number) => number;
    readonly parameters_topology_bc_x: (a: number) => number;
    readonly parameters_u_x: (a: number) => number;
    readonly __wbindgen_externrefs: WebAssembly.Table;
    readonly __wbindgen_free: (a: number, b: number, c: number) => void;
    readonly __wbindgen_malloc: (a: number, b: number) => number;
    readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
    readonly __externref_table_dealloc: (a: number) => void;
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
