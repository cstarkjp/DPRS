import init from "../pkg/dprs_wasm.js";
import { Log } from "./log.js";
import { JsParameters } from "./js_parameters.js";
import { MainBase } from "./base.js";
class Main extends MainBase {
    constructor(logger) {
        const model = "DKBedload";
        const dim = 2;
        const zoom = 2.2;
        const do_rough_background = true;
        super(logger, model, dim, zoom, do_rough_background);
        // console.log(`${model} ${dim}d child class`);
    }
    get_default_parameters() {
        const p = new JsParameters();
        p.dimensions.n_x = 150;
        p.dimensions.n_y = 100;
        p.dimensions.n_z = 1;
        p.settings.n_iterations = 500;
        p.settings.sample_period = 1;
        p.settings.random_seed = 31;
        p.settings.initial_seeding = "edge";
        p.settings.growth_model = "DKBedload";
        p.settings.growth_scheme = "BedloadC";
        // These values are chosen to lie on the p1-p2 phase diagram boundary
        // p.probabilities.p_1 = 0.61487;  // random_seed: 5
        // p.probabilities.p_2 = 0.9;
        // p.probabilities.p_1 = 0.72082;   // random_seed: 2
        // p.probabilities.p_2 = 0.7;
        p.probabilities.p_1 = 0.8135; // random_seed: 13  // 31
        p.probabilities.p_2 = 0.5;
        // p.probabilities.p_1 = 0.8945;   // random_seed: 6
        // p.probabilities.p_2 = 0.3;
        // p.probabilities.p_1 = 0.96693;  // random_seed: 4
        // p.probabilities.p_2 = 0.1;
        // p.probabilities.p_1 = 0.99677;  // random_seed: ?
        // p.probabilities.p_2 = 0.01; 
        p.probabilities.p_conj = 1e-6;
        p.probabilities.p_nbr = 0.5;
        p.probabilities.p_diag = 0.1;
        p.probabilities.u_x = 1;
        p.probabilities.p_initial = 0.001;
        p.preset = 2;
        return p;
    }
    run_simulation(dim) {
        super.run_simulation(dim);
    }
    get_presets() {
        return [
            ["1", "Critical  p1~0.62  p2~0.90"],
            ["2", "Critical  p1~0.81  p2~0.50"],
            ["3", "Critical  p1~0.89  p2~0.30"],
            ["4", "Critical  p1~0.97  p2~0.10"],
            ["5", "Critical  p1~0.97  p2~0.01"],
        ];
    }
    enact_preset(preset) {
        console.log(`Enacting preset ${preset}`);
        var p = this.get_default_parameters();
        switch (preset) {
            case 0:
                p.preset = 0;
                return;
            case 1:
                p.preset = 1;
                p.probabilities.p_1 = 0.61487;
                p.probabilities.p_2 = 0.9;
                p.probabilities.p_conj = 1e-5;
                p.settings.random_seed = 5;
                p.settings.n_iterations = 1000;
                break;
            case 2:
                p.preset = 2;
                p.probabilities.p_1 = 0.8135;
                p.probabilities.p_2 = 0.5;
                p.probabilities.p_conj = 1e-5;
                p.settings.random_seed = 31;
                p.settings.n_iterations = 1000;
                break;
            case 3:
                p.preset = 3;
                p.probabilities.p_1 = 0.8945;
                p.probabilities.p_2 = 0.3;
                p.probabilities.p_conj = 1e-5;
                p.settings.random_seed = 6;
                p.settings.n_iterations = 1000;
                break;
            case 4:
                p.preset = 4;
                p.probabilities.p_1 = 0.96693;
                p.probabilities.p_2 = 0.1;
                p.probabilities.p_conj = 1e-5;
                p.settings.random_seed = 4;
                p.settings.n_iterations = 1000;
                break;
            case 5:
                p.preset = 5;
                p.probabilities.p_1 = 0.99677;
                p.probabilities.p_2 = 0.01;
                p.probabilities.p_conj = 1e-5;
                p.settings.random_seed = 1;
                p.settings.n_iterations = 1000;
                break;
            default:
                break;
        }
        this.simulation_controls.parameters = p;
        this.simulation_controls.populate_webpage_entries();
    }
}
window.main = null;
function complete_init() {
    const window_log = new Log("Log");
    const main = new Main(window_log); //window.location.search
    window.log = window_log;
    window.main = main;
}
window.addEventListener("load", (e) => {
    init().then(() => { complete_init(); });
});
