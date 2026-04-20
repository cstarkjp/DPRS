export function get_input_float(id: string, min: number, max: number): number {
  const e = document.getElementById(id);
  if (!(e instanceof HTMLInputElement)) {
    return 0;
  }
  var p = Number.parseFloat(e.value);
  if (!(p >= min && p <= max)) {
    p = (min + max) / 2;
  }
  e.value = p.toString();
  return p;
}

export function get_input_int(id: string, min: number, max: number): number {
  const e = document.getElementById(id);
  if (!(e instanceof HTMLInputElement)) {
    return min;
  }
  var p = Number.parseInt(e.value);
  if (!(p >= min && p <= max)) {
    p = min;
  }
  e.value = p.toString();
  return p;
}

export function set_input_value(id: string, value: any): void {
  const e = document.getElementById(id);
  if (e instanceof HTMLInputElement) {
    e.value = value.toString();
  }
}

export function set_input_checked(id: string, checked: boolean): void {
  const e = document.getElementById(id);
  if (e instanceof HTMLInputElement) {
    e.checked = checked;
  }
}

export function set_input_range(id: string, min: any, max: any): void {
  const e = document.getElementById(id);
  if (e instanceof HTMLInputElement) {
    e.min = min.toString();
    e.max = max.toString();
  }
}

export function get_input_checked(id: string): boolean {
  const e = document.getElementById(id);
  if (e instanceof HTMLInputElement) {
    return e.checked;
  } else {
    return false;
  }
}

export function get_input_radio_checked(parent_id: string): null | string {
  const e = document.getElementById(parent_id);
  if (e === null) {
    return null;
  }
  const selected_e = e.querySelector(":checked");
  if (selected_e instanceof HTMLInputElement) {
    return selected_e.value;
  } else {
    return null;
  }
}

interface IdClasses {
  id?: string;
  classes?: string;
}

interface Range {
  min: number;
  max: number;
  value?: number;
  step?: number;
}

export class HtmlElement {
  ele: HTMLElement;
  constructor(ele: HTMLElement) {
    this.ele = ele;
  }

  static new_ele(ele_type: string, classes?: string) {
    const ele = document.createElement(ele_type);
    if (classes) {
      ele.className = classes;
    }
    return new HtmlElement(ele);
  }

  clear() {
    while (this.ele.firstChild) {
      this.ele.removeChild(this.ele.firstChild);
    }
  }

  add_ele(ele_type: string, id: string = "", classes: string = "") {
    const ele = document.createElement(ele_type);
    if (id) {
      ele.setAttribute("id", id);
    }
    if (classes) {
      ele.className = classes;
    }
    this.ele.appendChild(ele);
    return new HtmlElement(ele);
  }

  add_tags(tag_values: Array<[string, string]>) {
    for (const [tag, value] of tag_values) {
      this.ele.setAttribute(tag, value);
    }
    return this;
  }

  static set_id_classes(doc_ele: Element, id_classes: IdClasses): void {
    if (id_classes.id !== undefined) {
      doc_ele.id = id_classes.id;
    }
    if (id_classes.classes !== undefined) {
      doc_ele.className = id_classes.classes;
    }
  }

  add_input_button(
    value: string,
    callback: () => void,
    id_classes: IdClasses = {},
  ) {
    const input = document.createElement("input");
    input.setAttribute("type", "button");
    input.setAttribute("value", value);
    input.onclick = callback;
    HtmlElement.set_id_classes(input, id_classes);
    this.ele.appendChild(input);
    return new HtmlElement(input);
  }

  add_input_checkbox(name: string, id_classes: IdClasses = {}) {
    const input = document.createElement("input");
    input.setAttribute("type", "checkbox");
    input.setAttribute("name", name);
    HtmlElement.set_id_classes(input, id_classes);
    this.ele.appendChild(input);
    return new HtmlElement(input);
  }

  add_input_radio(
    name: string,
    value: string,
    required: boolean,
    id_classes: IdClasses = {},
  ) {
    const input = document.createElement("input");
    input.setAttribute("type", "radio");
    input.setAttribute("name", name);
    input.setAttribute("value", value);
    if (required) {
      input.setAttribute("required", "true");
    }
    HtmlElement.set_id_classes(input, id_classes);
    this.ele.appendChild(input);
    return new HtmlElement(input);
  }

