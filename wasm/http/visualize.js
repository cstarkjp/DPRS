import * as html from "./html.js";
import { Log } from "./log.js";

export class Visualize {
  constructor(simulation) {
    this.simulation = simulation;
    this.width = 0;
    this.height = 0;
  }

  canvas_simple(div_id, scale, stagger) {
    this.width = this.simulation.dims.n_x * scale;
    this.height = this.simulation.params.n_iterations * scale;
    const div = document.getElementById(div_id);
    if (!div) {
      return;
    }

    const div_html = new html.Element(div);
    div_html.clear();
    this.canvas = div_html.add_ele("canvas", "visualize");
    this.canvas.ele.width = this.width;
    this.canvas.ele.height = this.height;
    const ctx = this.canvas.ele.getContext("2d");

    var x_ofs = 0;
    var x_scale = scale;
    var y_scale = scale;
    if (stagger) {
      y_scale = 0.5 * y_scale;
      x_ofs = 0.5;
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
  }
}
