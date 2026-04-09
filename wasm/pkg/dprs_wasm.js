/* @ts-self-types="./dprs_wasm.d.ts" */

export class Dims {
    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(Dims.prototype);
        obj.__wbg_ptr = ptr;
        DimsFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        DimsFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_dims_free(ptr, 0);
    }
    constructor() {
        const ret = wasm.dims_new();
        this.__wbg_ptr = ret >>> 0;
        DimsFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * @returns {number}
     */
    get n_x() {
        const ret = wasm.__wbg_get_dims_n_x(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @returns {number}
     */
    get n_y() {
        const ret = wasm.__wbg_get_dims_n_y(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @returns {number}
     */
    get n_z() {
        const ret = wasm.__wbg_get_dims_n_z(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @param {number} arg0
     */
    set n_x(arg0) {
        wasm.__wbg_set_dims_n_x(this.__wbg_ptr, arg0);
    }
    /**
     * @param {number} arg0
     */
    set n_y(arg0) {
        wasm.__wbg_set_dims_n_y(this.__wbg_ptr, arg0);
    }
    /**
     * @param {number} arg0
     */
    set n_z(arg0) {
        wasm.__wbg_set_dims_n_z(this.__wbg_ptr, arg0);
    }
}
if (Symbol.dispose) Dims.prototype[Symbol.dispose] = Dims.prototype.free;

export class Parameters {
    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(Parameters.prototype);
        obj.__wbg_ptr = ptr;
        ParametersFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        ParametersFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_parameters_free(ptr, 0);
    }
    /**
     * @returns {Dims}
     */
    get dims() {
        const ret = wasm.parameters_dims(this.__wbg_ptr);
        return Dims.__wrap(ret);
    }
    /**
     * Create a new [Parameters]
     */
    constructor() {
        const ret = wasm.parameters_new();
        this.__wbg_ptr = ret >>> 0;
        ParametersFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * @returns {Params}
     */
    get params() {
        const ret = wasm.parameters_params(this.__wbg_ptr);
        return Params.__wrap(ret);
    }
    /**
     * @returns {Probabilities}
     */
    get probabilities() {
        const ret = wasm.parameters_probabilities(this.__wbg_ptr);
        return Probabilities.__wrap(ret);
    }
    /**
     * @param {Dims} value
     */
    set dims(value) {
        _assertClass(value, Dims);
        wasm.parameters_set_dims(this.__wbg_ptr, value.__wbg_ptr);
    }
    /**
     * @param {Params} value
     */
    set params(value) {
        _assertClass(value, Params);
        wasm.parameters_set_params(this.__wbg_ptr, value.__wbg_ptr);
    }
    /**
     * @param {Probabilities} value
     */
    set probabilities(value) {
        _assertClass(value, Probabilities);
        wasm.parameters_set_probabilities(this.__wbg_ptr, value.__wbg_ptr);
    }
    /**
     * @param {TopoBc} value
     */
    set topo_bc_x(value) {
        _assertClass(value, TopoBc);
        wasm.parameters_set_topo_bc_x(this.__wbg_ptr, value.__wbg_ptr);
    }
    /**
     * @param {TopoBc} value
     */
    set topo_bc_y(value) {
        _assertClass(value, TopoBc);
        wasm.parameters_set_topo_bc_y(this.__wbg_ptr, value.__wbg_ptr);
    }
    /**
     * @param {TopoBc} value
     */
    set topo_bc_z(value) {
        _assertClass(value, TopoBc);
        wasm.parameters_set_topo_bc_z(this.__wbg_ptr, value.__wbg_ptr);
    }
    /**
     * @returns {TopoBc}
     */
    get topo_bc_x() {
        const ret = wasm.parameters_topo_bc_x(this.__wbg_ptr);
        return TopoBc.__wrap(ret);
    }
}
if (Symbol.dispose) Parameters.prototype[Symbol.dispose] = Parameters.prototype.free;

export class Params {
    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(Params.prototype);
        obj.__wbg_ptr = ptr;
        ParamsFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        ParamsFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_params_free(ptr, 0);
    }
    /**
     * @returns {number}
     */
    get n_iterations() {
        const ret = wasm.__wbg_get_params_n_iterations(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @returns {number}
     */
    get random_seed() {
        const ret = wasm.__wbg_get_params_random_seed(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @returns {number}
     */
    get sample_period() {
        const ret = wasm.__wbg_get_params_sample_period(this.__wbg_ptr);
        return ret >>> 0;
    }
    constructor() {
        const ret = wasm.params_new();
        this.__wbg_ptr = ret >>> 0;
        ParamsFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * @param {number} arg0
     */
    set n_iterations(arg0) {
        wasm.__wbg_set_params_n_iterations(this.__wbg_ptr, arg0);
    }
    /**
     * @param {number} arg0
     */
    set random_seed(arg0) {
        wasm.__wbg_set_params_random_seed(this.__wbg_ptr, arg0);
    }
    /**
     * @param {number} arg0
     */
    set sample_period(arg0) {
        wasm.__wbg_set_params_sample_period(this.__wbg_ptr, arg0);
    }
}
if (Symbol.dispose) Params.prototype[Symbol.dispose] = Params.prototype.free;

export class Probabilities {
    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(Probabilities.prototype);
        obj.__wbg_ptr = ptr;
        ProbabilitiesFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        ProbabilitiesFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_probabilities_free(ptr, 0);
    }
    /**
     * @returns {number}
     */
    get p_1() {
        const ret = wasm.__wbg_get_probabilities_p_1(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {number}
     */
    get p_2() {
        const ret = wasm.__wbg_get_probabilities_p_2(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {number}
     */
    get p_initial() {
        const ret = wasm.__wbg_get_probabilities_p_initial(this.__wbg_ptr);
        return ret;
    }
    constructor() {
        const ret = wasm.probabilities_new();
        this.__wbg_ptr = ret >>> 0;
        ProbabilitiesFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * @param {number} arg0
     */
    set p_1(arg0) {
        wasm.__wbg_set_probabilities_p_1(this.__wbg_ptr, arg0);
    }
    /**
     * @param {number} arg0
     */
    set p_2(arg0) {
        wasm.__wbg_set_probabilities_p_2(this.__wbg_ptr, arg0);
    }
    /**
     * @param {number} arg0
     */
    set p_initial(arg0) {
        wasm.__wbg_set_probabilities_p_initial(this.__wbg_ptr, arg0);
    }
}
if (Symbol.dispose) Probabilities.prototype[Symbol.dispose] = Probabilities.prototype.free;

export class Simulation {
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        SimulationFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_simulation_free(ptr, 0);
    }
    /**
     * Create a new [Parameters]
     * @param {Parameters} parameters
     */
    constructor(parameters) {
        _assertClass(parameters, Parameters);
        const ret = wasm.simulation_new(parameters.__wbg_ptr);
        this.__wbg_ptr = ret >>> 0;
        SimulationFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * @returns {Parameters}
     */
    get parameters() {
        const ret = wasm.simulation_parameters(this.__wbg_ptr);
        return Parameters.__wrap(ret);
    }
    /**
     * @param {number} index
     * @returns {Uint8Array | undefined}
     */
    result(index) {
        const ret = wasm.simulation_result(this.__wbg_ptr, index);
        let v1;
        if (ret[0] !== 0) {
            v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
            wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
        }
        return v1;
    }
    /**
     * @param {Parameters} parameters
     */
    set parameters(parameters) {
        _assertClass(parameters, Parameters);
        wasm.simulation_set_parameters(this.__wbg_ptr, parameters.__wbg_ptr);
    }
    simulate() {
        wasm.simulation_simulate(this.__wbg_ptr);
    }
}
if (Symbol.dispose) Simulation.prototype[Symbol.dispose] = Simulation.prototype.free;

export class TopoBc {
    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(TopoBc.prototype);
        obj.__wbg_ptr = ptr;
        TopoBcFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        TopoBcFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_topobc_free(ptr, 0);
    }
    /**
     * @returns {boolean}
     */
    get fix_max() {
        const ret = wasm.__wbg_get_topobc_fix_max(this.__wbg_ptr);
        return ret !== 0;
    }
    /**
     * @returns {boolean}
     */
    get fix_min() {
        const ret = wasm.__wbg_get_topobc_fix_min(this.__wbg_ptr);
        return ret !== 0;
    }
    /**
     * @returns {boolean}
     */
    get fix_value() {
        const ret = wasm.__wbg_get_topobc_fix_value(this.__wbg_ptr);
        return ret !== 0;
    }
    /**
     * @returns {boolean}
     */
    get periodic() {
        const ret = wasm.__wbg_get_topobc_periodic(this.__wbg_ptr);
        return ret !== 0;
    }
    /**
     * @param {boolean} arg0
     */
    set fix_max(arg0) {
        wasm.__wbg_set_topobc_fix_max(this.__wbg_ptr, arg0);
    }
    /**
     * @param {boolean} arg0
     */
    set fix_min(arg0) {
        wasm.__wbg_set_topobc_fix_min(this.__wbg_ptr, arg0);
    }
    /**
     * @param {boolean} arg0
     */
    set fix_value(arg0) {
        wasm.__wbg_set_topobc_fix_value(this.__wbg_ptr, arg0);
    }
    /**
     * @param {boolean} arg0
     */
    set periodic(arg0) {
        wasm.__wbg_set_topobc_periodic(this.__wbg_ptr, arg0);
    }
    constructor() {
        const ret = wasm.topobc_new();
        this.__wbg_ptr = ret >>> 0;
        TopoBcFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
}
if (Symbol.dispose) TopoBc.prototype[Symbol.dispose] = TopoBc.prototype.free;

function __wbg_get_imports() {
    const import0 = {
        __proto__: null,
        __wbg___wbindgen_throw_81fc77679af83bc6: function(arg0, arg1) {
            throw new Error(getStringFromWasm0(arg0, arg1));
        },
        __wbindgen_init_externref_table: function() {
            const table = wasm.__wbindgen_externrefs;
            const offset = table.grow(4);
            table.set(0, undefined);
            table.set(offset + 0, undefined);
            table.set(offset + 1, null);
            table.set(offset + 2, true);
            table.set(offset + 3, false);
        },
    };
    return {
        __proto__: null,
        "./dprs_wasm_bg.js": import0,
    };
}

const DimsFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_dims_free(ptr >>> 0, 1));
const ParametersFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_parameters_free(ptr >>> 0, 1));
const ParamsFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_params_free(ptr >>> 0, 1));
const ProbabilitiesFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_probabilities_free(ptr >>> 0, 1));
const SimulationFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_simulation_free(ptr >>> 0, 1));
const TopoBcFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_topobc_free(ptr >>> 0, 1));

function _assertClass(instance, klass) {
    if (!(instance instanceof klass)) {
        throw new Error(`expected instance of ${klass.name}`);
    }
}

function getArrayU8FromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return getUint8ArrayMemory0().subarray(ptr / 1, ptr / 1 + len);
}

function getStringFromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return decodeText(ptr, len);
}

let cachedUint8ArrayMemory0 = null;
function getUint8ArrayMemory0() {
    if (cachedUint8ArrayMemory0 === null || cachedUint8ArrayMemory0.byteLength === 0) {
        cachedUint8ArrayMemory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8ArrayMemory0;
}

let cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });
cachedTextDecoder.decode();
const MAX_SAFARI_DECODE_BYTES = 2146435072;
let numBytesDecoded = 0;
function decodeText(ptr, len) {
    numBytesDecoded += len;
    if (numBytesDecoded >= MAX_SAFARI_DECODE_BYTES) {
        cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });
        cachedTextDecoder.decode();
        numBytesDecoded = len;
    }
    return cachedTextDecoder.decode(getUint8ArrayMemory0().subarray(ptr, ptr + len));
}

let wasmModule, wasm;
function __wbg_finalize_init(instance, module) {
    wasm = instance.exports;
    wasmModule = module;
    cachedUint8ArrayMemory0 = null;
    wasm.__wbindgen_start();
    return wasm;
}

async function __wbg_load(module, imports) {
    if (typeof Response === 'function' && module instanceof Response) {
        if (typeof WebAssembly.instantiateStreaming === 'function') {
            try {
                return await WebAssembly.instantiateStreaming(module, imports);
            } catch (e) {
                const validResponse = module.ok && expectedResponseType(module.type);

                if (validResponse && module.headers.get('Content-Type') !== 'application/wasm') {
                    console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve Wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);

                } else { throw e; }
            }
        }

        const bytes = await module.arrayBuffer();
        return await WebAssembly.instantiate(bytes, imports);
    } else {
        const instance = await WebAssembly.instantiate(module, imports);

        if (instance instanceof WebAssembly.Instance) {
            return { instance, module };
        } else {
            return instance;
        }
    }

    function expectedResponseType(type) {
        switch (type) {
            case 'basic': case 'cors': case 'default': return true;
        }
        return false;
    }
}

function initSync(module) {
    if (wasm !== undefined) return wasm;


    if (module !== undefined) {
        if (Object.getPrototypeOf(module) === Object.prototype) {
            ({module} = module)
        } else {
            console.warn('using deprecated parameters for `initSync()`; pass a single object instead')
        }
    }

    const imports = __wbg_get_imports();
    if (!(module instanceof WebAssembly.Module)) {
        module = new WebAssembly.Module(module);
    }
    const instance = new WebAssembly.Instance(module, imports);
    return __wbg_finalize_init(instance, module);
}

async function __wbg_init(module_or_path) {
    if (wasm !== undefined) return wasm;


    if (module_or_path !== undefined) {
        if (Object.getPrototypeOf(module_or_path) === Object.prototype) {
            ({module_or_path} = module_or_path)
        } else {
            console.warn('using deprecated parameters for the initialization function; pass a single object instead')
        }
    }

    if (module_or_path === undefined) {
        module_or_path = new URL('dprs_wasm_bg.wasm', import.meta.url);
    }
    const imports = __wbg_get_imports();

    if (typeof module_or_path === 'string' || (typeof Request === 'function' && module_or_path instanceof Request) || (typeof URL === 'function' && module_or_path instanceof URL)) {
        module_or_path = fetch(module_or_path);
    }

    const { instance, module } = await __wbg_load(await module_or_path, imports);

    return __wbg_finalize_init(instance, module);
}

export { initSync, __wbg_init as default };
