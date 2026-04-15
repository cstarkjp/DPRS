import * as html from "./html.js";

export class Logger {
  log: Log;
  src: string;
  reason_stack: Array<string>;

  constructor(log: Log, src: string) {
    this.log = log;
    this.src = src;
    this.reason_stack = [];
  }

  push_reason(reason: string) {
    this.reason_stack.push(reason);
  }

  pop_reason() {
    this.reason_stack.pop();
  }

  reason_top() {
    const x = this.reason_stack[this.reason_stack.length - 1];
    if (x == undefined) {
      return "";
    } else {
      return x;
    }
  }

  info(reason: string, message?: string) {
    if (!message) {
      message = reason;
      reason = this.reason_top();
    }
    this.log.add_log("info", this.src, reason, message);
  }
  warning(reason: string, message?: string) {
    if (!message) {
      message = reason;
      reason = this.reason_top();
    }
    this.log.add_log("warning", this.src, reason, message);
  }
  error(reason: string, message?: string) {
    if (!message) {
      message = reason;
      reason = this.reason_top();
    }
    this.log.add_log("error", this.src, reason, message);
  }
  fatal(reason: string, message?: string) {
    if (!message) {
      message = reason;
      reason = this.reason_top();
    }
    this.log.add_log("fatal error", this.src, reason, message);
  }
}

class LogEntry {
  severity: string;
  src: string;
  reason: string;
  error: string;

  constructor(severity: string, src: string, reason: string, error: string) {
    this.severity = severity;
    this.src = src;
    this.reason = reason;
    this.error = error;
  }
}

export class Log {
  log: Array<LogEntry>;
  div: html.HtmlElement;

  constructor(div_id: string) {
    this.log = [];
    const div = document.getElementById(div_id);

    if (div) {
      this.div = new html.HtmlElement(div);
    } else {
      throw new Error("Div id not found for Log");
    }
  }

  reset_log() {
    this.log = [];
    this.fill_div();
  }

  is_empty() {
    return this.log.length == 0;
  }

  add_log(severity: string, src: string, reason: string, error: string) {
    this.log.push(new LogEntry(severity, src, reason, error));
    this.fill_div();
  }

  fill_div() {
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
