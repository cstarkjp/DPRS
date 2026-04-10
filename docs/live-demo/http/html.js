export class Element {
  constructor(ele) {
    this.ele = ele;
  }

  static new_ele(ele_type, classes) {
    const ele = document.createElement(ele_type);
    ele.className = classes;
    return new Element(ele);
  }

  clear() {
    while (this.ele.firstChild) {
      this.ele.removeChild(this.ele.firstChild);
    }
  }

  add_ele(ele_type, classes) {
    const ele = document.createElement(ele_type);
    if (classes) {
      ele.className = classes;
    }
    this.ele.appendChild(ele);
    return new Element(ele);
  }

  add_tags(tag_values) {
    for (const [tag, value] of Object.entries(tag_values)) {
      this.ele[tag] = value;
    }
    return this;
  }

  set_content(content) {
    //console.log(this.ele);
    if (content instanceof Node) {
      this.ele.appendChild(content);
    } else if (content instanceof Element) {
      this.ele.appendChild(content.ele);
    } else {
      this.ele.innerText = content;
    }
  }
}

export class Table {
  constructor(classes) {
    this.classes = classes;
    this.headings = [];
    this.heading_classes = "";
    this.body = [];
  }

  add_headings(headings) {
    for (const h of headings) {
      this.headings.push(h);
    }
  }

  add_body(body_elements) {
    this.body.push(body_elements);
  }

  as_html() {
    const table = Element.new_ele("table", this.classes);

    if (this.headings != []) {
      const tr = table.add_ele("tr", this.heading_classes);
      let i = 0;
      for (const h of this.headings) {
        const th = tr.add_ele("th", "th" + i);
        th.set_content(h);
        i += 1;
      }
    }

    for (const c of this.body) {
      const tr = table.add_ele("tr", "");
      for (const d of c) {
        const td = tr.add_ele("td", "");
        td.set_content(d);
      }
    }
    return table;
  }
}

export function clear(id) {
  while (id.firstChild) {
    id.removeChild(id.firstChild);
  }
}

//mp add_ele
export function add_ele(parent, type, classes) {
  const ele = document.createElement(type);
  ele.className = classes;
  parent.append(ele);
  return ele;
}

//mp if_ele_id
export function if_ele_id(ele_id, v, f) {
  const e = document.getElementById(ele_id);
  if (e != null) {
    f(e, v);
  }
}

//mp table
export function table(table_classes, headings, contents) {
  const table = document.createElement("table");
  table.className = "browser_table " + table_classes[0];
  var tr;

  if (headings) {
    tr = document.createElement("tr");
    if (table_classes[1]) {
      tr.className = table_classes[1];
    }
    let i = 0;
    for (const h of headings) {
      const th = document.createElement("th");
      th.innerText = h;
      th.className = "th" + i;
      i += 1;
      tr.appendChild(th);
    }
    table.appendChild(tr);
  }

  for (const c of contents) {
    tr = document.createElement("tr");
    for (const d of c) {
      const td = document.createElement("td");
      // console.log(typeof d);
      if (typeof d === "string") {
        td.innerHTML = d;
      } else {
        td.appendChild(d);
      }
      tr.appendChild(td);
    }
    table.appendChild(tr);
  }
  return table;
}

//mp vtable
export function vtable(table_classes, contents) {
  const table = document.createElement("table");
  table.className = "browser_table " + table_classes;
  var tr;

  for (const c of contents) {
    tr = document.createElement("tr");
    let td_or_th = "th";
    for (const d of c) {
      const td = document.createElement(td_or_th);
      td.innerHTML = d;
      tr.appendChild(td);
      td_or_th = "td";
    }
    table.appendChild(tr);
  }
  return table;
}
