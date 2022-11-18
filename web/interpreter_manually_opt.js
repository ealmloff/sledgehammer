let op, len, ns, attr, i, j, value, element, pos, char, numAttributes, endRounded, inptr, metadata, parent, numNodes, node, id, nodes;

export function work_last_created() {
    inptr.Work();
}

export function update_last_memory(mem) {
    inptr.UpdateMemory(mem);
}

function exOp() {
    // first bool: op & 0x20
    // second bool: op & 0x40

    switch (op & 0x1F) {
        // first child
        case 0:
            inptr.l = inptr.l.firstChild;
            break;
        // next sibling
        case 1:
            inptr.l = inptr.l.nextSibling;
            break;
        // parent
        case 2:
            inptr.l = inptr.l.parentNode;
            break;
        // store with id
        case 3:
            inptr.n[inptr.v.u32(inptr.u, true)] = inptr.l;
            inptr.u += 4;
            break;
        // set last node
        case 4:
            inptr.l = inptr.n[inptr.v.u32(inptr.u, true)];
            inptr.u += 4;
            break;
        // stop
        case 5:
            return true;
        // create full element
        case 6:
            inptr.l = inptr.createFullElement();
            break;
        // append children
        case 7:
            // the first bool is encoded as op & (1 << 5)
            if (op & 0x20) {
                parent = inptr.n[inptr.v.u32(inptr.u, true)];
                inptr.u += 4;
            }
            else {
                parent = inptr.l;
            }
            // the second bool is encoded as op & (1 << 6)
            if (op & 0x40) {
                parent.appendChild(inptr.n[inptr.v.u32(inptr.u, true)]);
                inptr.u += 4;
            }
            else {
                parent.appendChild(inptr.l);
            }
            break;
        // replace with
        case 8:
            // the second bool is encoded as op & (1 << 6)
            if (op & 0x40) {
                parent = inptr.n[inptr.v.u32(inptr.u, true)];
                inptr.u += 4;
            }
            else {
                parent = inptr.l;
            }
            // the first bool is encoded as op & (1 << 5)
            if (op & 0x20) {
                numNodes = inptr.v.u8(inptr.u++, true);
                nodes = [];
                for (i = 0; i < numNodes; i++) {
                    if (inptr.v.u8(inptr.u++, true)) {
                        nodes.push(inptr.n[inptr.v.u32(inptr.u, true)]);
                        inptr.u += 4;
                    }
                    else {
                        nodes.push(inptr.l);
                    }
                }
                parent.replaceWith(...n);
            }
            else {
                // the third bool is encoded as op & (1 << 7)
                if (op & 0x80) {
                    parent.replaceWith(inptr.n[inptr.v.u32(inptr.u, true)]);
                    inptr.u += 4;
                }
                else {
                    parent.replaceWith(inptr.l);
                }
            }
            break;
        // insert after
        case 9:
            // the second bool is encoded as op & (1 << 6)
            if (op & 0x40) {
                parent = inptr.n[inptr.v.u32(inptr.u, true)];
                inptr.u += 4;
            }
            else {
                parent = inptr.l;
            }
            // the first bool is encoded as op & (1 << 5)
            if (op & 0x20) {
                numNodes = inptr.v.u8(inptr.u++, true);
                nodes = [];
                for (i = 0; i < numNodes; i++) {
                    if (inptr.v.u8(inptr.u++, true)) {
                        nodes.push(inptr.n[inptr.v.u32(inptr.u, true)]);
                        inptr.u += 4;
                    }
                    else {
                        nodes.push(inptr.l);
                    }
                }
                parent.after(...n);
            } else {
                // the third bool is encoded as op & (1 << 7)
                if (op & 0x80) {
                    parent.after(inptr.n[inptr.v.u32(inptr.u, true)]);
                    inptr.u += 4;
                }
                else {
                    parent.after(inptr.l);
                }
            }
            break;
        // insert before
        case 10:
            // the second bool is encoded as op & (1 << 6)
            if (op & 0x40) {
                parent = inptr.n[inptr.v.u32(inptr.u, true)];
                inptr.u += 4;
            }
            else {
                parent = inptr.l;
            }
            // the first bool is encoded as op & (1 << 5)
            if (op & 0x20) {
                numNodes = inptr.v.u8(inptr.u++, true);
                nodes = [];
                for (i = 0; i < numNodes; i++) {
                    if (inptr.v.u8(inptr.u++, true)) {
                        nodes.push(inptr.n[inptr.v.u32(inptr.u, true)]);
                        inptr.u += 4;
                    }
                    else {
                        nodes.push(inptr.l);
                    }
                }
                parent.before(...n);
            } else {
                // the third bool is encoded as op & (1 << 7)
                if (op & 0x80) {
                    parent.before(inptr.n[inptr.v.u32(inptr.u, true)]);
                    inptr.u += 4;
                }
                else {
                    parent.before(inptr.l);
                }
            }
            break;
        // remove
        case 11:
            // the first bool is encoded as op & (1 << 5)
            if (op & 0x20) {
                inptr.n[inptr.v.u32(inptr.u, true)].remove();
                inptr.u += 4;
            }
            else {
                inptr.l.remove();
            }
            break;
        // create text node
        case 12:
            inptr.l = document.createTextNode(inptr.s.substring(inptr.o, inptr.o += inptr.v.u16(inptr.u, true)));
            inptr.u += 2;
            // the first bool is encoded as op & (1 << 5)
            if (op & 0x20) {
                inptr.n[inptr.v.u32(inptr.u, true)] = inptr.l;
                inptr.u += 4;
            }
            break;
        // create element
        case 13:
            inptr.l = inptr.createElement();
            // the second bool is encoded as op & (1 << 6)
            if (op & 0x20) {
                inptr.n[inptr.v.u32(inptr.u, true)] = inptr.l;
                inptr.u += 4;
            }
            break;
        // set text
        case 14:
            // the first bool is encoded as op & (1 << 5)
            if (op & 0x20) {
                id = inptr.v.u32(inptr.u, true);
                inptr.u += 4;
                inptr.n[id].textContent = inptr.s.substring(inptr.o, inptr.o += inptr.v.u16(inptr.u, true));
                inptr.u += 2;
            }
            else {
                inptr.l.textContent = inptr.s.substring(inptr.o, inptr.o += inptr.v.u16(inptr.u, true));
                inptr.u += 2;
            }
            break;
        // set attribute
        case 15:
            // the first bool is encoded as op & (1 << 5)
            if (op & 0x20) {
                node = inptr.n[inptr.v.u32(inptr.u, true)];
                inptr.u += 4;
            }
            else {
                node = inptr.l;
            }
            // the second bool is encoded as op & (1 << 6)
            // first bool encodes if the attribute is a string
            if (op & 0x40) {
                // the first two lengths
                i = inptr.v.u32(inptr.u, true);
                inptr.u += 4;
                attr = inptr.s.substring(inptr.o, inptr.o += i & 0xFFFF);
                // the third bool is encoded as op & (1 << 7)
                // second bool encodes if the attribute has a namespace
                if (op & 0x80) {
                    node.setAttributeNS(inptr.s.substring(inptr.o, inptr.o += (i & 0xFFFF0000) >>> 16), attr, inptr.s.substring(inptr.o, inptr.o += inptr.v.u16(inptr.u, true)));
                    inptr.u += 2;
                }
                else {
                    node.setAttribute(attr, inptr.s.substring(inptr.o, inptr.o += (i & 0xFFFF0000) >>> 16));
                }
            } else {
                // the first length and attribute id or the attribute id and the first length
                i = inptr.v.u32(inptr.u, true);
                // we only read 3 bytes out of the 4
                inptr.u += 3;
                // the third bool is encoded as op & (1 << 7)
                // second bool encodes if the attribute has a namespace
                if (op & 0x80) {
                    ns = inptr.s.substring(inptr.o, inptr.o += i & 0xFFFF);
                    node.setAttributeNS(ns, attrs[(i & 0xFF0000) >>> 16], inptr.s.substring(inptr.o, inptr.o += inptr.v.u16(inptr.u, true)));
                    inptr.u += 2;
                }
                else {
                    node.setAttribute(attrs[i & 0xFF], inptr.s.substring(inptr.o, inptr.o += (i & 0xFFFF00) >>> 8));
                }
            }
            break;
        // remove attribute
        case 16:
            // the first bool is encoded as op & (1 << 5)
            if (op & 0x20) {
                node = inptr.n[inptr.v.u32(inptr.u, true)];
                inptr.u += 4;
            }
            else {
                node = inptr.l;
            }
            // the second bool is encoded as op & (1 << 6)
            // second bool encodes if the attribute is a string
            if (op & 0x40) {
                // the third bool is encoded as op & (1 << 7)
                // second bool encodes if the attribute has a namespace
                if (op & 0x80) {
                    i = inptr.v.u32(inptr.u, true);
                    inptr.u += 4;
                    attr = inptr.s.substring(inptr.o, inptr.o += i & 0xFFFF);
                    node.removeAttributeNS(inptr.s.substring(inptr.o, inptr.o += (i & 0xFFFF0000) >>> 16), attr);
                } else {
                    node.removeAttribute(inptr.s.substring(inptr.o, inptr.o += inptr.v.u16(inptr.u, true)));
                    inptr.u += 2;
                }
            } else {
                // the third bool is encoded as op & (1 << 7)
                // second bool encodes if the attribute has a namespace
                if (op & 0x80) {
                    i = inptr.v.u32(inptr.u, true);
                    // we only read 3 bytes out of the 4
                    inptr.u += 3;
                    attr = attrs[i & 0xFF];
                    node.removeAttributeNS(inptr.s.substring(inptr.o, inptr.o += (i & 0xFFFF00) >>> 8), attr);
                }
                else {
                    node.removeAttribute(attrs[inptr.v.u8(inptr.u++)]);
                }
            }
            break;
        // set style
        case 17:
            // the first bool is encoded as op & (1 << 5)
            if (op & 0x20) {
                node = inptr.n[inptr.v.u32(inptr.u, true)];
                inptr.u += 4;
            }
            else {
                node = inptr.l;
            }
            i = inptr.v.u32(inptr.u, true);
            inptr.u += 4;
            node.style.setProperty(inptr.s.substring(inptr.o, inptr.o += i & 0xFFFF), inptr.s.substring(inptr.o, inptr.o += (i & 0xFFFF0000) >>> 16));
            break;
        // remove style
        case 18:
            // the first bool is encoded as op & (1 << 5)
            if (op & 0x20) {
                node = inptr.n[inptr.v.u32(inptr.u, true)];
                inptr.u += 4;
            }
            else {
                node = inptr.l;
            }
            node.style.removeProperty(inptr.s.substring(inptr.o, inptr.o += inptr.v.u16(inptr.u, true)));
            inptr.u += 2;
            break;
        // clone node
        case 19:
            // the first bool is encoded as op & (1 << 5)
            if (op & 0x20) {
                inptr.l = inptr.n[inptr.v.u32(inptr.u, true)].cloneNode(true);
                inptr.u += 4;
            }
            else {
                inptr.l = inptr.l.cloneNode(true);
            }
            // the second bool is encoded as op & (1 << 6)
            if (op & 0x40) {
                inptr.n[inptr.v.u32(inptr.u, true)] = inptr.l;
                inptr.u += 4;
            }
            break;
        default:
            break;
    }
}

