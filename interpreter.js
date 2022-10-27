export function work_last_created() {
    interpreter.Work();
}

export function last_needs_memory() {
    return interpreter.view.buffer.byteLength === 0;
}

export function update_last_memory(mem) {
    interpreter.UpdateMemory(mem);
}

let parent, len, children, node, ns, attr, op, i, name, value, element, ptr, metadata, pos, end, out, char, numAttributes, endRounded, halfByte, dis, interpreter;

const opLookup = [
    // first child
    function () {
        interpreter.lastNode = interpreter.lastNode.firstChild;
    },
    // next sibling
    function () {
        interpreter.lastNode = interpreter.lastNode.nextSibling;
    },
    // parent
    function () {
        interpreter.lastNode = interpreter.lastNode.parentNode;
    },
    // store with id
    function () {
        interpreter.nodes[interpreter.decodeId()] = interpreter.lastNode;
    },
    // set last node
    function () {
        interpreter.lastNode = interpreter.nodes[interpreter.decodeId()];
    },
    // set id size
    function () {
        interpreter.idSize = interpreter.view.getUint8(interpreter.u8BufPos++);
        interpreter.updateDecodeIdFn();
    },
    // stop
    function () {
        console.error("stop");
    },
    // create full element
    function () {
        interpreter.createFullElement();
    },
    // append children
    function () {
        // the first bool is encoded as op & (1 << 6)
        if (op & 0x40) {
            parent = interpreter.nodes[interpreter.decodeId()];
        }
        else {
            parent = interpreter.lastNode;
        }
        // the second bool is encoded as op & (1 << 7)
        // second bool encodes if there are many children
        if (op & 0x80) {
            len = interpreter.decodeU32();
            for (i = 0; i < len; i++) {
                parent.appendChild(interpreter.nodes[interpreter.decodeId()]);
            }
        }
        else {
            const id = interpreter.decodeId();
            parent.appendChild(interpreter.nodes[id]);
        }
    },
    // replace with
    function () {
        // the first bool is encoded as op & (1 << 6)
        if (op & 0x40) {
            parent = interpreter.nodes[interpreter.decodeId()];
        }
        else {
            parent = interpreter.lastNode;
        }
        len = interpreter.decodeU32();
        if (len === 1) {
            parent.replaceWith(interpreter.nodes[interpreter.decodeId()]);
        }
        else {
            children = [];
            for (i = 0; i < len; i++) {
                children.push(interpreter.nodes[interpreter.decodeId()]);
            }
            parent.replaceWith(...children);
        }
    },
    // insert after
    function () {
        // the first bool is encoded as op & (1 << 6)
        if (op & 0x40) {
            parent = interpreter.nodes[interpreter.decodeId()];
        }
        else {
            parent = interpreter.lastNode;
        }
        len = interpreter.decodeU32();
        if (len === 1) {
            parent.after(interpreter.nodes[interpreter.decodeId()]);
        } else {
            children = [];
            for (i = 0; i < len; i++) {
                children.push(interpreter.nodes[interpreter.decodeId()]);
            }
            parent.after(...children);
        }
    },
    // insert before
    function () {
        // the first bool is encoded as op & (1 << 6)
        if (op & 0x40) {
            parent = interpreter.nodes[interpreter.decodeId()];
        }
        else {
            parent = interpreter.lastNode;
        }
        len = interpreter.decodeU32();
        if (len === 1) {
            parent.before(interpreter.nodes[interpreter.decodeId()]);
        } else {
            children = [];
            for (i = 0; i < len; i++) {
                children.push(interpreter.nodes[interpreter.decodeId()]);
            }
            parent.before(...children);
        }
    },
    // remove
    function () {
        // the first bool is encoded as op & (1 << 6)
        if (op & 0x40) {
            interpreter.nodes[interpreter.decodeId()].remove();
        }
        else {
            interpreter.lastNode.remove();
        }
    },
    // create text node
    function () {
        interpreter.lastNode = document.createTextNode(interpreter.strings.substring(interpreter.strPos, interpreter.strPos += interpreter.decodeU16()));
        // the first bool is encoded as op & (1 << 6)
        if (op & 0x40) {
            interpreter.nodes[interpreter.decodeId()] = interpreter.lastNode;
        }
    },
    // create element
    function () {
        name = interpreter.strings.substring(interpreter.strPos, interpreter.strPos += interpreter.decodeU16());
        // the first bool is encoded as op & (1 << 6)
        if (op & 0x40) {
            interpreter.lastNode = document.createElementNS(name, interpreter.strings.substring(interpreter.strPos, interpreter.strPos += interpreter.decodeU16()));
        }
        else {
            interpreter.lastNode = document.createElement(name);
        }
        // the second bool is encoded as op & (1 << 7)
        if (op & 0x80) {
            interpreter.nodes[interpreter.decodeId()] = interpreter.lastNode;
        }
    },
    // set text
    function () {
        // the first bool is encoded as op & (1 << 6)
        if (op & 0x40) {
            interpreter.nodes[interpreter.decodeId()].textContent = interpreter.strings.substring(interpreter.strPos, interpreter.strPos += interpreter.decodeU16());;
        }
        else {
            interpreter.lastNode.textContent = interpreter.strings.substring(interpreter.strPos, interpreter.strPos += interpreter.decodeU16());;
        }
    },
    // set attribute
    function () {
        // the first bool is encoded as op & (1 << 6)
        if (op & 0x40) {
            node = interpreter.nodes[interpreter.decodeId()];
        }
        else {
            node = interpreter.lastNode;
        }
        // the second bool is encoded as op & (1 << 7)
        if (op & 0x80) {
            node.setAttribute(interpreter.strings.substring(interpreter.strPos, interpreter.strPos += interpreter.decodeU16()), interpreter.strings.substring(interpreter.strPos, interpreter.strPos += interpreter.decodeU16()));
        } else {
            node.setAttribute(attrs[interpreter.view.getUint8(interpreter.u8BufPos++)], interpreter.strings.substring(interpreter.strPos, interpreter.strPos += interpreter.decodeU16()));
        }
    },
    // set attribute ns
    function () {
        // the first bool is encoded as op & (1 << 6)
        if (op & 0x40) {
            node = interpreter.nodes[interpreter.decodeId()];
        }
        else {
            node = interpreter.lastNode;
        }
        attr = interpreter.strings.substring(interpreter.strPos, interpreter.strPos += interpreter.decodeU16());
        ns = interpreter.strings.substring(interpreter.strPos, interpreter.strPos += interpreter.decodeU16());
        value = interpreter.strings.substring(interpreter.strPos, interpreter.strPos += interpreter.decodeU16());
        if (ns === "style") {
            // @ts-ignore
            node.style[attr] = value;
        } else if (ns != null || ns != undefined) {
            node.setAttributeNS(ns, attr, value);
        }
    },
    // remove attribute
    function () {
        // the first bool is encoded as op & (1 << 6)
        if (op & 0x40) {
            node = interpreter.nodes[interpreter.decodeId()];
        }
        else {
            node = interpreter.lastNode;
        }
        // the second bool is encoded as op & (1 << 7)
        if (op & 0x80) {
            node.removeAttribute(interpreter.strings.substring(interpreter.strPos, interpreter.strPos += interpreter.decodeU16()));
        } else {
            node.removeAttribute(attrs[interpreter.view.getUint8(interpreter.u8BufPos++)]);
        }
    },
    // remove attribute ns
    function () {
        // the first bool is encoded as op & (1 << 6)
        if (op & 0x40) {
            node = interpreter.nodes[interpreter.decodeId()];
        }
        else {
            node = interpreter.lastNode;
        }
        attr = interpreter.strings.substring(interpreter.strPos, interpreter.strPos += interpreter.decodeU16());
        node.removeAttributeNS(interpreter.strings.substring(interpreter.strPos, interpreter.strPos += interpreter.decodeU16()), attr);
    },
    // clone node
    function () {
        // the first bool is encoded as op & (1 << 6)
        if (op & 0x40) {
            interpreter.lastNode = interpreter.nodes[interpreter.decodeId()].cloneNode(true);
        }
        else {
            interpreter.lastNode = interpreter.lastNode.cloneNode(true);
        }
        // the second bool is encoded as op & (1 << 7)
        if (op & 0x80) {
            interpreter.nodes[interpreter.decodeId()] = interpreter.lastNode;
        }
    },
    // clone node children
    function () {
        // the first bool is encoded as op & (1 << 6)
        if (op & 0x40) {
            node = interpreter.nodes[interpreter.decodeId()].cloneNode(true).firstChild;
        }
        else {
            node = interpreter.lastNode.cloneNode(true).firstChild;
        }
        for (; node !== null; node = node.nextSibling) {
            if (interpreter.view.getUint8(interpreter.u8BufPos++) === 1) {
                interpreter.nodes[interpreter.decodeId()] = node;
            }
        }
    },
];

