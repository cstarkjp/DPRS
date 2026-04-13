import * as html from "./html.js";
import * as log from "./log.js";
/**
 * A visualization 'div' for simulations
 *
 * At present this can only display 1D simulations
 *
 */
export class Visualize {
    /**
     *
     * Create a new Visualize for a simulation
     *
     * This does not populate it
     *
     */
    constructor(logger, simulation, div_id) {
        this.log = new log.Logger(logger, "viz");
        this.simulation = simulation;
        this.width = 0;
        this.height = 0;
        const div = document.getElementById(div_id);
        if (!div) {
            throw new Error(`div ${div_id} not found, to create a Visualize canvas`);
        }
        this.div = new html.HtmlElement(div);
    }
    /**
     *
     * Create a simple 2D canvas for a 1D simulation (in the X) with time increasing in the Y
     *
     */
    canvas_simple(scale, sim_control) {
        this.log.push_reason("canvas");
        const stagger = this.simulation.results_are_staggered();
        var x_ofs = 0;
        var x_scale = scale;
        var y_scale = scale;
        if (stagger) {
            y_scale = 0.5 * y_scale;
            x_ofs = 0.5;
        }
        this.width = this.simulation.parameters.dims.n_x * x_scale;
        this.height = this.simulation.n_results() * y_scale;
        this.log.info(`Created canvas size ${this.width} x ${this.height} with stagger ${stagger} and scale ${x_scale}x${y_scale}`);
        this.div.clear();
        const canvas = this.div.add_ele("canvas", "", "visualize");
        const canvas_ele = canvas.ele;
        canvas_ele.width = this.width;
        canvas_ele.height = this.height;
        const ctx = canvas_ele.getContext("2d");
        if (ctx === null) {
            this.log.error("Failed to get 2D context from HTML Canvas element for the visualizer");
            this.log.pop_reason();
            return;
        }
        for (let y = 0; y <= this.simulation.n_results(); y++) {
            let data = this.simulation.result(y);
            if (!data) {
                break;
            }
            for (let x = 0; x < data.length; x += 1) {
                if (data[x] != 0) {
                    ctx.fillStyle = "purple";
                }
                else {
                    ctx.fillStyle = "lightgrey";
                }
                ctx.fillRect((x + x_ofs) * x_scale, y * y_scale, x_scale, y_scale);
            }
            if (stagger) {
                x_ofs = 0.5 - x_ofs;
            }
        }
        this.log.info("Completed canvas");
        this.log.pop_reason();
    }
}