  add_input_range(
    name: string,
    range: Range,
    callback: (event: Event, value: number) => void,
    id_classes: IdClasses = {},
  ) {
    var value = range.min;
    var step = 1;
    if (range.value !== undefined) {
      value = range.value;
    }
    if (range.step !== undefined) {
      step = range.step;
    }
    const input = document.createElement("input");
    input.setAttribute("type", "range");
    input.setAttribute("name", name);
    input.setAttribute("value", value.toString());
    input.setAttribute("min", range.min.toString());
    input.setAttribute("max", range.max.toString());
    input.setAttribute("step", step.toString());
    // const x: HTMLInputElement = new HTMLInputElement();
    // x.on
    input.oninput = (e) => {
      var value;
      if (step == 1) {
        value = Number.parseFloat(input.value);
      } else {
        value = Number.parseFloat(input.value);
      }
      callback(e, value);
    };
    HtmlElement.set_id_classes(input, id_classes);
    this.ele.appendChild(input);
    return new HtmlElement(input);
  }

  add_input_text(name: string, value: string, id_classes: IdClasses = {}) {
    const input = document.createElement("input");
    input.setAttribute("type", "text");
    input.setAttribute("name", name);
    input.setAttribute("value", value);
    HtmlElement.set_id_classes(input, id_classes);
    this.ele.appendChild(input);
    return new HtmlElement(input);
  }

  add_label(for_input?: string, id_classes: IdClasses = {}) {
    const label = document.createElement("label");
    if (for_input) {
      label.setAttribute("for", for_input);
    }
    HtmlElement.set_id_classes(label, id_classes);
    this.ele.appendChild(label);
    return new HtmlElement(label);
  }

  set_content(content: any) {
    //console.log(this.ele);
    if (content instanceof Node) {
      this.ele.appendChild(content);
    } else if (content instanceof HtmlElement) {
      this.ele.appendChild(content.ele);
    } else {
      this.ele.insertAdjacentText("afterbegin", content);
    }
  }

  set_style(style: string, value?: string) {
    /* This is not supported by FireFox
    if (value) {
      this.ele.attributeStyleMap.set(style, value);
    } else {
      this.ele.attributeStyleMap.delete(style);
    }
    */
    if (value) {
      this.ele.style = `${style}: ${value};`;
    } else {
      this.ele.style = "";
    }
  }
}

interface Table {
  classes: string;
  headings: Array<HtmlElement>;
  heading_classes: string;
  body: Array<Array<HtmlElement>>;
}

class Table {
  constructor(classes: string) {
    this.classes = classes;
    this.headings = [];
    this.heading_classes = "";
    this.body = [];
  }

  add_headings(headings: Array<HtmlElement>) {
    for (const h of headings) {
      this.headings.push(h);
    }
  }

  add_body(body_elements: Array<HtmlElement>) {
    this.body.push(body_elements);
  }

  as_html() {
    const table = HtmlElement.new_ele("table", this.classes);

    if (this.headings.length > 0) {
      const tr = table.add_ele("tr", "", this.heading_classes);
      let i = 0;
      for (const h of this.headings) {
        const th = tr.add_ele("th", "th" + i);
        th.set_content(h);
        i += 1;
      }
    }

    for (const c of this.body) {
      const tr = table.add_ele("tr");
      for (const d of c) {
        const td = tr.add_ele("td");
        td.set_content(d);
      }
    }
    return table;
  }
}

function clear(id: Node) {
  while (id.firstChild) {
    id.removeChild(id.firstChild);
  }
}

function add_ele(parent: Node, type: string, classes: string) {
  const ele = document.createElement(type);
  ele.className = classes;
  parent.appendChild(ele);
  return ele;
}

function if_ele_id(ele_id: string, v: any, f: any) {
  const e = document.getElementById(ele_id);
  if (e != null) {
    f(e, v);
  }
}