export class JsInterpreter {
    constructor(root, mem, _metadata_ptr, _ptr_ptr, _str_ptr_ptr, _str_len_ptr) {
        this.root = root;
        this.lastNode = root;
        this.nodes = [root];
        this.parents = [];
        this.view = new DataView(mem.buffer);
        this.idSize = 1;
        this.last_start_pos;
        this.metadata_ptr = _metadata_ptr;
        this.ptr_ptr = _ptr_ptr;
        this.str_ptr_ptr = _str_ptr_ptr;
        this.str_len_ptr = _str_len_ptr;
        this.strings = "";
        this.strPos = 0;
        this.decoder = new TextDecoder();
        interpreter = this;
        this.updateDecodeIdFn();
    }

    NeedsMemory() {
        return this.view.buffer.byteLength === 0;
    }

    UpdateMemory(mem) {
        this.view = new DataView(mem.buffer);
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
        op = this.view.getUint8(this.u8BufPos++);
        halfByte = op & 1;
        if (halfByte) {
            dis = (op & 0x0E) >> 1;
        }
        else {
            dis = (op & 0x3E) >> 1;
        }

        // this is faster than a while(true) loop
        for (; ;) {
            // first bool: op & 0x40
            // second bool: op & 0x80
            opLookup[dis]();
            if (halfByte) {
                dis = op >> 4;
                halfByte = false;
            } else {
                op = this.view.getUint8(this.u8BufPos++);
                halfByte = op & 1;
                if (halfByte) {
                    dis = (op & 0x0E) >> 1;
                }
                else {
                    dis = (op & 0x3E) >> 1;
                }
            }
            if (dis === 6) {
                return;
            }
        }
    }