export class JsInterpreter {
    constructor(mem, _metadata_ptr, _ptr_ptr, _str_ptr_ptr, _str_len_ptr) {
        this.l;
        this.n = [];
        this.p = [];
        this.UpdateMemory(mem);
        this.lp;
        this.ls;
        this.m = _metadata_ptr;
        this.pt = _ptr_ptr;
        this.sp = _str_ptr_ptr;
        this.sl = _str_len_ptr;
        this.s = "";
        this.o = 0;
        this.d = new TextDecoder();
        this.i = 1;
        inptr = this;
    }

    NeedsMemory() {
        return this.v.buffer.byteLength === 0;
    }

    UpdateMemory(mem) {
        this.v = new DataView(mem.buffer);
        this.v.u32 = this.v.getUint32;
        this.v.u16 = this.v.getUint16;
        this.v.u8 = this.v.getUint8;
    }

    Work() {
        metadata = this.v.u8(this.m);
        if (metadata & 0x01) {
            this.lp = this.v.u32(this.pt, true);
        }
        this.u = this.lp;
        if (metadata & 0x04) {
            len = this.v.u32(this.sl, true);
            if (metadata & 0x02) {
                this.ls = this.v.u32(this.sp, true);
            }
            // for small strings decoding them in javascript to avoid the overhead of native calls is faster
            // the fourth boolean contains information about whether the string is all ascii or utf8 and small
            if (metadata & 0x08) {
                pos = this.ls;
                this.s = "";
                endRounded = pos + ((len / 4) | 0) * 4;
                while (pos < endRounded) {
                    char = this.v.u32(pos);
                    this.s += String.fromCharCode(char >> 24, (char & 0x00FF0000) >> 16, (char & 0x0000FF00) >> 8, (char & 0x000000FF));
                    pos += 4;
                }
                switch (this.ls + len - pos) {
                    case 3:
                        char = this.v.u32(pos);
                        this.s += String.fromCharCode(char >> 24, (char & 0x00FF0000) >> 16, (char & 0x0000FF00) >> 8);
                        break;
                    case 2:
                        char = this.v.u16(pos);
                        this.s += String.fromCharCode(char >> 8, char & 0xFF);
                        break;
                    case 1:
                        this.s += String.fromCharCode(this.v.u8(pos));
                        break;
                    case 0:
                        break;
                }
            }
            else {
                this.s = this.d.decode(new DataView(this.v.buffer, this.ls, len));
            }
            this.o = 0;
        }

        // this is faster than a while(true) loop
        for (; ;) {
            // op = this.v.u8(this.u++);
            // if (this.exOp(op & 0x1F)) return;
            op = this.v.u32(this.u, true);
            this.u += 4;
            if (exOp()) return;
            op >>>= 8;
            if (exOp()) return;
            op >>>= 8;
            if (exOp()) return;
            op >>>= 8;
            if (exOp()) return;
        }
    }

