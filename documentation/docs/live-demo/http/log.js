import * as html from "./html.js";
//a Log
export class Logger {
  constructor(log, src) {
    this.log = log;
    this.src = src;
    this.reason_stack = [];
  }
  push_reason(reason) {
    this.reason_stack.push(reason);
  }
  pop_reason(reason) {
    this.reason_stack.pop();
  }
  info(reason, message) {
    if (!message) {
      message = reason;
      reason = this.reason_stack.at(-1);
    }
    this.log.add_log("info", this.src, reason, message);
  }
  warning(reason, message) {
    if (!message) {
      message = reason;
      reason = this.reason_stack.at(-1);
    }
    this.log.add_log("warning", this.src, reason, message);
  }
  error(reason, message) {
    if (!message) {
      message = reason;
      reason = this.reason_stack.at(-1);
    }
    this.log.add_log("error", this.src, reason, message);
  }
  fatal(reason, message) {
    if (!message) {
      message = reason;
      reason = this.reason_stack.at(-1);
    }
    this.log.add_log("fatal error", this.src, reason, message);
  }
}
export class Log {
  //fp constructor
  constructor(div_id) {
    this.log = [];
    const div = document.getElementById(div_id);
    this.div = null;

    if (div) {
      this.div = new html.Element(div);
    }
  }

  //mp reset_log
  reset_log() {
    this.log = [];
    this.fill_div();
  }

  //ap is_empty
  is_empty() {
    return this.log.length == 0;
  }

  //ap log
  log() {
    return this.log;
  }

  //mp add_log
  add_log(severity, src, reason, error) {
    this.log.push({
      severity: severity,
      src: src,
      reason: reason,
      error: error,
    });
    this.fill_div();
  }

  //mp fill_div
  fill_div() {
    if (!this.div) {
      return;
    }
    this.div.clear();
    const table = this.div.add_ele("table");
    for (const e of this.log) {
      const tr = table.add_ele("tr");
      tr.add_ele("th").set_content(e.severity);
      tr.add_ele("td").set_content(e.src);
      tr.add_ele("td").set_content(e.reason);
      tr.add_ele("td").set_content(e.error);
    }
  }
}