    createElement() {
        element = this.view.getUint8(this.u8BufPos++);
        if (element === 255) {
            return document.createElement(this.strings.substring(this.strPos, this.strPos += this.decodeU16()));
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
                    attr = this.strings.substring(this.strPos, this.strPos += this.decodeU16());
                    ns = this.strings.substring(this.strPos, this.strPos += this.decodeU16());
                    value = this.strings.substring(this.strPos, this.strPos += this.decodeU16());
                    parent_element.setAttributeNS(ns, attr, value);
                    break;
                case 255:
                    parent_element.setAttribute(this.strings.substring(this.strPos, this.strPos += this.decodeU16()), this.strings.substring(this.strPos, this.strPos += this.decodeU16()));
                    break;
                default:
                    parent_element.setAttribute(attrs[attr], this.strings.substring(this.strPos, this.strPos += this.decodeU16()));
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
            return this.decodeId();
        }
    }

    updateDecodeIdFn() {
        switch (this.idSize) {
            case 1:
                this.decodeId = function () {
                    return this.view.getUint8(this.u8BufPos++);
                };
                break;
            case 2:
                this.decodeId = function () {
                    this.u8BufPos += 2;
                    return this.view.getUint16(this.u8BufPos - 2, true);
                };
                break;
            case 4:
                this.decodeId = function () {
                    this.u8BufPos += 4;
                    return this.view.getUint32(this.u8BufPos - 4, true);
                };
                break;
        }
    }

    decodeU32() {
        this.u8BufPos += 4;
        return this.view.getUint32(this.u8BufPos - 4, true);
    }