    createElement() {
        j = this.v.u32(this.u, true);
        element = j & 0xFF;
        switch (element) {
            case 255:
                // the element is encoded as an enum and the namespace is encoded as a string
                // we use all 4 bytes of i just read
                this.u += 4;
                element = document.createElement(els[(j & 0xFF00) >>> 8], this.s.substring(this.o, this.o += (j & 0xFFFF0000) >>> 16));
                return element;
            case 254:
                // the element is encoded as a string
                // we use 3 bytes of i just read
                this.u += 3;
                element = document.createElement(this.s.substring(this.o, this.o += (j & 0xFFFF00) >>> 8));
                return element;
            case 253:
                // the element and namespace are encoded as strings
                // we use 3 bytes of i just read
                this.u += 3;
                element = this.s.substring(this.o, this.o += (j & 0xFFFF00) >>> 8);
                element = document.createElementNS(this.s.substring(this.o, this.o += this.v.u16(this.u, true)), element);
                this.u += 2;
                return element;
            default:
                this.u++;
                // the element is encoded as an enum
                return document.createElement(els[element]);
        }
    }

    createFullElement() {
        let parent_id;
        j = this.v.u8(this.u++);
        if (j & 0x1) {
            parent_id = this.v.u32(this.u, true);
            this.u += 4;
        }
        if (j & 0x2) {
            node = document.createTextNode(this.s.substring(this.o, this.o += this.v.u16(this.u, true)));
            this.u += 2;
            if (parent_id !== null) {
                this.n[parent_id] = node;
            }
            return node;
        }
        else {
            const parent_element = this.createElement();
            j = this.v.u16(this.u, true);
            this.u += 2;
            numAttributes = j & 0xFF;
            const numChildren = (j & 0xFF00) >>> 8;
            for (i = 0; i < numAttributes; i++) {
                j = this.v.u32(this.u, true);
                attr = j & 0xFF;
                switch (attr) {
                    case 255:
                        // the attribute is encoded as an enum and the namespace is encoded as a string
                        // we use all 4 bytes of j just read
                        this.u += 4;
                        attr = attrs[this.v.u8((j & 0xFF00) >>> 8)];
                        parent_element.setAttributeNS(this.s.substring(this.o, this.o += (j & 0xFFFF0000) >>> 16), attr);
                        break;
                    case 254:
                        // the attribute is encoded as a string
                        // move one byte forward to skip the byte for attr
                        this.u++;
                        j = this.v.u32(this.u, true);
                        this.u += 4;
                        attr = this.s.substring(this.o, this.o += j & 0xFFFF);
                        parent_element.setAttribute(attr, this.s.substring(this.o, this.o += (j & 0xFFFF0000) >>> 16));
                        break;
                    case 253:
                        // the attribute and namespace are encoded as strings
                        // we use 3 bytes of j just read
                        this.u += 3;
                        attr = this.s.substring(this.o, this.o += (j & 0xFFFF00) >>> 8);
                        j = this.v.u32(this.u, true);
                        this.u += 4;
                        ns = this.s.substring(this.o, this.o += j & 0xFFFF);
                        value = this.s.substring(this.o, this.o += (j & 0xFFFF0000) >>> 16);
                        parent_element.setAttributeNS(ns, attr, value);
                        break;
                    default:
                        // we use 3 bytes of j just read
                        this.u += 3;
                        parent_element.setAttribute(attrs[attr], this.s.substring(this.o, this.o += (j & 0xFFFF00) >>> 8));
                        break;
                }
            }
            for (let w = 0; w < numChildren; w++) {
                parent_element.appendChild(this.createFullElement());
            }
            if (parent_id !== null) {
                this.n[parent_id] = parent_element;
            }
            return parent_element;
        }
    }

