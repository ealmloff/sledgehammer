export function work_last_created() {
    window.interpreter.Work();
}

export function last_needs_memory() {
    return window.interpreter.NeedsMemory();
}

export function update_last_memory(mem) {
    window.interpreter.UpdateMemory(mem);
}

let parent, len, children, node, ns, attr, text, i, name, value, element, ptr;

export class JsInterpreter {
    constructor(root, mem, _ptr_ptr, _str_ptr_ptr, _str_len_ptr) {
        this.root = root;
        this.lastNode = root;
        this.nodes = [root];
        this.parents = [];
        this.view = new DataView(mem.buffer);
        this.idSize = 1;
        this.ptr_ptr = _ptr_ptr;
        this.str_ptr_ptr = _str_ptr_ptr;
        this.str_len_ptr = _str_len_ptr;
        this.strings = "";
        this.strPos = 0;
        this.decoder = new TextDecoder();
        window.interpreter = this;
    }

    NeedsMemory() {
        return this.view.buffer.byteLength === 0;
    }

    UpdateMemory(mem) {
        console.log("Updating memory");
        this.view = new DataView(mem.buffer);
    }

    Work() {
        this.u8BufPos = this.view.getUint32(this.ptr_ptr, true);
        ptr = this.view.getUint32(this.str_ptr_ptr, true);
        len = this.view.getUint32(this.str_len_ptr, true);
        if (len > 0) {
            // for small strings decoding them in javascript to avoid the overhead of native calls is faster
            if (len < 25) {
                this.strings = this.utf8Decode(ptr, len);
            }
            else {
                this.strings = this.decoder.decode(new DataView(this.view.buffer, ptr, len));
            }
        }
        this.strPos = 0;
        // this is faster than a while(true) loop
        for (; ;) {
            switch (this.view.getUint8(this.u8BufPos++)) {
                // append children
                case 0:
                    parent = this.getNode();
                    len = this.decodeU32();
                    for (i = 0; i < len; i++) {
                        const child = this.nodes[this.decodeId()];
                        parent.appendChild(child);
                    }
                    break;
                // replace with
                case 1:
                    parent = this.getNode();
                    len = this.decodeU32();
                    children = [];
                    for (i = 0; i < len; i++) {
                        children.push(this.nodes[this.decodeId()]);
                    }
                    parent.replaceWith(...children);
                    break;
                // insert after
                case 2:
                    parent = this.getNode();
                    len = this.decodeU32();
                    children = [];
                    for (i = 0; i < len; i++) {
                        children.push(this.nodes[this.decodeId()]);
                    }
                    parent.after(...children);
                    break;
                // insert before
                case 3:
                    parent = this.getNode();
                    len = this.decodeU32();
                    children = [];
                    for (i = 0; i < len; i++) {
                        children.push(this.nodes[this.decodeId()]);
                    }
                    parent.before(...children);
                    break;
                // remove
                case 4:
                    this.getNode().remove();
                    break;
                // create text node
                case 5:
                    this.lastNode = document.createTextNode(this.strings.substring(this.strPos, this.strPos += this.decodeU16()));
                    if (this.view.getUint8(this.u8BufPos++) === 1) {
                        this.nodes[this.decodeId()] = this.lastNode;
                    }
                    this.checkAppendParent();
                    break;
                // create element
                case 6:
                    name = this.strings.substring(this.strPos, this.strPos += this.decodeU16());
                    if (this.nodes[this.u8BufPos++] === 1) {
                        this.lastNode = document.createElementNS(name, this.strings.substring(this.strPos, this.strPos += this.decodeU16()));
                    }
                    else {
                        this.lastNode = document.createElement(name);
                    }
                    if (this.view.getUint8(this.u8BufPos++) === 1) {
                        this.nodes[this.decodeId()] = this.lastNode;
                    }
                    this.checkAppendParent();
                    children = this.decodeU32();
                    if (children > 0) {
                        this.parents.push([this.lastNode, children]);
                    }
                    break;
                // create placeholder
                case 7:
                    this.lastNode = document.createElement("pre");
                    this.lastNode.hidden = true;
                    if (this.view.getUint8(this.u8BufPos++) === 1) {
                        this.nodes[this.decodeId()] = this.lastNode;
                    }
                    this.checkAppendParent();
                    break;
                // set text
                case 10:
                    node = this.getNode();
                    text = this.strings.substring(this.strPos, this.strPos += this.decodeU16());
                    node.textContent = text;
                    break;
                // set attribute
                case 11:
                    node = this.getNode();
                    attr = this.view.getUint8(this.u8BufPos++);
                    switch (attr) {
                        case 254:
                            attr = this.strings.substring(this.strPos, this.strPos += this.decodeU16());
                            ns = this.strings.substring(this.strPos, this.strPos += this.decodeU16());
                            value = this.strings.substring(this.strPos, this.strPos += this.decodeU16());
                            if (ns === "style") {
                                // @ts-ignore
                                node.style[attr] = value;
                            } else if (ns != null || ns != undefined) {
                                node.setAttributeNS(ns, attr, value);
                            }
                            break;
                        case 255:
                            attr = this.strings.substring(this.strPos, this.strPos += this.decodeU16());
                            value = this.strings.substring(this.strPos, this.strPos += this.decodeU16());
                            node.setAttribute(attr, value);
                            break;
                        default:
                            attr = convertAttribute(attr);
                            value = this.strings.substring(this.strPos, this.strPos += this.decodeU16());
                            node.setAttribute(attr, value);
                            break;
                    }
                    break;
                // remove attribute
                case 12:
                    node = this.getNode();
                    attr = this.view.getUint8(this.u8BufPos++);
                    switch (attr) {
                        case 254:
                            attr = this.strings.substring(this.strPos, this.strPos += this.decodeU16());
                            node.removeAttributeNS(this.strings.substring(this.strPos, this.strPos += this.decodeU16()), attr);
                            break;
                        case 255:
                            attr = this.strings.substring(this.strPos, this.strPos += this.decodeU16());
                            node.removeAttributeNS(ns, attr);
                            break;
                        default:
                            attr = convertAttribute(attr);
                            node.removeAttributeNS(ns, attr);
                            break;
                    }
                    break;
                // clone node
                case 13:
                    this.lastNode = this.getNode().cloneNode(true);
                    if (this.view.getUint8(this.u8BufPos++) === 1) {
                        this.nodes[this.decodeId()] = this.lastNode;
                    }
                    break;
                // clone node children
                case 14:
                    for (let current = this.getNode().cloneNode(true).firstChild; current !== null; current = current.nextSibling) {
                        if (this.view.getUint8(this.u8BufPos++) === 1) {
                            this.nodes[this.decodeId()] = current;
                        }
                    }
                    break;
                // first child
                case 15:
                    this.lastNode = this.lastNode.firstChild;
                    break;
                // next sibling
                case 16:
                    this.lastNode = this.lastNode.nextSibling;
                    break;
                // parent
                case 17:
                    this.lastNode = this.lastNode.parentNode;
                    break;
                // store with id
                case 18:
                    this.nodes[this.decodeId()] = this.lastNode;
                    break;
                // set last node
                case 19:
                    this.lastNode = this.nodes[this.decodeId()];
                    break;
                // set id size
                case 20:
                    this.idSize = this.view.getUint8(this.u8BufPos++);
                    break;
                // stop
                case 21:
                    return;
                // create full element
                case 22:
                    this.createFullElement();
                default:
                    this.u8BufPos--;
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
            return document.createElement(convertElement(element));
        }
    }

    createFullElement() {
        const parent_id = this.decodeMaybeId(),
            parent_element = this.createElement(),
            numAttributes = this.view.getUint8(this.u8BufPos++);
        for (let i = 0; i < numAttributes; i++) {
            attr = this.view.getUint8(this.u8BufPos++);
            switch (attr) {
                case 254:
                    attr = this.strings.substring(this.strPos, this.strPos += this.decodeU16());
                    ns = this.strings.substring(this.strPos, this.strPos += this.decodeU16());
                    parent_element.setAttributeNS(ns, attr, value);
                    break;
                case 255:
                    attr = this.strings.substring(this.strPos, this.strPos += this.decodeU16());
                    value = this.strings.substring(this.strPos, this.strPos += this.decodeU16());
                    parent_element.setAttribute(attr, value);
                    break;
                default:
                    attr = convertAttribute(attr);
                    value = this.strings.substring(this.strPos, this.strPos += this.decodeU16());
                    parent_element.setAttribute(attr, value);
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

    checkAppendParent() {
        if (this.parents.length > 0) {
            const lastParent = this.parents[this.parents.length - 1];
            lastParent[1]--;
            if (lastParent[1] === 0) {
                this.parents.pop();
            }
            lastParent[0].appendChild(this.lastNode);
        }
    }

    // decodes and returns a node
    getNode() {
        if (this.view.getUint8(this.u8BufPos++) === 1) {
            return this.nodes[this.decodeId()];
        }
        else {
            return this.lastNode;
        }
    }

    decodeMaybeId() {
        if (this.view.getUint8(this.u8BufPos++) === 0) {
            return null;
        }
        else {
            return this.decodeId();
        }
    }

    decodeId() {
        switch (this.idSize) {
            case 1:
                return this.view.getUint8(this.u8BufPos++);
            case 2:
                this.u8BufPos += 2;
                return this.view.getUint16(this.u8BufPos - 2, true);
            case 4:
                this.u8BufPos += 4;
                return this.view.getUint32(this.u8BufPos - 4, true);
            case 8:
                this.u8BufPos += 8;
                return this.view.getUint64(this.u8BufPos - 8, true);
            default:
                let val = this.view.getUint8(this.u8BufPos++);
                for (let i = 1; i < this.idSize; i++) {
                    val |= this.view.getUint8(this.u8BufPos++) << (i * 8);
                }
                return val;
        }
    }

    decodeU64() {
        this.u8BufPos += 8;
        return this.view.getUint64(this.u8BufPos - 8, true);
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

    utf8Decode(start, byteLength) {
        let pos = start;
        const end = pos + byteLength;
        let out = "";
        let byte1;
        while (pos < end) {
            byte1 = this.view.getUint8(pos++);
            if ((byte1 & 0x80) === 0) {
                // 1 byte
                out += String.fromCharCode(byte1);
            } else if ((byte1 & 0xe0) === 0xc0) {
                // 2 bytes
                out += String.fromCharCode(((byte1 & 0x1f) << 6) | (this.view.getUint8(pos++) & 0x3f));
            } else if ((byte1 & 0xf0) === 0xe0) {
                // 3 bytes
                out += String.fromCharCode(((byte1 & 0x1f) << 12) | ((this.view.getUint8(pos++) & 0x3f) << 6) | (this.view.getUint8(pos++) & 0x3f));
            } else if ((byte1 & 0xf8) === 0xf0) {
                // 4 bytes
                let unit = ((byte1 & 0x07) << 0x12) | ((this.view.getUint8(pos++) & 0x3f) << 0x0c) | ((this.view.getUint8(pos++) & 0x3f) << 0x06) | (this.view.getUint8(pos++) & 0x3f);
                if (unit > 0xffff) {
                    unit -= 0x10000;
                    out += String.fromCharCode(((unit >>> 10) & 0x3ff) | 0xd800);
                    unit = 0xdc00 | (unit & 0x3ff);
                }
                out += String.fromCharCode(unit);
            } else {
                out += String.fromCharCode(byte1);
            }
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
function convertElement(id) {
    return els[id];
}

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
function convertAttribute(id) {
    return attrs[id];
}

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
function convertEvent(id) {
    return events[id];
}