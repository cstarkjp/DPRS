/**
 * This contains JsParameters
 */
import * as DprsWasm from "../pkg/dprs_wasm.js";
class Probabilities {
    constructor() {
        /**
         * This contains JsParameters
         */
        this.p_1 = 0.70548515;
        this.p_2 = 0.70548515;
        this.p_conj = 0.0;
        this.p_nbr = 0.5;
        this.p_diag = 0.1;
        this.u_x = 0.0;
        this.p_initial = 0.5;
    }
    set_parameters(parameters) {
        parameters.p_1 = this.p_1;
        parameters.p_2 = this.p_2;
        parameters.p_conj = this.p_conj;
        parameters.p_nbr = this.p_nbr;
        parameters.p_diag = this.p_diag;
        parameters.u_x = this.u_x;
        parameters.p_initial = this.p_initial;
    }
    from_json(probabilities) {
        const p_1 = probabilities["p_1"];
        const p_2 = probabilities["p_2"];
        const p_conj = probabilities["p_conj"];
        const p_nbr = probabilities["p_nbr"];
        const p_diag = probabilities["p_diag"];
        const u_x = probabilities["u_x"];
        const p_initial = probabilities["p_initial"];
        if (typeof p_1 == "number") {
            this.p_1 = p_1;
        }
        if (typeof p_2 == "number") {
            this.p_2 = p_2;
        }
        if (typeof p_conj == "number") {
            this.p_conj = p_conj;
        }
        if (typeof p_nbr == "number") {
            this.p_nbr = p_nbr;
        }
        if (typeof p_diag == "number") {
            this.p_diag = p_diag;
        }
        if (typeof u_x == "number") {
            this.u_x = u_x;
        }
        if (typeof p_initial == "number") {
            this.p_initial = p_initial;
        }
    }
}
class Settings {
    constructor() {
        this.n_iterations = 500;
        this.sample_period = 1;
        this.random_seed = 1;
        this.initial_seeding = "center";
        this.growth_model = "DomanyKinzel";
        this.growth_scheme = "Staggered";
    }
    set_parameters(parameters) {
        parameters.n_iterations = this.n_iterations;
        parameters.sample_period = this.sample_period;
        parameters.random_seed = this.random_seed;
        parameters.initial_condition = this.initial_seeding;
    }
    wasm_growth_model() {
        return this.growth_model;
    }
    wasm_growth_scheme() {
        return this.growth_scheme;
    }
    from_json(params_dict) {
        const n_iterations = params_dict["n_iterations"];
        const sample_period = params_dict["sample_period"];
        const random_seed = params_dict["random_seed"];
        const initial_seeding = params_dict["initial_seeding"];
        const growth_model = params_dict["growth_model"];
        const growth_scheme = params_dict["growth_scheme"];
        if (typeof n_iterations == "number") {
            this.n_iterations = n_iterations;
        }
        if (typeof sample_period == "number") {
            this.sample_period = sample_period;
        }
        if (typeof random_seed == "number") {
            this.random_seed = random_seed;
        }
        if (typeof initial_seeding == "string") {
            this.initial_seeding = initial_seeding;
        }
        if (typeof growth_model == "string") {
            this.growth_model = growth_model;
        }
        if (typeof growth_scheme == "string") {
            this.growth_scheme = growth_scheme;
        }
    }
}
class Topology {
    constructor() {
        this.periodic = true;
        this.fix_min = false;
        this.fix_max = false;
        this.fix_value = false;
    }
    topology_bc() {
        const topology = new DprsWasm.TopologyBc();
        topology.periodic = this.periodic;
        topology.fix_min = this.fix_min;
        topology.fix_max = this.fix_max;
        topology.fix_value = this.fix_value;
        return topology;
    }
    from_json(params_dict) {
        const periodic = params_dict["periodic"];
        const fix_min = params_dict["fix_min"];
        const fix_max = params_dict["fix_max"];
        const fix_value = params_dict["fix_value"];
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
class Dimensions {
    constructor() {
        this.n_x = 350;
        this.n_y = 0;
        this.n_z = 0;
    }
    set_parameters(parameters) {
        parameters.n_x = this.n_x;
        parameters.n_y = this.n_y;
        parameters.n_z = this.n_z;
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
        this.settings = new Settings();
        this.topology = [new Topology(), new Topology(), new Topology()];
        this.dimensions = new Dimensions();
        this.preset = 0;
        this.topology[0].periodic = true;
        this.topology[1].periodic = true;
        this.topology[2].periodic = true;
        this.dimensions.n_x = 350;
        this.dimensions.n_y = 0;
        this.dimensions.n_z = 0;
    }
    as_parameters() {
        this.probabilities.set_parameters(this.parameters);
        this.settings.set_parameters(this.parameters);
        this.dimensions.set_parameters(this.parameters);
        this.parameters.topology_bc_x = this.topology[0].topology_bc();
        this.parameters.topology_bc_y = this.topology[1].topology_bc();
        this.parameters.topology_bc_z = this.topology[2].topology_bc();
        return this.parameters;
    }
    wasm_growth_model() {
        return this.settings.wasm_growth_model();
    }
    wasm_growth_scheme() {
        return this.settings.wasm_growth_scheme();
    }
    dim() {
        if (this.dimensions.n_y > 1) {
            return 2;
        }
        return 1;
    }
    as_json() {
        const parameters = {
            probabilities: this.probabilities,
            settings: this.settings,
            dims: this.dimensions,
            topo: this.topology,
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
        this.dimensions.from_json(obj.dims);
        this.settings.from_json(obj.settings);
        this.probabilities.from_json(obj.probabilities);
        this.topology[0].from_json(obj.topo[0]);
        this.topology[1].from_json(obj.topo[1]);
        this.topology[2].from_json(obj.topo[2]);
        return;
    }
}
