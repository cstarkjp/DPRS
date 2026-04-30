import * as html from "./html.js";
import { JsParameters } from "./js_parameters.js";
export class SimulationControls {
    // t_increment: number;
    constructor(ele_id, div_id, dims) {
        this.parameters = new JsParameters();
        this.ele_id = ele_id;
        this.dims = dims;
        // this.t_increment = 1;
        const div = document.getElementById(div_id);
        if (!div) {
            throw new Error(`Failed to find ${div_id} to build SimulationControls`);
        }
        this.div = new html.HtmlElement(div);
        this.build_html();
    }
    get_float(id, min, max) {
        return html.get_input_float(this.ele_id + id, min, max);
    }
    get_int(id, min, max) {
        return html.get_input_int(this.ele_id + id, min, max);
    }
    populate_value(id, value) {
        html.set_input_value(this.ele_id + id, value);
    }
    populate_values() {
        this.populate_value("p_1", this.parameters.probabilities.p_1);
        this.populate_value("p_2", this.parameters.probabilities.p_2);
        this.populate_value("p_diag", this.parameters.probabilities.p_diag);
        this.populate_value("p_conj", this.parameters.probabilities.p_conj);
        this.populate_value("p_initial", this.parameters.probabilities.p_initial);
        this.populate_value("n_iterations", this.parameters.params.n_iterations);
        this.populate_value("sample_period", this.parameters.params.sample_period);
        this.populate_value("random_seed", this.parameters.params.random_seed);
        this.populate_value("n_x", this.parameters.dims.n_x);
        this.populate_value("n_y", this.parameters.dims.n_y);
        this.populate_value("n_z", this.parameters.dims.n_z);
        if (this.parameters.params.seed_kind == "center") {
            html.set_input_checked(this.ele_id + "seed_center", true);
        }
        else if (this.parameters.params.seed_kind == "edge") {
            html.set_input_checked(this.ele_id + "seed_edge", true);
        }
        else {
            html.set_input_checked(this.ele_id + "seed_random", true);
        }
        if (this.parameters.params.simulation_kind == "simple_dk") {
            html.set_input_checked(this.ele_id + "sk_simple_dk", true);
            // this.t_increment = 1;
        }
        else if (this.parameters.params.simulation_kind == "staggered_dk") {
            html.set_input_checked(this.ele_id + "sk_staggered_dk", true);
            // this.t_increment = 2;
        }
        else if (this.parameters.params.simulation_kind == "bedload") {
            html.set_input_checked(this.ele_id + "sk_bedload", true);
            // this.t_increment = 1;
        }
    }
    set_ic_randomize() {
        html.set_input_checked(this.ele_id + "seed_random", true);
    }
    set_ic_centralcell() {
        html.set_input_checked(this.ele_id + "seed_center", true);
    }
    set_ic_edgecell() {
        html.set_input_checked(this.ele_id + "seed_edge", true);
    }
    set_simple_dk() {
        html.set_input_checked(this.ele_id + "sk_simple_dk", true);
        // this.t_increment = 1;
    }
    set_staggered_dk() {
        html.set_input_checked(this.ele_id + "sk_staggered_dk", true);
        // this.t_increment = 2;
    }
    set_bedload() {
        html.set_input_checked(this.ele_id + "sk_bedload", true);
        // this.t_increment = 1;
    }
    populate_parameters() {
        const simulation_choice = html.get_input_radio_checked(this.ele_id + "sim_kind");
        const seed_kind = html.get_input_radio_checked(this.ele_id + "_seed_kind");
        this.parameters.probabilities.p_1 = this.get_float("p_1", 0, 1);
        this.parameters.probabilities.p_2 = this.get_float("p_2", 0, 1);
        this.parameters.probabilities.p_diag = this.get_float("p_diag", 0, 1);
        this.parameters.probabilities.p_conj = this.get_float("p_conj", 0, 1);
        this.parameters.probabilities.p_initial = this.get_float("p_initial", 0, 1);
        if (simulation_choice !== null) {
            this.parameters.params.simulation_kind = simulation_choice;
        }
        if (seed_kind !== null) {
            this.parameters.params.seed_kind = seed_kind;
        }
        this.parameters.params.n_iterations = this.get_int("n_iterations", 0, 1000000);
        this.parameters.params.sample_period = this.get_int("sample_period", 1, 100000);
        this.parameters.params.random_seed = this.get_int("random_seed", 1, 100000);
        this.parameters.dims.n_x = this.get_int("n_x", 10, 10000);
        this.parameters.dims.n_y = this.get_int("n_y", 10, 10000);
        this.parameters.dims.n_z = this.get_int("n_z", 10, 10000);
    }
    build_html() {
        const ele_id = this.ele_id;
        const dims = this.dims;
        this.div.clear();
        const table = this.div.add_ele("table", { classes: "sim_ctrl" });
        const dims_table = table
            .add_ele("tr")
            .add_ele("td")
            .add_ele("table", { classes: "dims" });
        const probs_table = table
            .add_ele("tr")
            .add_ele("td")
            .add_ele("table", { classes: "probability" });
        const param_table = table
            .add_ele("tr")
            .add_ele("td")
            .add_ele("table", { classes: "param" });
        const seed_table = table
            .add_ele("tr")
            .add_ele("td")
            .add_ele("table", { classes: "seed" });
        const control_table = table
            .add_ele("tr")
            .add_ele("td")
            .add_ele("table", { classes: "control" });
        {
            const tr = dims_table.add_ele("tr", { id: ele_id + "dims" });
            const td = tr.add_ele("td");
            // const label_nx = "<div><mjx-container class='MathJax CtxtMenu_Attached_0' jax='CHTML' style='font-size: 119.5%; position: relative;' tabindex='0' ctxtmenu_counter='1'><mjx-math class='MJX-TEX' aria-hidden='true'><mjx-msub><mjx-mi class='mjx-i'><mjx-c class='mjx-c1D45B TEX-I'></mjx-c></mjx-mi><mjx-script style='vertical-align: -0.15em;'><mjx-mi class='mjx-i' size='s'><mjx-c class='mjx-c1D465 TEX-I'></mjx-c></mjx-mi></mjx-script></mjx-msub></mjx-math><mjx-assistive-mml unselectable='on' display='inline'><mjx-container class='MathJax CtxtMenu_Attached_0' jax='CHTML' style='font-size: 119.5%; position: relative;' tabindex='0' ctxtmenu_counter='4'><mjx-math class='MJX-TEX' aria-hidden='true'><mjx-msub><mjx-mi class='mjx-i'><mjx-c class='mjx-c1D45B TEX-I'></mjx-c></mjx-mi><mjx-script style='vertical-align: -0.15em;'><mjx-mi class='mjx-i' size='s'><mjx-c class='mjx-c1D465 TEX-I'></mjx-c></mjx-mi></mjx-script></mjx-msub></mjx-math><mjx-assistive-mml unselectable='on' display='inline'><math xmlns='http://www.w3.org/1998/Math/MathML'><msub><mi>n</mi><mi>x</mi></msub></math></mjx-assistive-mml></mjx-container></mjx-assistive-mml></mjx-container></div>";
            td.add_label("n_x", { classes: "sim_controls_label" }).set_content("x:");
            td.add_input_text("n_x", "20", {
                id: this.ele_id + "n_x",
                classes: "sim_controls_text dims_n_text",
            });
            if (dims >= 2) {
                const td = tr.add_ele("td");
                td.add_label("n_y", { classes: "sim_controls_label" }).set_content("y:");
                td.add_input_text("n_y", "20", {
                    id: this.ele_id + "n_y",
                    classes: "sim_controls_text dims_n_text",
                });
            }
            if (dims >= 3) {
                const td = tr.add_ele("td");
                td.add_label("n_z", { classes: "sim_controls_label" }).set_content("z:");
                const x = td.add_input_text("n_z", "20", {
                    id: this.ele_id + "n_z",
                    classes: "sim_controls_text dims_n_text",
                });
            }
            // const td_dummy1 = tr.add_ele("td");
            // td_dummy1.add_label("dummy1", { classes: "dummy" }).set_content("   ");
        }
        {
            const tr = probs_table.add_ele("tr", { id: ele_id + "probability" });
            for (const [label, thing] of [
                ["p_1", "p_1"], ["p_2", "p_2"], ["p_d", "p_diag"],
            ]) {
                const td = tr.add_ele("td");
                td.add_label(thing, { classes: "sim_controls_label" }).set_content(label + ":");
                td.add_input_text(thing, "0.5", {
                    id: this.ele_id + thing,
                    classes: "sim_controls_text prob_text",
                });
            }
        }
        {
            const tr = probs_table.add_ele("tr", { id: ele_id + "probability" });
            for (const [label, thing] of [
                ["p_0", "p_initial"], ["p_x", "p_conj"],
            ]) {
                const td = tr.add_ele("td");
                td.add_label(thing, { classes: "sim_controls_label" }).set_content(label + ":");
                td.add_input_text(thing, "0", {
                    id: this.ele_id + thing,
                    classes: "sim_controls_text prob_text",
                });
            }
        }
        {
            const tr = param_table.add_ele("tr", { id: ele_id + "sim_controls" });
            for (const [label, name, value] of [
                ["Steps", "n_iterations", "1000"],
                ["Slicing", "sample_period", "20"],
                ["Seed", "random_seed", "1"],
            ]) {
                const td = tr.add_ele("td");
                td.add_label(name, { classes: "sim_controls_label" }).set_content(label + ":");
                td.add_input_text(name, value, {
                    id: this.ele_id + name,
                    classes: "sim_controls_text params_text",
                });
            }
        }
        {
            const tr = seed_table.add_ele("tr", { id: ele_id + "_seed_kind" });
            for (const [name, value] of [
                ["edge", "Edge cell"],
                ["center", "Center cell"],
                ["random", "Randomized"],
            ]) {
                const td = tr.add_ele("td");
                td.add_input_radio(ele_id + "_seed_kind", name, true, {
                    id: ele_id + "seed_" + name,
                    classes: "sim_controls_radio " + name,
                });
                td.add_label(ele_id + "seed_" + name, {
                    classes: "sim_controls_label",
                }).set_content(value);
            }
        }
        {
            const tr = seed_table.add_ele("tr", { id: ele_id + "sim_kind" });
            for (const [name, value] of [
                ["simple_dk", "Simple"],
                ["staggered_dk", "Staggered"],
                ["bedload", "Bedload"],
            ]) {
                const td = tr.add_ele("td");
                td.add_input_radio(ele_id + "_sim_kind", name, true, {
                    id: ele_id + "sk_" + name,
                    classes: "sim_controls_radio " + name,
                });
                td.add_label(ele_id + "sk_" + name, {
                    classes: "sim_controls_label " + name,
                }).set_content(value);
            }
        }
        {
            const tr = control_table.add_ele("tr", { id: ele_id + "controls" });
            const td_run = tr.add_ele("td");
            td_run.add_input_button("Run simulation", () => {
                window.main.run_simulation(dims);
            }, {
                id: ele_id + "run_simulation",
                classes: "controls simulation run_simulation",
            });
            const td_save = tr.add_ele("td");
            td_save.add_input_button("Save simulation", () => {
                window.main.save_simulation(dims);
            }, {
                id: ele_id + "save_simulation",
                classes: "controls simulation save_simulation",
            });
        }
    }
}
