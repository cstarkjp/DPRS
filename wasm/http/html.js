export { HtmlElement };
class HtmlElement {
    constructor(ele) {
        this.ele = ele;
    }
    static new_ele(ele_type, classes) {
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
    add_ele(ele_type, classes) {
        const ele = document.createElement(ele_type);
        if (classes) {
            ele.className = classes;
        }
        this.ele.appendChild(ele);
        return new HtmlElement(ele);
    }
    add_tags(tag_values) {
        tag_values.forEach((value, tag) => {
            this.ele.setAttribute(tag, value);
        });
        return this;
    }
    add_tags_old(tag_values) {
        for (const [tag, value] of Object.entries(tag_values)) {
            this.ele.setAttribute(tag, value);
        }
        return this;
    }
    set_content(content) {
        //console.log(this.ele);
        if (content instanceof Node) {
            this.ele.appendChild(content);
        }
        else if (content instanceof HtmlElement) {
            this.ele.appendChild(content.ele);
        }
        else {
            this.ele.insertAdjacentText("afterbegin", content);
        }
    }
}
class Table {
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
function clear(id) {
    while (id.firstChild) {
        id.removeChild(id.firstChild);
    }
}
function add_ele(parent, type, classes) {
    const ele = document.createElement(type);
    ele.className = classes;
    parent.appendChild(ele);
    return ele;
}
function if_ele_id(ele_id, v, f) {
    const e = document.getElementById(ele_id);
    if (e != null) {
        f(e, v);
    }
}
//# sourceMappingURL=html.js.map