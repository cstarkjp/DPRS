/**
 * This contains JsParameters
 */
import * as DprsWasm from "../pkg/dprs_wasm.js";
class Probabilities {
    constructor() {
        /**
         * This contains JsParameters
         */
        this.p_initial = 0.5;
        this.p_1 = 0.705485152;
        this.p_2 = 0.705485152;
    }
    as_parameters() {
        const probabilities = new DprsWasm.Probabilities();
        probabilities.p_initial = this.p_initial;
        probabilities.p_1 = this.p_1;
        probabilities.p_2 = this.p_2;
        return probabilities;
    }
    from_json(probabilities) {
        const p_initial = probabilities["p_initial"];
        const p_1 = probabilities["p_1"];
        const p_2 = probabilities["p_z"];
        if (typeof p_initial == "number") {
            this.p_initial = p_initial;
        }
        if (typeof p_1 == "number") {
            this.p_1 = p_1;
        }
        if (typeof p_2 == "number") {
            this.p_2 = p_2;
        }
    }
}
class Params {
    constructor() {
        this.n_iterations = 600;
        this.sample_period = 1;
        this.random_seed = 1;
        this.initial_center = true;
        this.simulation_kind = "staggered_dk";
    }
    as_parameters() {
        const params = new DprsWasm.Params();
        params.n_iterations = this.n_iterations;
        params.sample_period = this.sample_period;
        params.random_seed = this.random_seed;
        params.initial_center = this.initial_center;
        params.simulation_kind = DprsWasm.SimulationKind.SimplifiedDomanyKinzel;
        if (this.simulation_kind == "staggered_dk") {
            params.simulation_kind = DprsWasm.SimulationKind.StaggeredDomanyKinzel;
        }
        return params;
    }
    wasm_simulation_kind() {
        var simulation_kind = DprsWasm.SimulationKind.SimplifiedDomanyKinzel;
        if (this.simulation_kind == "staggered_dk") {
            simulation_kind = DprsWasm.SimulationKind.StaggeredDomanyKinzel;
        }
        return simulation_kind;
    }
    from_json(params) {
        const n_iterations = params["n_iterations"];
        const sample_period = params["sample_period"];
        const random_seed = params["random_seed"];
        const initial_center = params["initial_center"];
        const simulation_kind = params["simulation_kind"];
        if (typeof n_iterations == "number") {
            this.n_iterations = n_iterations;
        }
        if (typeof sample_period == "number") {
            this.sample_period = sample_period;
        }
        if (typeof random_seed == "number") {
            this.random_seed = random_seed;
        }
        if (typeof initial_center == "boolean") {
            this.initial_center = initial_center;
        }
        if (typeof simulation_kind == "string") {
            this.simulation_kind = simulation_kind;
        }
    }
}
class Topo {
    constructor() {
        this.periodic = true;
        this.fix_min = false;
        this.fix_max = false;
        this.fix_value = false;
    }
    topo_bc() {
        const topo = new DprsWasm.TopoBc();
        topo.periodic = this.periodic;
        topo.fix_min = this.fix_min;
        topo.fix_max = this.fix_max;
        topo.fix_value = this.fix_value;
        return topo;
    }
    from_json(params) {
        const periodic = params["periodic"];
        const fix_min = params["fix_min"];
        const fix_max = params["fix_max"];
        const fix_value = params["fix_value"];
        if (typeof periodic == "boolean") {
            this.periodic = periodic;
        }
        if (typeof fix_min == "boolean") {
            this.fix_min = fix_min;
        }
        if (typeof fix_max == "boolean") {
            this.fix_max = fix_max;
        }
        if (typeof fix_value == "boolean") {
            this.fix_value = fix_value;
        }
    }
}
class Dims {
    constructor() {
        this.n_x = 400;
        this.n_y = 0;
        this.n_z = 0;
    }
    as_parameters() {
        const dims = new DprsWasm.Dims();
        dims.n_x = this.n_x;
        dims.n_y = this.n_y;
        dims.n_z = this.n_z;
        return dims;
    }
    from_json(dims) {
        const n_x = dims["n_x"];
        const n_y = dims["n_y"];
        const n_z = dims["n_z"];
        if (typeof n_x == "number") {
            this.n_x = n_x;
        }
        if (typeof n_y == "number") {
            this.n_y = n_y;
        }
        if (typeof n_z == "number") {
            this.n_z = n_z;
        }
    }
}
/**
 * This contains JsParameters
 */
export class JsParameters {
    constructor() {
        this.parameters = new DprsWasm.Parameters();
        this.probabilities = new Probabilities();
        this.params = new Params();
        this.topo = [new Topo(), new Topo(), new Topo()];
        this.dims = new Dims();
        this.topo[0].periodic = true;
        this.topo[1].periodic = false;
        this.topo[2].periodic = false;
        this.dims.n_x = 400;
        this.dims.n_y = 0;
        this.dims.n_z = 0;
    }
    as_parameters() {
        this.parameters.probabilities = this.probabilities.as_parameters();
        this.parameters.params = this.params.as_parameters();
        this.parameters.dims = this.dims.as_parameters();
        this.parameters.topo_bc_x = this.topo[0].topo_bc();
        this.parameters.topo_bc_y = this.topo[1].topo_bc();
        this.parameters.topo_bc_z = this.topo[2].topo_bc();
        console.log(this.parameters);
        return this.parameters;
    }
    wasm_simulation_kind() {
        return this.params.wasm_simulation_kind();
    }
    as_json() {
        const parameters = {
            probabilities: this.probabilities,
            params: this.params,
            dims: this.dims,
            topo: this.topo,
        };
        const json = JSON.stringify(parameters);
        console.log(json);
        return json;
    }
    from_json(json) {
        let obj = null;
        try {
            obj = JSON.parse(json);
        }
        catch (error) {
            console.log("Failed to parse json");
            return;
        }
        this.dims.from_json(obj.dims);
        this.params.from_json(obj.params);
        this.probabilities.from_json(obj.probabilities);
        this.topo[0].from_json(obj.topo[0]);
        this.topo[1].from_json(obj.topo[1]);
        this.topo[2].from_json(obj.topo[2]);
        return;
    }
}
