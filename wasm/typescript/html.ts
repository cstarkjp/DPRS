export { HtmlElement };

interface HtmlElement {
  ele: Element;
}
class HtmlElement {
  constructor(ele: Element) {
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

  add_ele(ele_type: string, classes?: string) {
    const ele = document.createElement(ele_type);
    if (classes) {
      ele.className = classes;
    }
    this.ele.appendChild(ele);
    return new HtmlElement(ele);
  }

  add_tags(tag_values: Map<string, string>) {
    tag_values.forEach((value, tag) => {
      this.ele.setAttribute(tag, value);
    });
    return this;
  }
  add_tags_old(tag_values: any) {
    for (const [tag, value] of Object.entries(tag_values)) {
      this.ele.setAttribute(tag, value as string);
    }
    return this;
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
