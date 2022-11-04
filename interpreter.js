let op, len, ns, attr, i, value, element, ptr, pos, end, out, char, numAttributes, endRounded, inptr, buffer, metadata, parent, children, node, name, id, nodes;

export function work_last_created() {
    inptr.Work();
}

export function last_needs_memory() {
    return !buffer.byteLength;
}

export function update_last_memory(mem) {
    inptr.UpdateMemory(mem);
}

function exOp() {
    // first bool: op & 0x20
    // second bool: op & 0x40

    // first child
    switch (op & 0x1F) {
        case 0:
            inptr.lastNode = inptr.lastNode.firstChild;
            break;
        // next sibling
        case 1:
            inptr.lastNode = inptr.lastNode.nextSibling;
            break;
        // parent
        case 2:
            inptr.lastNode = inptr.lastNode.parentNode;
            break;
        // store with id
        case 3:
            inptr.nodes[inptr.view.getUint32(inptr.u8BufPos, true)] = inptr.lastNode;
            inptr.u8BufPos += 4;
            break;
        // set last node
        case 4:
            inptr.lastNode = inptr.nodes[inptr.view.getUint32(inptr.u8BufPos, true)];
            inptr.u8BufPos += 4;
            break;
        // stop
        case 5:
            return true;
        // create full element
        case 6:
            this.lastNode = inptr.createFullElement();
            break;
        // append children
        case 7:
            // the first bool is encoded as op & (1 << 5)
            if (op & 0x20) {
                parent = inptr.nodes[inptr.view.getUint32(inptr.u8BufPos, true)];
                inptr.u8BufPos += 4;
            }
            else {
                parent = inptr.lastNode;
            }
            // the second bool is encoded as op & (1 << 6)
            // second bool encodes if there are many children
            if (op & 0x40) {
                len = inptr.decodeU32();
                for (i = 0; i < len; i++) {
                    parent.appendChild(inptr.nodes[inptr.view.getUint32(inptr.u8BufPos, true)]);
                    inptr.u8BufPos += 4;
                }
            }
            else {
                parent.appendChild(inptr.nodes[inptr.view.getUint32(inptr.u8BufPos, true)]);
                inptr.u8BufPos += 4;
            }
            break;
        // replace with
        case 8:
            // the first bool is encoded as op & (1 << 5)
            if (op & 0x20) {
                parent = inptr.nodes[inptr.view.getUint32(inptr.u8BufPos, true)];
                inptr.u8BufPos += 4;
            }
            else {
                parent = inptr.lastNode;
            }
            if (op & 0x40) {
                len = inptr.decodeU32();
                children = [];
                for (i = 0; i < len; i++) {
                    children.push(inptr.nodes[inptr.view.getUint32(inptr.u8BufPos, true)]);
                    inptr.u8BufPos += 4;
                }
                parent.replaceWith(...children);
            }
            else {
                parent.replaceWith(inptr.nodes[inptr.view.getUint32(inptr.u8BufPos, true)]);
                inptr.u8BufPos += 4;
            }
            break;
        // insert after
        case 9:
            // the first bool is encoded as op & (1 << 5)
            if (op & 0x20) {
                parent = inptr.nodes[inptr.view.getUint32(inptr.u8BufPos, true)];
                inptr.u8BufPos += 4;
            }
            else {
                parent = inptr.lastNode;
            }
            if (op & 0x40) {
                len = inptr.decodeU32();
                children = [];
                for (i = 0; i < len; i++) {
                    children.push(inptr.nodes[inptr.view.getUint32(inptr.u8BufPos, true)]);
                    inptr.u8BufPos += 4;
                }
                parent.after(...children);
            } else {
                parent.after(inptr.nodes[inptr.view.getUint32(inptr.u8BufPos, true)]);
                inptr.u8BufPos += 4;
            }
            break;
        // insert before
        case 10:
            // the first bool is encoded as op & (1 << 5)
            if (op & 0x20) {
                parent = inptr.nodes[inptr.view.getUint32(inptr.u8BufPos, true)];
                inptr.u8BufPos += 4;
            }
            else {
                parent = inptr.lastNode;
            }
            if (op & 0x40) {
                len = inptr.decodeU32();
                children = [];
                for (i = 0; i < len; i++) {
                    children.push(inptr.nodes[inptr.view.getUint32(inptr.u8BufPos, true)]);
                    inptr.u8BufPos += 4;
                }
                parent.before(...children);
            } else {
                parent.before(inptr.nodes[inptr.view.getUint32(inptr.u8BufPos, true)]);
                inptr.u8BufPos += 4;
            }
            break;
        // remove
        case 11:
            // the first bool is encoded as op & (1 << 5)
            if (op & 0x20) {
                inptr.nodes[inptr.view.getUint32(inptr.u8BufPos, true)].remove();
                inptr.u8BufPos += 4;
            }
            else {
                inptr.lastNode.remove();
            }
            break;
        // create text node
        case 12:
            inptr.lastNode = document.createTextNode(inptr.strings.substring(inptr.strPos, inptr.strPos += inptr.view.getUint16(inptr.u8BufPos, true)));
            inptr.u8BufPos += 2;
            // the first bool is encoded as op & (1 << 5)
            if (op & 0x20) {
                inptr.nodes[inptr.view.getUint32(inptr.u8BufPos, true)] = inptr.lastNode;
                inptr.u8BufPos += 4;
            }
            break;
        // create element
        case 13:
            name = inptr.strings.substring(inptr.strPos, inptr.strPos += inptr.view.getUint16(inptr.u8BufPos, true));
            inptr.u8BufPos += 2;
            // the first bool is encoded as op & (1 << 5)
            if (op & 0x20) {
                inptr.lastNode = document.createElementNS(name, inptr.strings.substring(inptr.strPos, inptr.strPos += inptr.view.getUint16(inptr.u8BufPos, true)));
                inptr.u8BufPos += 2;
            }
            else {
                inptr.lastNode = document.createElement(name);
            }
            // the second bool is encoded as op & (1 << 6)
            if (op & 0x40) {
                inptr.nodes[inptr.view.getUint32(inptr.u8BufPos, true)] = inptr.lastNode;
                inptr.u8BufPos += 4;
            }
            break;
        // set text
        case 14:
            // the first bool is encoded as op & (1 << 5)
            if (op & 0x20) {
                id = inptr.view.getUint32(inptr.u8BufPos, true);
                inptr.u8BufPos += 4;
                inptr.nodes[id].textContent = inptr.strings.substring(inptr.strPos, inptr.strPos += inptr.view.getUint16(inptr.u8BufPos, true));
                inptr.u8BufPos += 2;
            }
            else {
                inptr.lastNode.textContent = inptr.strings.substring(inptr.strPos, inptr.strPos += inptr.view.getUint16(inptr.u8BufPos, true));
                inptr.u8BufPos += 2;
            }
            break;
        // set attribute
        case 15:
            // the first bool is encoded as op & (1 << 5)
            if (op & 0x20) {
                node = inptr.nodes[inptr.view.getUint32(inptr.u8BufPos, true)];
                inptr.u8BufPos += 4;
            }
            else {
                node = inptr.lastNode;
            }
            // the second bool is encoded as op & (1 << 6)
            if (op & 0x40) {
                attr = inptr.strings.substring(inptr.strPos, inptr.strPos += inptr.view.getUint16(inptr.u8BufPos, true));
                inptr.u8BufPos += 2;
                node.setAttribute(attr, inptr.strings.substring(inptr.strPos, inptr.strPos += inptr.view.getUint16(inptr.u8BufPos, true)));
                inptr.u8BufPos += 2;
            } else {
                node.setAttribute(attrs[inptr.view.getUint8(inptr.u8BufPos++)], inptr.strings.substring(inptr.strPos, inptr.strPos += inptr.view.getUint16(inptr.u8BufPos, true)));
                inptr.u8BufPos += 2;
            }
            break;
        // set attribute ns
        case 16:
            // the first bool is encoded as op & (1 << 5)
            if (op & 0x20) {
                node = inptr.nodes[inptr.view.getUint32(inptr.u8BufPos, true)];
                inptr.u8BufPos += 4;
            }
            else {
                node = inptr.lastNode;
            }
            attr = inptr.strings.substring(inptr.strPos, inptr.strPos += inptr.view.getUint16(inptr.u8BufPos, true));
            inptr.u8BufPos += 2;
            ns = inptr.strings.substring(inptr.strPos, inptr.strPos += inptr.view.getUint16(inptr.u8BufPos, true));
            inptr.u8BufPos += 2;
            value = inptr.strings.substring(inptr.strPos, inptr.strPos += inptr.view.getUint16(inptr.u8BufPos, true));
            inptr.u8BufPos += 2;
            if (ns === "style") {
                // @ts-ignore
                node.style[attr] = value;
            } else if (ns != null || ns != undefined) {
                node.setAttributeNS(ns, attr, value);
            }
            break;
        // remove attribute
        case 17:
            // the first bool is encoded as op & (1 << 5)
            if (op & 0x20) {
                node = inptr.nodes[inptr.view.getUint32(inptr.u8BufPos, true)];
                inptr.u8BufPos += 4;
            }
            else {
                node = inptr.lastNode;
            }
            // the second bool is encoded as op & (1 << 6)
            if (op & 0x40) {
                node.removeAttribute(inptr.strings.substring(inptr.strPos, inptr.strPos += inptr.view.getUint16(inptr.u8BufPos, true)));
                inptr.u8BufPos += 2;
            } else {
                node.removeAttribute(attrs[inptr.view.getUint8(inptr.u8BufPos++)]);
            }
            break;
        // remove attribute ns
        case 18:
            // the first bool is encoded as op & (1 << 5)
            if (op & 0x20) {
                node = inptr.nodes[inptr.view.getUint32(inptr.u8BufPos, true)];
                inptr.u8BufPos += 4;
            }
            else {
                node = inptr.lastNode;
            }
            attr = inptr.strings.substring(inptr.strPos, inptr.strPos += inptr.view.getUint16(inptr.u8BufPos, true));
            inptr.u8BufPos += 2;
            node.removeAttributeNS(inptr.strings.substring(inptr.strPos, inptr.strPos += inptr.view.getUint16(inptr.u8BufPos, true)), attr);
            inptr.u8BufPos += 2;
            break;
        // clone node
        case 19:
            // the first bool is encoded as op & (1 << 5)
            if (op & 0x20) {
                const id = inptr.view.getUint32(inptr.u8BufPos, true);
                inptr.lastNode = inptr.nodes[id].cloneNode(true);
                inptr.u8BufPos += 4;
            }
            else {
                inptr.lastNode = inptr.lastNode.cloneNode(true);
            }
            // the second bool is encoded as op & (1 << 6)
            if (op & 0x40) {
                inptr.nodes[inptr.view.getUint32(inptr.u8BufPos, true)] = inptr.lastNode;
                inptr.u8BufPos += 4;
            }
            break;
        // clone node children
        case 20:
            // the first bool is encoded as op & (1 << 5)
            if (op & 0x20) {
                node = inptr.nodes[inptr.view.getUint32(inptr.u8BufPos, true)].cloneNode(true).firstChild;
                inptr.u8BufPos += 4;
            }
            else {
                node = inptr.lastNode.cloneNode(true).firstChild;
            }
            for (; node !== null; node = node.nextSibling) {
                if (inptr.view.getUint8(inptr.u8BufPos++) === 1) {
                    inptr.nodes[inptr.view.getUint32(inptr.u8BufPos, true)] = node;
                    inptr.u8BufPos += 4;
                }
            }
            break;
        default:
            break;
    }
}