    decodeU16() {
        this.u8BufPos += 2;
        return this.view.getUint16(this.u8BufPos - 2, true);
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

const events = [
    "abort",
    "activate",
    "addstream",
    "addtrack",
    "afterprint",
    "afterscriptexecute",
    "animationcancel",
    "animationend",
    "animationiteration",
    "animationstart",
    "appinstalled",
    "audioend",
    "audioprocess",
    "audiostart",
    "auxclick",
    "beforeinput",
    "beforeprint",
    "beforescriptexecute",
    "beforeunload",
    "beginEvent",
    "blocked",
    "blur",
    "boundary",
    "bufferedamountlow",
    "cancel",
    "canplay",
    "canplaythrough",
    "change",
    "click",
    "close",
    "closing",
    "complete",
    "compositionend",
    "compositionstart",
    "compositionupdate",
    "connect",
    "connectionstatechange",
    "contentdelete",
    "contextmenu",
    "copy",
    "cuechange",
    "cut",
    "datachannel",
    "dblclick",
    "devicechange",
    "devicemotion",
    "deviceorientation",
    "DOMActivate",
    "DOMContentLoaded",
    "DOMMouseScroll",
    "drag",
    "dragend",
    "dragenter",
    "dragleave",
    "dragover",
    "dragstart",
    "drop",
    "durationchange",
    "emptied",
    "end",
    "ended",
    "endEvent",
    "enterpictureinpicture",
    "error",
    "focus",
    "focusin",
    "focusout",
    "formdata",
    "fullscreenchange",
    "fullscreenerror",
    "gamepadconnected",
    "gamepaddisconnected",
    "gatheringstatechange",
    "gesturechange",
    "gestureend",
    "gesturestart",
    "gotpointercapture",
    "hashchange",
    "icecandidate",
    "icecandidateerror",
    "iceconnectionstatechange",
    "icegatheringstatechange",
    "input",
    "inputsourceschange",
    "install",
    "invalid",
    "keydown",
    "keypress",
    "keyup",
    "languagechange",
    "leavepictureinpicture",
    "load",
    "loadeddata",
    "loadedmetadata",
    "loadend",
    "loadstart",
    "lostpointercapture",
    "mark",
    "merchantvalidation",
    "message",
    "messageerror",
    "mousedown",
    "mouseenter",
    "mouseleave",
    "mousemove",
    "mouseout",
    "mouseover",
    "mouseup",
    "mousewheel",
    "msContentZoom",
    "u8BufestureChange",
    "u8BufestureEnd",
    "u8BufestureHold",
    "u8BufestureStart",
    "u8BufestureTap",
    "MSInertiaStart",
    "MSManipulationStateChanged",
    "mute",
    "negotiationneeded",
    "nomatch",
    "notificationclick",
    "offline",
    "online",
    "open",
    "orientationchange",
    "pagehide",
    "pageshow",
    "paste",
    "pause",
    "payerdetailchange",
    "paymentmethodchange",
    "play",
    "playing",
    "pointercancel",
    "pointerdown",
    "pointerenter",
    "pointerleave",
    "pointerlockchange",
    "pointerlockerror",
    "pointermove",
    "pointerout",
    "pointerover",
    "pointerup",
    "popstate",
    "progress",
    "push",
    "pushsubscriptionchange",
    "ratechange",
    "readystatechange",
    "rejectionhandled",
    "removestream",
    "removetrack",
    "removeTrack",
    "repeatEvent",
    "reset",
    "resize",
    "resourcetimingbufferfull",
    "result",
    "resume",
    "scroll",
    "search",
    "seeked",
    "seeking",
    "select",
    "selectedcandidatepairchange",
    "selectend",
    "selectionchange",
    "selectstart",
    "shippingaddresschange",
    "shippingoptionchange",
    "show",
    "signalingstatechange",
    "slotchange",
    "soundend",
    "soundstart",
    "speechend",
    "speechstart",
    "squeeze",
    "squeezeend",
    "squeezestart",
    "stalled",
    "start",
    "statechange",
    "storage",
    "submit",
    "success",
    "suspend",
    "timeout",
    "timeupdate",
    "toggle",
    "tonechange",
    "touchcancel",
    "touchend",
    "touchmove",
    "touchstart",
    "track",
    "transitioncancel",
    "transitionend",
    "transitionrun",
    "transitionstart",
    "unhandledrejection",
    "unload",
    "unmute",
    "upgradeneeded",
    "versionchange",
    "visibilitychange",
    "voiceschanged",
    "volumechange",
    "vrdisplayactivate",
    "vrdisplayblur",
    "vrdisplayconnect",
    "vrdisplaydeactivate",
    "vrdisplaydisconnect",
    "vrdisplayfocus",
    "vrdisplaypointerrestricted",
    "vrdisplaypointerunrestricted",
    "vrdisplaypresentchange",
    "waiting",
    "webglcontextcreationerror",
    "webglcontextlost",
    "webglcontextrestored",
    "webkitmouseforcechanged",
    "webkitmouseforcedown",
    "webkitmouseforceup",
    "webkitmouseforcewillbegin",
    "wheel",
];