import * as html from "./html.js";
import * as log from "./log.js";
import { JsSimulation } from "./js_simulation.js";

/**
 * A visualization 'div' for simulations
 *
 * At present this can only display 1D simulations
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

  /**
   *
   * Div element for the visualize
   *
   */
  div: html.HtmlElement;

  /**
   *
   * width of the required canvs
   *
   */
  width: number;

  /**
   *
   * height of the required canvs
   *
   */
  height: number;

  /**
   *
   * Zoom scale to use
   *
   */
  scale: number;

  /**
   *
   * Which 'time slice' to use for 2D
   *
   */
  slice: number;

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
    this.width = 0;
    this.height = 0;
    this.scale = 1;
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

    this.log.info(
      `Created canvas size ${this.width} x ${this.height} with scale ${x_scale}x${y_scale}`,
    );

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

    let data = this.simulation.result(this.slice);
    if (!data) {
      this.log.info(`No data in slice ${this.slice}`);
    } else {
      for (let y = 0; y < this.simulation.parameters.dims.n_y; y++) {
        for (let x = 0; x < this.simulation.parameters.dims.n_x; x += 1) {
          const n = y * this.simulation.parameters.dims.n_x + x;
          if (data[n] != 0) {
            ctx.fillStyle = "purple";
          } else {
            ctx.fillStyle = "lightgrey";
          }
          ctx.fillRect(x * x_scale, y * y_scale, x_scale, y_scale);
        }
      }
    }
    this.log.info("Completed canvas");
    this.log.pop_reason();
  }
}
