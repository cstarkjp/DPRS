import init from "../pkg/dprs_wasm.js";
import { Log } from "./log.js";
import { JsParameters } from "./js_parameters.js";
import { MainBase } from "./base.js";
class Main extends MainBase {
    constructor(logger) {
        const model = "dk";
        const dim = 1;
        super(logger, model, dim);
        console.log(`${model} ${dim}d child class`);
    }
    get_default_parameters() {
        const p = new JsParameters();
        p.dimensions.n_x = 350;
        p.dimensions.n_y = 1;
        p.dimensions.n_z = 1;
        p.settings.n_iterations = 500;
        p.settings.sample_period = 1;
        p.settings.random_seed = 1;
        p.settings.seed_kind = "random";
        p.settings.simulation_kind = "staggered_dk";
        p.probabilities.p_1 = 0.7054; //0.70548515
        p.probabilities.p_2 = 0.7054;
        p.probabilities.p_conj = 0.0;
        p.probabilities.p_nbr = 0.0;
        p.probabilities.p_diag = 0.0;
        p.probabilities.u_x = 0.0;
        p.probabilities.p_initial = 0.5;
        return p;
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
    init().then(() => {
        complete_init();
    });
});
