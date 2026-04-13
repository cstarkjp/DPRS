/**
 * A 'tab' in a tabbed page
 */
class Tab {
    /**
     * Create a new Tab given its parent, 'li' element and tab number
     *
     * The 'li' has at least one 'a' element in it
     */
    constructor(tabs, li, num) {
        this.tabs = tabs;
        this.li = li;
        this.num = num;
        var errored = undefined;
        var div_id = null;
        var div = null;
        const a = li.getElementsByTagName("a")[0];
        if (a === undefined) {
            errored = "failed to find 'a' element in Tab - bug in Tabs";
        }
        if (!errored) {
            div_id = a.getAttribute("href");
            if (div_id === null) {
                errored = `tab ${num} 'a' item did not have an 'href' attribute`;
            }
            else {
                div = document.querySelector(div_id);
                if (div === null || !(div instanceof HTMLDivElement)) {
                    errored = `tab ${num} has an href of '{$div_id}' but the relevant div could not be found in the document`;
                }
            }
        }
        if (errored) {
            throw new Error(`Failed to make Tab: ${errored}`);
        }
        this.div_id = div_id;
        this.div = div;
        a.onclick = (e) => {
            this.tabs.hash_change(this.div_id);
            e.preventDefault();
        };
    }
    /**
     * Return the 'id' of this tab - currently this is the 'div_id' property
     */
    id() {
        return this.div_id;
    }
    /**
     * Return true if the 'hash' matches the element for this tab (i.e. thing,html#banana has 'banana' match this tab)
     *
     * This therefore compares the 'hash' with the div_id
     *
     */
    has_hash(hash) {
        return this.div_id == hash;
    }
    /**
     * Set the 'hidden' style for the div associated with this tab
     */
    set_hidden(hidden) {
        this.div.hidden = hidden;
        hidden = hidden;
        if (hidden) {
            this.li.className = "";
        }
        else {
            this.li.className = "active";
        }
    }
}
/**
 * A class that handles a set of Tabs, only one of which should be selected, and that will become 'unhidden' while the others are 'hidden'
 */
export class Tabs {
    /**
     * Create a new set of tabs whose tab list can be selected with 'container_select'
     *
     * This tab list must be an element that contains a 'ul' element, which in
     * turn has 'li' for each tab, with each 'li' having an 'a' with an 'href'
     * identifying the tab it is associated with.
     */
    constructor(container_select, callback) {
        this.tabs = [];
        this.callback = callback;
        var errored = undefined;
        const tab_list = document.querySelector(container_select);
        if (tab_list === null) {
            errored = `tab list ${container_select} could not be found`;
        }
        var ul = undefined;
        if (!errored) {
            ul = tab_list.getElementsByTagName("ul")[0];
            if (!(ul instanceof HTMLUListElement)) {
                errored =
                    "tab list must contain an 'ul' element but one could not be found";
            }
        }
        if (!errored) {
            var i = 0;
            for (const li of ul.getElementsByTagName("li")) {
                if (li.getElementsByTagName("a").length != 0) {
                    this.tabs.push(new Tab(this, li, i));
                }
                i += 1;
            }
            if (this.tabs.length == 0) {
                errored =
                    "no tabs ('li' items with child that is an 'a' item) found in the 'ul' tab list element";
            }
        }
        if (errored) {
            throw new Error(`Failed to make 'Tabs': ${errored}`);
        }
        this.selected_tab_number = undefined;
        this.post_init();
    }
    /**
     * After the contents are initialized, actually make the document changes happen
     *
     * This tab list must be an element that contains a 'ul' element, which in
     * turn has 'li' for each tab, with each 'li' having an 'a' with an 'href'
     * identifying the tab it is associated with.
     */
    post_init() {
        const me = this;
        window.addEventListener("hashchange", () => {
            me.hash_change(location.hash);
        });
        if (this.hash_change(location.hash) === undefined) {
            this.select_tab(0);
        }
    }
    /// Invoked when an <a href='#...'> link is selected
    hash_change(hash_name) {
        for (const t of this.tabs) {
            if (t.has_hash(hash_name)) {
                return this.select_tab(t.num);
            }
        }
        return undefined;
    }
    select_tab(tab_number) {
        if (tab_number >= this.tabs.length) {
            tab_number = 0;
        }
        if (tab_number == this.selected_tab_number) {
            return;
        }
        for (const t of this.tabs) {
            t.set_hidden(t.num != tab_number);
        }
        this.selected_tab_number = tab_number;
        this.callback(this.tabs[this.selected_tab_number].id());
        return this.selected_tab_number;
    }
}
