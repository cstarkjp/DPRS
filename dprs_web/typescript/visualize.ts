import * as html from "./html.js";
import * as log from "./log.js";
import { JsSimulation } from "./js_simulation.js";
import { SimulationControls } from "./simulation_controls.js";
import { Animate } from "./animate.js";

/**
 * A visualization 'div' for simulations
 *
 */

export class Visualize {
  /**
   * Logger to report progress to (as a source of 'sim')
   */
  log: log.Logger;

  /**
   *
   * The underlying simulation
   *
   */
  simulation: JsSimulation;

  /** The simulation controls in use for this visualization
   *
   * This may be changed if a simulation of a different dimension is run
   */
  simulation_controls: null | SimulationControls = null;

  /** Animation class to allow for animated visualization */
  anim: Animate;

  /** Div element for the visualize
   *
   */
  div: html.HtmlElement;

  /** Width of the required canvs
   *
   */
  width: number = 0;

  /** Height of the required canvs
   *
   */
  height: number = 0;

  /** Zoom scale to use
   *
   */
  scale: number = 1;

  /** Which 'time slice' to use for 2D
   *
   */
  slice: number;

  /** Which direction to animate 'time slice' when animating
   *
   */
  slice_delta: number = 1;

  /** Target frames per second of animation */
  frames_per_second: number = 25;

  /**
   *
   * Create a new Visualize for a simulation
   *
   * This does not populate it
   *
   */
  constructor(logger: log.Log, simulation: JsSimulation, div_id: string) {
    this.log = new log.Logger(logger, "viz");
    this.simulation = simulation;
    this.anim = new Animate((time) => this.animation_tick(time));
    this.slice = 0;

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
  canvas_1d(sim_control: any): void {
    this.log.push_reason("canvas_1d");

    const stagger = this.simulation.results_are_staggered();
    var x_ofs = 0;
    var x_scale = this.scale;
    var y_scale = this.scale;
    if (stagger) {
      y_scale = 0.5 * y_scale;
      x_ofs = 0.5;
    }

    this.width = this.simulation.parameters.dims.n_x * x_scale;
    this.height = this.simulation.n_results() * y_scale;

    this.log.info(
      `Created canvas size ${this.width} x ${this.height} with stagger ${stagger} and scale ${x_scale}x${y_scale}`,
    );

    this.div.clear();
    const canvas = this.div.add_ele("canvas", "", "visualize canvas_1d");
    const canvas_ele = canvas.ele as HTMLCanvasElement;
    canvas_ele.width = this.width;
    canvas_ele.height = this.height;
    const ctx = canvas_ele.getContext("2d");
    if (ctx === null) {
      this.log.error(
        "Failed to get 2D context from HTML Canvas element for the visualizer",
      );
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
        } else {
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

  /**
   *
   * Create a simple 2D canvas for a 1D simulation (in the X) with time increasing in the Y
   *
   */
  canvas_2d(sim_control: any): void {
    this.log.push_reason("canvas_2d");

    var x_scale = this.scale;
    var y_scale = this.scale;

    this.width = this.simulation.parameters.dims.n_x * x_scale;
    this.height = this.simulation.parameters.dims.n_y * y_scale;

    /*
    this.log.info(
      `Created canvas size ${this.width} x ${this.height} with scale ${x_scale}x${y_scale}`,
    );
    */

    this.div.clear();
    const canvas = this.div.add_ele("canvas", "", "visualize canvas_2d");
    const canvas_ele = canvas.ele as HTMLCanvasElement;
    canvas_ele.width = this.width;
    canvas_ele.height = this.height;
    const ctx = canvas_ele.getContext("2d");
    if (ctx === null) {
      this.log.error(
        "Failed to get 2D context from HTML Canvas element for the visualizer",
      );
      this.log.pop_reason();
      return;
    }

    ctx.fillStyle = "lightgrey";
    ctx.fillRect(0, 0, this.width, this.height);
    ctx.fillStyle = "purple";

    let data = this.simulation.result(this.slice);
    if (!data) {
      this.log.info(`No data in slice ${this.slice}`);
    } else {
      var n = 0;
      for (let y = 0; y < this.simulation.parameters.dims.n_y; y++) {
        var last = null;
        var x_start = null;
        for (let x = 0; x < this.simulation.parameters.dims.n_x; x += 1) {
          const d = data[n];
          if (last !== null && d != last) {
            if (last != 0) {
              ctx.fillRect(
                x_start! * x_scale,
                y * y_scale,
                (x - x_start!) * x_scale,
                y_scale,
              );
            }
          }
          if (d != last) {
            x_start = x;
            last = d;
          }
          n = n + 1;
        }
        if (last != 0) {
          ctx.fillRect(
            x_start! * x_scale,
            y * y_scale,
            (this.simulation.parameters.dims.n_x - x_start!) * x_scale,
            y_scale,
          );
        }
      }
    }
    /*
    this.log.info("Completed canvas");
    */
    this.log.pop_reason();
  }

  /** Set redraw */
  set_redraw(simulation_controls: SimulationControls): void {
    this.simulation_controls = simulation_controls;
  }

  /** Redraw */
  redraw(): void {
    const dim = this.simulation.dim;
    if (dim > 1) {
      this.canvas_2d(this.simulation_controls);
    } else {
      this.canvas_1d(this.simulation_controls);
    }
  }

  /** Stop any animation
   *
   */
  stop_animation(): void {
    this.anim.stop();
  }

  set_zoom(zoom: number): void {
    this.scale = zoom;
    this.redraw();
  }

  set_slice(slice: number): void {
    this.stop_animation();
    this.slice = slice;
    this.redraw();
  }

  playback_simulation(fps: number): void {
    if (fps == 0) {
      this.anim.stop();
      return;
    }
    this.slice_delta = 1;
    if (fps < 0) {
      this.slice_delta = -1;
      fps = -fps;
    }
    this.frames_per_second = fps;
    console.log("Set fps to", this.frames_per_second);
    this.anim.restart(0, (time) => this.animation_start(time));
  }

  animation_start(time: number): void {
    this.log.info("animation", "Start");

    if (this.simulation.dim < 2) {
      return;
    }
    this.anim.schedule();
  }

  animation_tick(time: number): void {
    if (this.simulation.dim < 2) {
      this.log.error("animation", "Should not reach here with dim < 2");
      return;
    }

    if (this.slice >= 0 && this.slice < this.simulation.n_results()) {
      html.set_input_value("slice", this.slice);
      this.redraw();
    }

    var next_slice = this.slice + this.slice_delta;
    if (next_slice > 0 && next_slice < this.simulation.n_results()) {
      this.slice = next_slice;
      this.anim.schedule_at(time + 1000 / this.frames_per_second);
    } else {
      const total_time = this.anim.duration();
      const n_frames = this.simulation.n_results();
      const fps = (n_frames / total_time) * 1000;
      this.log.info(
        "animation",
        `Played back @ ${fps} frames per second : ${n_frames} frames / ${total_time}ms`,
      );
    }
  }
}
