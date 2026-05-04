import * as html from "./html.js";
import * as log from "./log.js";
import { Animate } from "./animate.js";
/**
 * A visualization 'div' for simulations
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
        /** The simulation controls in use for this visualization
         *
         * This may be changed if a simulation of a different dimension is run
         */
        this.simulation_controls = null;
        /** Width of the required canvas
         *
         */
        this.width = 0;
        /** Height of the required canvas
         *
         */
        this.height = 0;
        /** Zoom scale to use
         *
         */
        this.zoom = 1;
        this.max_zoom = 5;
        /** Which direction to animate 'time slice' when animating
         *
         */
        this.slice_delta = 1;
        /** Target frames per second of animation */
        this.frames_per_second = 120;
        /** Animation state (I see no reason why we can't track this...) */
        this.is_playing = false;
        /** Slice increment for simple vs staggered */
        this.t_increment = 1;
        /** Random canvas for "rough" background */
        this.rough_background = null;
        this.do_rough_background = true;
        this.log = new log.Logger(logger, "viz");
        this.simulation = simulation;
        this.anim = new Animate((time) => this.animation_tick(time));
        this.slice = 0;
        this.t_increment = 1;
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
    canvas_1d(sim_control) {
        this.log.push_reason("canvas_1d");
        var x_ofs = 0;
        const x_scale = this.zoom;
        var y_scale = this.zoom;
        const is_staggered = this.simulation.results_are_staggered();
        if (is_staggered) {
            y_scale = 0.5 * y_scale;
            x_ofs = 0.5;
            this.t_increment = 2;
        }
        else {
            this.t_increment = 1;
        }
        this.width = this.simulation.parameters.dimensions.n_x * x_scale;
        this.height = this.simulation.n_results() * y_scale;
        this.log.info(`Created canvas size ${this.width} x ${this.height} with stagger ${is_staggered} and scale ${x_scale}x${y_scale}`);
        this.div.clear();
        const canvas = this.div.add_ele("canvas", {
            classes: "visualize canvas_1d",
        });
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
            if (is_staggered) {
                x_ofs = 0.5 - x_ofs;
            }
        }
        this.log.info("Completed canvas");
        this.log.pop_reason();
    }
    /**
     *
     * Create a simple 2D canvas for a 1D simulation (in the X) with time increasing in the Y
     *
     */
    canvas_2d(sim_control) {
        this.log.push_reason("canvas_2d");
        const is_staggered = this.simulation.results_are_staggered();
        if (is_staggered) {
            this.t_increment = 2;
        }
        else {
            this.t_increment = 1;
        }
        const x_scale = this.zoom;
        const y_scale = this.zoom;
        this.width = this.simulation.parameters.dimensions.n_x * x_scale;
        this.height = this.simulation.parameters.dimensions.n_y * y_scale;
        /*
        this.log.info(
          `Created canvas size ${this.width} x ${this.height} with scale ${x_scale}x${y_scale}`,
        );
        */
        this.div.clear();
        const canvas = this.div.add_ele("canvas", {
            classes: "visualize canvas_2d",
        });
        const canvas_ele = canvas.ele;
        canvas_ele.width = this.width;
        canvas_ele.height = this.height;
        const ctx = canvas_ele.getContext("2d");
        if (ctx === null) {
            this.log.error("Failed to get 2D context from HTML Canvas element for the visualizer");
            this.log.pop_reason();
            return;
        }
        // Get the lattice size
        let n_x = this.simulation.parameters.dimensions.n_x;
        let n_y = this.simulation.parameters.dimensions.n_y;
        if (this.do_rough_background) {
            // Make a "rough" looking canvas
            ctx.fillStyle = "black";
            ctx.fillRect(0, 0, this.width, this.height);
            var redo;
            if (this.rough_background == null) {
                redo = true;
            }
            else {
                // Make sure to rebuild the rough bgrd if we've made the lattice bigger
                // and run a new sim
                if (this.rough_background.width < n_x * this.max_zoom ||
                    this.rough_background.height < n_y * this.max_zoom) {
                    redo = true;
                }
                else {
                    redo = false;
                }
            }
            if (redo) {
                this.rough_background = ctx.createImageData(n_x * this.max_zoom, n_y * this.max_zoom);
                let rough_canvas_data = this.rough_background.data;
                for (let i = 0; i < rough_canvas_data.length; i += 4) {
                    let alpha = Math.random() * 30 + 30;
                    rough_canvas_data[i + 3] = alpha;
                }
            }
            ctx.putImageData(this.rough_background, 0, 0);
        }
        else {
            // Make a blank canvas
            // Unoccupied cells are colored grey
            ctx.fillStyle = "lightgrey";
            ctx.fillRect(0, 0, this.width, this.height);
        }
        // Get this lattice slice (flattened into a 1d array) maybe
        const t_slice = this.slice;
        // const lattice_slice = this.simulation.result(t_slice);
        const lattice_slice = this.simulation.result_sum_kernel_with_threshold(t_slice, 2, 1, 1);
        n_x = n_x - 1;
        n_y = n_y - 1;
        // Print the time slice in the lower-left corner of the canvas
        const offset = 10;
        ctx.font = "12px Arial";
        ctx.fillStyle = "#505050";
        ctx.fillText(t_slice.toString(), offset, n_y * y_scale - offset);
        // Occupied cells are colored purple
        ctx.fillStyle = "purple";
        // Color in the occupied cells with appropriately size pixel rectangles
        const empty = 0;
        if (!lattice_slice) {
            this.log.info(`No data in slice ${this.slice}`);
        }
        else {
            // Loop over the lattice in (x,y) - once scaled we have canvas pixel coordinates
            for (let y = 0; y < n_y; y++) {
                let previous_cell_state = null;
                let x_start = null;
                // This isn't the correct way to get ux, but...
                const u_x = this.simulation_controls.get_webpage_float("u_x", -100, 100);
                // const u_x = this.simulation_controls!.parameters.probabilities.u_x;
                // const u_x = this.simulation.parameters.probabilities.u_x;
                const x_sense = Math.sign(u_x);
                const x_drift = Math.abs(u_x) * t_slice;
                const x_shift = Math.trunc(x_drift) % n_x;
                // const x_error = x_drift - x_shift;
                for (let x = 0; x < n_x; x++) {
                    // This is where a velocity shift can be implemented for time slice t
                    // with a shift ~ (u_x * t * (n_x/L)) modulo n_x
                    let i_cell = y * n_x + ((x - x_sense * x_shift + n_x) % n_x);
                    const cell_state = lattice_slice[i_cell];
                    // At the start of the row, when x=0, previous_cell_state=null,
                    // so this is skipped
                    if (previous_cell_state !== null &&
                        cell_state != previous_cell_state) {
                        // Plot a rectangle that's the RLE width of occupied cells,
                        // and height of one cell, with both sizes scaled to canvas pixels
                        if (previous_cell_state != empty) {
                            ctx.fillRect(x_start * x_scale, y * y_scale, (x - x_start) * x_scale, y_scale);
                        }
                    }
                    if (cell_state != previous_cell_state) {
                        x_start = x;
                        previous_cell_state = cell_state;
                    }
                }
                if (previous_cell_state != empty) {
                    // At end of each lattice row:
                    // plot a rectangle that's the RLE width of occupied cells,
                    // and height of one cell, with both sizes scaled to canvas pixels
                    ctx.fillRect(x_start * x_scale, y * y_scale, (n_x - x_start) * x_scale, y_scale);
                }
            }
        }
        this.log.pop_reason();
    }
    /** If we're zooming etc, need to reset rough canvas to force redraw */
    reset_rough_background() {
        this.rough_background = null;
    }
    /** Set redraw */
    set_redraw(simulation_controls) {
        this.simulation_controls = simulation_controls;
    }
    /** Redraw */
    redraw() {
        const dim = this.simulation.dim;
        if (dim > 1) {
            this.canvas_2d(this.simulation_controls);
        }
        else {
            this.canvas_1d(this.simulation_controls);
        }
    }
    /** Stop any animation */
    animation_stop() {
        this.set_animation_is_stopped();
        this.anim.stop();
    }
    animation_start(time) {
        this.log.info("animation", "Start");
        if (this.simulation.dim < 2) {
            return;
        }
        this.set_animation_is_playing();
        this.anim.schedule();
    }
    // Need this to have a dual-function pause/or/play button
    get_animation_state() {
        return this.is_playing;
    }
    set_animation_state(is_playing) {
        this.is_playing = is_playing;
    }
    set_animation_is_stopped() {
        this.set_animation_state(false);
    }
    set_animation_is_playing() {
        this.set_animation_state(true);
    }
    get_fps() {
        // console.log("Current fps:", this.frames_per_second);
        return this.frames_per_second;
    }
    set_fps(fps) {
        // console.log("Setting fps:", this.frames_per_second);
        this.frames_per_second = fps;
    }
    // Step backward by one iteration, freezing the playback if need be
    decrement_slice() {
        this.animation_stop();
        const next_slice = this.slice - this.t_increment;
        if (next_slice >= 0 && next_slice <= this.simulation.n_results()) {
            this.slice = next_slice;
        }
        this.redraw();
        html.set_input_value("slice", this.slice);
    }
    // Step forward by one iteration, freezing the playback if need be
    increment_slice() {
        this.animation_stop();
        const next_slice = this.slice + this.t_increment;
        if (next_slice >= 0 && next_slice <= this.simulation.n_results()) {
            this.slice = next_slice;
        }
        this.redraw();
        html.set_input_value("slice", this.slice);
    }
    playback_simulation(fps) {
        if (fps == 0) {
            this.animation_stop();
            return;
        }
        this.slice_delta = 1;
        if (fps < 0) {
            this.slice_delta = -1;
            fps = -fps;
        }
        this.set_fps(fps);
        this.set_animation_is_playing();
        this.anim.restart(0, (time) => this.animation_start(time));
    }
    animation_tick(time) {
        if (this.simulation.dim < 2) {
            this.log.error("animation", "Should not reach here with dim < 2");
            return;
        }
        if (this.slice >= 0 && this.slice <= this.simulation.n_results()) {
            html.set_input_value("slice", this.slice);
            this.redraw();
        }
        const next_slice = this.slice + this.slice_delta;
        if (next_slice > 0 && next_slice <= this.simulation.n_results()) {
            this.slice = next_slice;
            this.anim.schedule_at(time + 1000 / this.get_fps());
        }
        else {
            const total_time = this.anim.duration();
            const n_frames = this.simulation.n_results();
            const fps = (n_frames / total_time) * 1000;
            this.log.info("animation", `Played back @ ${fps} frames per second : ${n_frames} frames / ${total_time}ms`);
        }
    }
    set_zoom(zoom) {
        html.set_input_value("zoom", zoom);
        this.zoom = zoom;
        this.redraw();
    }
    set_slice(slice) {
        this.animation_stop();
        this.slice = slice;
        this.redraw();
    }
}
