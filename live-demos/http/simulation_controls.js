import * as html from "./html.js";
import { JsParameters } from "./js_parameters.js";
export class SimulationControls {
    constructor(ele_id, div_id, dim, presets = null) {
        this.parameters = new JsParameters();
        this.ele_id = ele_id;
        this.dim = dim;
        this.presets = presets;
        const div = document.getElementById(div_id);
        if (!div) {
            throw new Error(`Failed to find ${div_id} to build SimulationControls`);
        }
        this.div = new html.HtmlElement(div);
        this.build_html();
    }
    // Set parameter values in web page
    populate_webpage_entries() {
        this.set_webpage_entry("p_1", this.parameters.probabilities.p_1);
        this.set_webpage_entry("p_2", this.parameters.probabilities.p_2);
        this.set_webpage_entry("p_conj", this.parameters.probabilities.p_conj);
        this.set_webpage_entry("p_nbr", this.parameters.probabilities.p_nbr);
        this.set_webpage_entry("p_diag", this.parameters.probabilities.p_diag);
        this.set_webpage_entry("u_x", this.parameters.probabilities.u_x);
        this.set_webpage_entry("p_initial", this.parameters.probabilities.p_initial);
        this.set_webpage_entry("n_iterations", this.parameters.settings.n_iterations);
        this.set_webpage_entry("sample_period", this.parameters.settings.sample_period);
        this.set_webpage_entry("random_seed", this.parameters.settings.random_seed);
        this.set_webpage_entry("n_x", this.parameters.dimensions.n_x);
        this.set_webpage_entry("n_y", this.parameters.dimensions.n_y);
        this.set_webpage_entry("n_z", this.parameters.dimensions.n_z);
        if (this.parameters.settings.initial_seeding == "center") {
            this.set_webpage_radio_button("seed_center", true);
        }
        else if (this.parameters.settings.initial_seeding == "edge") {
            this.set_webpage_radio_button("seed_edge", true);
        }
        else {
            this.set_webpage_radio_button("seed_random", true);
        }
        // if (this.presets != null) {
        //   console.log(`Setting preset in web page`,)
        //   this.set_webpage_radio_button("sim_preset" + this.parameters.preset.toString(), true);
        // }
        if (this.parameters.settings.growth_scheme == "Simple") {
            this.set_webpage_radio_button("simple", true);
        }
        else if (this.parameters.settings.growth_scheme == "Staggered") {
            this.set_webpage_radio_button("staggered", true);
        }
    }
    // Get parameter values from web page
    set_parameters_from_webpage_entries() {
        const growth_model = html.get_input_radio_checked(this.ele_id + "growth_model");
        const growth_scheme = html.get_input_radio_checked(this.ele_id + "growth_scheme");
        const initial_seeding = html.get_input_radio_checked(this.ele_id + "_seed_kind");
        const preset = html.get_input_radio_checked(this.ele_id + "_preset");
        if (growth_model !== null) {
            this.parameters.settings.growth_model = growth_model;
        }
        if (growth_scheme !== null) {
            this.parameters.settings.growth_scheme = growth_scheme;
        }
        if (initial_seeding !== null) {
            this.parameters.settings.initial_seeding = initial_seeding;
        }
        if (preset !== null) {
            this.parameters.preset = Number(preset);
        }
        this.parameters.probabilities.p_1 = this.get_webpage_float("p_1", 0, 1);
        this.parameters.probabilities.p_2 = this.get_webpage_float("p_2", 0, 1);
        this.parameters.probabilities.p_conj = this.get_webpage_float("p_conj", 0, 1);
        this.parameters.probabilities.p_nbr = this.get_webpage_float("p_nbr", 0, 1);
        this.parameters.probabilities.p_diag = this.get_webpage_float("p_diag", 0, 1);
        this.parameters.probabilities.u_x = this.get_webpage_float("u_x", -1e9, +1e9);
        this.parameters.probabilities.p_initial = this.get_webpage_float("p_initial", 0, 1);
        this.parameters.settings.n_iterations = this.get_webpage_int("n_iterations", 0, 1000000);
        this.parameters.settings.sample_period = this.get_webpage_int("sample_period", 1, 100000);
        this.parameters.settings.random_seed = this.get_webpage_int("random_seed", 1, 100000);
        this.parameters.dimensions.n_x = this.get_webpage_int("n_x", 10, 10000);
        this.parameters.dimensions.n_y = this.get_webpage_int("n_y", 10, 10000);
        this.parameters.dimensions.n_z = this.get_webpage_int("n_z", 10, 10000);
    }
    build_html() {
        const ele_id = this.ele_id;
        const dims = this.dim;
        this.div.clear();
        const table = this.div.add_ele("table", { classes: "sim_controls" });
        const dims_probabilities_table = table
            .add_ele("tr")
            .add_ele("td")
            .add_ele("table", { classes: "dims_probabilities" });
        const steps_slicing_seed_table = table
            .add_ele("tr")
            .add_ele("td")
            .add_ele("table", { classes: "steps_slicing_seed" });
        const edge_center_randomized_table = table
            .add_ele("tr")
            .add_ele("td")
            .add_ele("table", { classes: "edge_center_randomized" });
        const simple_staggered_table = table
            .add_ele("tr")
            .add_ele("td")
            .add_ele("table", { classes: "simple_staggered" });
        const presets_table = table
            .add_ele("tr")
            .add_ele("td")
            .add_ele("table", { classes: "presets" });
        const run_save_table = table
            .add_ele("tr")
            .add_ele("td")
            .add_ele("table", { classes: "run_save" });
        // Dims
        {
            let id = ele_id + "dims";
            const tr = dims_probabilities_table.add_ele("tr", { id: id });
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
        }
        // Probabilities
        {
            let id = ele_id + "probability";
            const tr = dims_probabilities_table.add_ele("tr", { id: id });
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
            let id = ele_id + "probability";
            const tr = dims_probabilities_table.add_ele("tr", { id: id });
            for (const [label, thing] of [
                ["p_0", "p_initial"], ["u_x", "u_x"], ["p_ext", "p_conj"],
            ]) {
                const td = tr.add_ele("td");
                td.add_label(thing, { classes: "sim_controls_label" }).set_content(label + ":");
                td.add_input_text(thing, "0", {
                    id: this.ele_id + thing,
                    classes: "sim_controls_text prob_text",
                });
            }
        }
        // Steps / slicing / seed
        {
            let id = ele_id + "sim_controls";
            const tr = steps_slicing_seed_table.add_ele("tr", { id: id });
            for (const [name, label, value] of [
                ["n_iterations", "Steps", "1000"],
                ["sample_period", "Slicing", "20"],
                ["random_seed", "Seed", "1"],
            ]) {
                const td = tr.add_ele("td");
                td.add_label(name, { classes: "sim_controls_label" }).set_content(label + ":");
                td.add_input_text(name, value, {
                    id: this.ele_id + name,
                    classes: "sim_controls_text params_text",
                });
            }
        }
        // Edge / center / randomized
        {
            let id = ele_id + "_seed_kind";
            const tr = edge_center_randomized_table.add_ele("tr", { id: ele_id + "_seed_kind" });
            for (const [name, value] of [
                ["edge", "Edge cell:"],
                ["center", "Center cell:"],
                ["random", "Randomized:"],
            ]) {
                const td = tr.add_ele("td");
                td.add_label(ele_id + "seed_" + name, {
                    classes: "sim_controls_label",
                }).set_content(value);
                td.add_input_radio(id, name, true, {
                    id: ele_id + "seed_" + name,
                    classes: "sim_controls_radio " + name,
                });
            }
        }
        // Presets
        {
            let id = ele_id + "sim_preset";
            const tr = presets_table.add_ele("tr", { id: id });
            // // Presets row label
            // const td = tr.add_ele("td");
            // const name = "label";
            // const value = "Presets:"
            // td.add_label(
            //   ele_id + "sim_" + name, { classes: "sim_preset_label " + name, }
            // ).set_content(value);
            // // console.log(`Creating radio button for bedload_2d preset ${this.presets}`,)
            // function preset_select_radio(event: Event) {
            //   const target = event.target as HTMLInputElement;
            //   const preset = Number(target.value);
            //   (window as any).main.enact_preset(preset);
            // }
            // if (this.presets != null) {
            //   for (const [name, value] of this.presets) {
            //     const td = tr.add_ele("td");
            //     td.add_label(
            //       ele_id + "sim_" + name, { classes: "sim_preset_label " + name, }
            //     ).set_content(value);
            //     td.add_input_radio_with_callback(
            //       id, name!, true,
            //       preset_select_radio,
            //       {
            //         id: ele_id + "sim_preset" + name,
            //         classes: "sim_preset_radio " + name,
            //       }
            //     );
            //   }
            // }
            console.log(`Creating dropdown menu for bedload_2d preset ${this.presets}`);
            function preset_select_dropdown(event) {
                const target = event.target;
                const preset = Number(target.value);
                window.main.enact_preset(preset);
            }
            if (this.presets != null) {
                const td = tr.add_ele("td");
                const value = "Example parameter choices:          ";
                const name = "label";
                td.add_label(ele_id + "sim_" + name, { classes: "sim_preset_label " + name, }).set_content(value);
                td.add_input_dropdown_with_callback(this.presets, preset_select_dropdown);
            }
        }
        // Simple / staggered
        {
            let id = ele_id + "growth_scheme";
            const tr = simple_staggered_table.add_ele("tr", { id: id });
            for (const [name, value] of [
                ["simple", "Simple"],
                ["staggered", "Staggered"],
            ]) {
                const td = tr.add_ele("td");
                // Using value not name because we want upper case
                td.add_input_radio(id, value, true, {
                    id: ele_id + name,
                    classes: "sim_controls_radio " + name,
                });
                td.add_label(ele_id + name, {
                    classes: "sim_controls_label " + name,
                }).set_content(value);
            }
        }
        // Run / save
        {
            let id = ele_id + "controls";
            const tr = run_save_table.add_ele("tr", { id: id });
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
    set_webpage_entry(id, value) {
        html.set_input_value(this.ele_id + id, value);
    }
    set_webpage_radio_button(id, value) {
        html.set_input_checked(this.ele_id + id, value);
    }
    get_webpage_float(id, min, max) {
        return html.get_input_float(this.ele_id + id, min, max);
    }
    get_webpage_int(id, min, max) {
        return html.get_input_int(this.ele_id + id, min, max);
    }
}