export class JsInterpreter {
    constructor(mem, _metadata_ptr, _ptr_ptr, _str_ptr_ptr, _str_len_ptr) {
        this.lastNode;
        this.nodes = [];
        this.parents = [];
        this.UpdateMemory(mem);
        this.last_start_pos;
        this.metadata_ptr = _metadata_ptr;
        this.ptr_ptr = _ptr_ptr;
        this.str_ptr_ptr = _str_ptr_ptr;
        this.str_len_ptr = _str_len_ptr;
        this.strings = "";
        this.strPos = 0;
        this.decoder = new TextDecoder();
        this.idSize = 1;
        inptr = this;
    }

    NeedsMemory() {
        return this.view.buffer.byteLength === 0;
    }

    UpdateMemory(mem) {
        this.view = new DataView(mem.buffer);
        buffer = mem.buffer;
    }

    Work() {
        metadata = this.view.getUint8(this.metadata_ptr);
        if (metadata & 0x01) {
            this.last_start_pos = this.view.getUint32(this.ptr_ptr, true);
        }
        this.u8BufPos = this.last_start_pos;
        if (metadata & 0x02) {
            len = this.view.getUint32(this.str_len_ptr, true);
            ptr = this.view.getUint32(this.str_ptr_ptr, true);
            // for small strings decoding them in javascript to avoid the overhead of native calls is faster
            if (len < 100) {
                // the fourth boolean contains information about whether the string is all ascii or utf8
                if (metadata & 0x04) {
                    this.strings = this.batchedAsciiDecode(ptr, len);
                }
                else {
                    this.strings = this.utf8Decode(ptr, len);
                }
            }
            else {
                this.strings = this.decoder.decode(new DataView(this.view.buffer, ptr, len));
            }
            this.strPos = 0;
        }

        // this is faster than a while(true) loop
        for (; ;) {
            // op = this.view.getUint8(this.u8BufPos++);
            // if (this.exOp(op & 0x1F)) return;
            op = this.view.getUint32(this.u8BufPos, true);
            this.u8BufPos += 4;
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
        element = this.view.getUint8(this.u8BufPos++);
        if (element === 255) {
            element = document.createElement(this.strings.substring(this.strPos, this.strPos += this.view.getUint16(this.u8BufPos, true)));
            this.u8BufPos += 2;
            return element;
        }
        else {
            return document.createElement(els[element]);
        }
    }

    createFullElement() {
        const parent_id = this.decodeMaybeIdByteBool(),
            parent_element = this.createElement();
        numAttributes = this.view.getUint8(this.u8BufPos++);
        for (i = 0; i < numAttributes; i++) {
            attr = this.view.getUint8(this.u8BufPos++);
            switch (attr) {
                case 254:
                    attr = this.strings.substring(this.strPos, this.strPos += this.view.getUint16(this.u8BufPos, true));
                    this.u8BufPos += 2;
                    ns = this.strings.substring(this.strPos, this.strPos += this.view.getUint16(this.u8BufPos, true));
                    this.u8BufPos += 2;
                    value = this.strings.substring(this.strPos, this.strPos += this.view.getUint16(this.u8BufPos, true));
                    this.u8BufPos += 2;
                    parent_element.setAttributeNS(ns, attr, value);
                    break;
                case 255:
                    attr = this.strings.substring(this.strPos, this.strPos += this.view.getUint16(this.u8BufPos, true));
                    this.u8BufPos += 2;
                    parent_element.setAttribute(attr, this.strings.substring(this.strPos, this.strPos += this.view.getUint16(this.u8BufPos, true)));
                    this.u8BufPos += 2;
                    break;
                default:
                    parent_element.setAttribute(attrs[attr], this.strings.substring(this.strPos, this.strPos += this.view.getUint16(this.u8BufPos, true)));
                    this.u8BufPos += 2;
                    break;
            }
        }
        const numChildren = this.view.getUint8(this.u8BufPos++);
        for (let i = 0; i < numChildren; i++) {
            parent_element.appendChild(this.createFullElement());
        }
        if (parent_id !== null) {
            this.nodes[parent_id] = parent_element;
        }
        return parent_element;
    }

    // decodes and returns a node encoded with a boolean as a byte representing whether it is a new id or let last node
    decodeMaybeIdByteBool() {
        if (this.view.getUint8(this.u8BufPos++) === 0) {
            return null;
        }
        else {
            const id = this.view.getUint32(this.u8BufPos, true);
            this.u8BufPos += 4;
            return id;
        }
    }

    decodeU32() {
        this.u8BufPos += 4;
        return this.view.getUint32(this.u8BufPos - 4, true);
    }

    SetNode(id, node) {
        this.nodes[id] = node;
    }

    GetNode(id) {
        return this.nodes[id];
    }

    utf8Decode(start, byteLength) {
        pos = start;
        end = pos + byteLength;
        out = "";
        while (pos < end) {
            char = this.view.getUint8(pos++);
            if ((char & 0x80) === 0) {
                // 1 byte
                out += String.fromCharCode(char);
            } else if ((char & 0xe0) === 0xc0) {
                // 2 bytes
                out += String.fromCharCode(((char & 0x1f) << 6) | (this.view.getUint8(pos++) & 0x3f));
            } else if ((char & 0xf0) === 0xe0) {
                // 3 bytes
                out += String.fromCharCode(((char & 0x1f) << 12) | ((this.view.getUint8(pos++) & 0x3f) << 6) | (this.view.getUint8(pos++) & 0x3f));
            } else if ((char & 0xf8) === 0xf0) {
                // 4 bytes
                let unit = ((char & 0x07) << 0x12) | ((this.view.getUint8(pos++) & 0x3f) << 0x0c) | ((this.view.getUint8(pos++) & 0x3f) << 0x06) | (this.view.getUint8(pos++) & 0x3f);
                if (unit > 0xffff) {
                    unit -= 0x10000;
                    out += String.fromCharCode(((unit >>> 10) & 0x3ff) | 0xd800);
                    unit = 0xdc00 | (unit & 0x3ff);
                }
                out += String.fromCharCode(unit);
            } else {
                out += String.fromCharCode(char);
            }
        }

        return out;
    }

    batchedAsciiDecode(start, byteLength) {
        pos = start;
        end = pos + byteLength;
        out = "";
        endRounded = pos + ((byteLength / 4) | 0) * 4;
        while (pos < endRounded) {
            char = this.view.getUint32(pos);
            out += String.fromCharCode(char >> 24, (char & 0x00FF0000) >> 16, (char & 0x0000FF00) >> 8, (char & 0x000000FF));
            pos += 4;
        }
        while (pos < end) {
            out += String.fromCharCode(this.view.getUint8(pos++));
        }
        return out;
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