    decodeU32() {
        this.u += 4;
        return this.v.u32(this.u - 4, true);
    }

    SetNode(id, node) {
        this.n[id] = node;
    }

    GetNode(id) {
        return this.n[id];
    }
}

const els = [
    "a",
    "abbr",
    "acronym",
    "address",
    "applet",
    "area",
    "article",
    "aside",
    "audio",
    "b",
    "base",
    "bdi",
    "bdo",
    "bgsound",
    "big",
    "blink",
    "blockquote",
    "body",
    "br",
    "button",
    "canvas",
    "caption",
    "center",
    "cite",
    "code",
    "col",
    "colgroup",
    "content",
    "data",
    "datalist",
    "dd",
    "del",
    "details",
    "dfn",
    "dialog",
    "dir",
    "div",
    "dl",
    "dt",
    "em",
    "embed",
    "fieldset",
    "figcaption",
    "figure",
    "font",
    "footer",
    "form",
    "frame",
    "frameset",
    "h1",
    "head",
    "header",
    "hgroup",
    "hr",
    "html",
    "i",
    "iframe",
    "image",
    "img",
    "input",
    "ins",
    "kbd",
    "keygen",
    "label",
    "legend",
    "li",
    "link",
    "main",
    "map",
    "mark",
    "marquee",
    "menu",
    "menuitem",
    "meta",
    "meter",
    "nav",
    "nobr",
    "noembed",
    "noframes",
    "noscript",
    "object",
    "ol",
    "optgroup",
    "option",
    "output",
    "p",
    "param",
    "picture",
    "plaintext",
    "portal",
    "pre",
    "progress",
    "q",
    "rb",
    "rp",
    "rt",
    "rtc",
    "ruby",
    "s",
    "samp",
    "script",
    "section",
    "select",
    "shadow",
    "slot",
    "small",
    "source",
    "spacer",
    "span",
    "strike",
    "strong",
    "style",
    "sub",
    "summary",
    "sup",
    "table",
    "tbody",
    "td",
    "template",
    "textarea",
    "tfoot",
    "th",
    "thead",
    "time",
    "title",
    "tr",
    "track",
    "tt",
    "u",
    "ul",
    "var",
    "video",
    "wbr",
    "xmp",
];

const attrs = [
    "accept-charset",
    "accept",
    "accesskey",
    "action",
    "align",
    "allow",
    "alt",
    "aria-atomic",
    "aria-busy",
    "aria-controls",
    "aria-current",
    "aria-describedby",
    "aria-description",
    "aria-details",
    "aria-disabled",
    "aria-dropeffect",
    "aria-errormessage",
    "aria-flowto",
    "aria-grabbed",
    "aria-haspopup",
    "aria-hidden",
    "aria-invalid",
    "aria-keyshortcuts",
    "aria-label",
    "aria-labelledby",
    "aria-live",
    "aria-owns",
    "aria-relevant",
    "aria-roledescription",
    "async",
    "autocapitalize",
    "autocomplete",
    "autofocus",
    "autoplay",
    "background",
    "bgcolor",
    "border",
    "buffered",
    "capture",
    "challenge",
    "charset",
    "checked",
    "cite",
    "class",
    "code",
    "codebase",
    "color",
    "cols",
    "colspan",
    "content",
    "contenteditable",
    "contextmenu",
    "controls",
    "coords",
    "crossorigin",
    "csp",
    "data",
    "datetime",
    "decoding",
    "default",
    "defer",
    "dir",
    "dirname",
    "disabled",
    "download",
    "draggable",
    "enctype",
    "enterkeyhint",
    "for",
    "form",
    "formaction",
    "formenctype",
    "formmethod",
    "formnovalidate",
    "formtarget",
    "headers",
    "height",
    "hidden",
    "high",
    "href",
    "hreflang",
    "http-equiv",
    "icon",
    "id",
    "importance",
    "inputmode",
    "integrity",
    "intrinsicsize",
    "ismap",
    "itemprop",
    "keytype",
    "kind",
    "label",
    "lang",
    "language",
    "list",
    "loading",
    "loop",
    "low",
    "manifest",
    "max",
    "maxlength",
    "media",
    "method",
    "min",
    "minlength",
    "multiple",
    "muted",
    "name",
    "novalidate",
    "open",
    "optimum",
    "pattern",
    "ping",
    "placeholder",
    "poster",
    "preload",
    "radiogroup",
    "readonly",
    "referrerpolicy",
    "rel",
    "required",
    "reversed",
    "role",
    "rows",
    "rowspan",
    "sandbox",
    "scope",
    "scoped",
    "selected",
    "shape",
    "size",
    "sizes",
    "slot",
    "span",
    "spellcheck",
    "src",
    "srcdoc",
    "srclang",
    "srcset",
    "start",
    "step",
    "style",
    "summary",
    "tabindex",
    "target",
    "title",
    "translate",
    "type",
    "usemap",
    "value",
    "width",
    "wrap",
];
