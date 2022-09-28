const globalData = {
    u8Buf: new Uint8Array(),
    templates: [],
    stack: [],
    nodes: [],
};
let u8BufPos,
    current_template,
    ptr_ptr,
    len_ptr,
    idSize = 1,
    // this is a crime, but it is a fast crime so...
    current_template_id,
    current_template_node_id;


class TemplateRef {
    constructor(fragment, dynamicNodePaths, id) {
        this.fragment = fragment;
        this.dynamicNodePaths = dynamicNodePaths;
        this.id = id;
        this.nodes = [];
    }

    buildRoots() {
        this.roots = [];
        let i = 0;
        for (let node = this.fragment.firstChild; node != null; node = node.nextSibling) {
            this.roots[i++] = node;
        }
    }

    build(id) {
        let current = this.fragment;
        const path = this.dynamicNodePaths[id];
        for (let i = 0; i < path.length; i++) {
            const idx = path[i];
            current = current.firstChild;
            for (let i2 = 0; i2 < idx; i2++) {
                current = current.nextSibling;
            }
        }
        this.nodes[id] = current;
        return current;
    }

    get(id) {
        const intial = this.nodes[id];
        if (intial === undefined) {
            return this.build(id);
        }
        else {
            return intial;
        }
    }

    parent() {
        return this.roots[0].parentNode;
    }

    first() {
        return this.roots[0];
    }

    last() {
        return this.roots[this.roots.length - 1];
    }

    move() {
        // move the root nodes into a new template
        this.fragment = new DocumentFragment();
        for (let n of this.roots) {
            this.fragment.appendChild(n);
        }
    }

    getFragment() {
        if (!this.placed) {
            this.placed = true;
        }
        else {
            this.move();
        }
        this.buildRoots();
        return this.fragment;
    }
}

class Template {
    constructor() {
        this.nodes = [];
        this.dynamicNodePaths = [];
        this.template = null;
        this.currentPath = [];
    }

    ref(id) {
        const template = this.template.content.cloneNode(true);
        return new TemplateRef(template, this.dynamicNodePaths, id);
    }
}

export function get(id) {
    const globalData2 = globalData;
    if (id === -1) {
        return globalData2.nodes[current_template_id - 1].get(current_template_node_id);
    }
    else {
        return globalData2.nodes[id - 1];
    }
}

export function interperter_init(mem, _ptr_ptr, _len_ptr) {
    const globalData2 = globalData;
    globalData2.u8Buf = new Uint8Array(mem.buffer);
    ptr_ptr = _ptr_ptr;
    len_ptr = _len_ptr;
}

export function prep() {
    const globalData2 = globalData;
    globalData2.stack = [];
    u8BufPos = 0;
    current_template = null;
    idSize = 1;
    globalData2.nodes = [];
}

export function set_node(id, node) {
    globalData.nodes[id - BigInt(1)] = node;
}

function utf8Decode(byteLength) {
    const u8Buf = globalData.u8Buf;
    const end = u8BufPos + byteLength;
    let out = "";
    while (u8BufPos < end) {
        const byte1 = u8Buf[u8BufPos++];
        if ((byte1 & 0x80) === 0) {
            // 1 byte
            out += String.fromCharCode(byte1);
        } else if ((byte1 & 0xe0) === 0xc0) {
            // 2 bytes
            const byte2 = u8Buf[u8BufPos++] & 0x3f;
            out += String.fromCharCode(((byte1 & 0x1f) << 6) | byte2);
        } else if ((byte1 & 0xf0) === 0xe0) {
            // 3 bytes
            const byte2 = u8Buf[u8BufPos++] & 0x3f;
            const byte3 = u8Buf[u8BufPos++] & 0x3f;
            out += String.fromCharCode(((byte1 & 0x1f) << 12) | (byte2 << 6) | byte3);
        } else if ((byte1 & 0xf8) === 0xf0) {
            // 4 bytes
            const byte2 = u8Buf[u8BufPos++] & 0x3f;
            const byte3 = u8Buf[u8BufPos++] & 0x3f;
            const byte4 = u8Buf[u8BufPos++] & 0x3f;
            let unit = ((byte1 & 0x07) << 0x12) | (byte2 << 0x0c) | (byte3 << 0x06) | byte4;
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

function asciiDecode(byteLength) {
    const globalData2 = globalData;
    const u8Buf = globalData2.u8Buf;
    const end = u8BufPos + byteLength;
    let out = "";
    while (u8BufPos < end) {
        out += String.fromCharCode(u8Buf[u8BufPos++]);
    }
    return out;
}

function decodeId() {
    const globalData2 = globalData;
    const u8Buf = globalData2.u8Buf;
    const id_code = u8Buf[u8BufPos++];
    if (id_code === 0) {
        return 0;
    }
    else if (id_code === 1) {
        let id = u8Buf[u8BufPos++];
        for (let i = 1; i < idSize; i++) {
            id |= u8Buf[u8BufPos++] << (i * 8);
        }
        return id;
    }
    else if (id_code === 2) {
        current_template_id = u8Buf[u8BufPos++];
        for (let i = 1; i < idSize; i++) {
            current_template_id |= u8Buf[u8BufPos++] << (i * 8);
        }
        current_template_node_id = u8Buf[u8BufPos++];
        for (let i = 1; i < idSize; i++) {
            current_template_node_id |= u8Buf[u8BufPos++] << (i * 8);
        }

        return -1;
    }
}

function decodePtr(s) {
    let start = s;
    const u8Buf = globalData.u8Buf;
    let val = u8Buf[start++];
    for (let i = 1; i < 4; i++) {
        val |= u8Buf[start++] << (i * 8);
    }
    return val;
}

function decodeU32() {
    const globalData2 = globalData;
    const u8Buf = globalData2.u8Buf;
    let val = u8Buf[u8BufPos];
    for (let i = 1; i < 4; i++) {
        val |= u8Buf[u8BufPos++] << (i * 8);
    }
    return val;
}

function createElement() {
    let str;
    const globalData2 = globalData;
    const u8Buf = globalData2.u8Buf;
    const element = u8Buf[u8BufPos++];
    if (element === 255) {
        const len = u8Buf[u8BufPos++];
        str = asciiDecode(len);
    }
    else {
        str = convertElement(element);
    }
    return document.createElement(str);
}

function createFullElement() {
    const globalData2 = globalData;
    const u8Buf = globalData2.u8Buf;
    const id = decodeId();
    const element = createElement();
    const numAttributes = u8Buf[u8BufPos++];
    for (let i = 0; i < numAttributes; i++) {
        const attribute = decodeAttribute();
        const value = decodeValue();
        element.setAttribute(attribute, value);
    }
    const numChildren = u8Buf[u8BufPos++];
    for (let i = 0; i < numChildren; i++) {
        if (current_template) {
            const old = current_template.currentPath.slice();
            current_template.currentPath.push(i);
            element.appendChild(createFullElement());
            current_template.currentPath = old;
        }
        else {
            element.appendChild(createFullElement());
        }
    }
    if (current_template) {
        if (id !== 0) {
            current_template.nodes[id - 1] = element;
            current_template.dynamicNodePaths[id - 1] = current_template.currentPath;
        }
    }
    else {
        if (id !== 0) {
            globalData2.nodes[id - 1] = element;
        }
    }
    return element;
}

function decodeValue() {
    const identifier = globalData.u8Buf[u8BufPos++];
    if (identifier === 255) {
        return true;
    }
    else if (identifier === 0) {
        return false;
    }
    else {
        return utf8Decode(identifier);
    }
}

function decodeAttribute() {
    const u8Buf = globalData.u8Buf;
    const data = u8Buf[u8BufPos++];
    if (data === 255) {
        const len = u8Buf[u8BufPos++];
        return asciiDecode(len);
    }
    else {
        return convertAttribute(data);
    }
}

export function work() {
    const globalData2 = globalData;
    const u8Buf = globalData2.u8Buf;
    u8BufPos = decodePtr(ptr_ptr);
    const end = u8BufPos + decodePtr(len_ptr);
    while (u8BufPos < end) {
        switch (u8Buf[u8BufPos++]) {
            // push root
            case 0:
                {
                    const id = decodeId();
                    globalData2.stack.push(id);
                }
                break;
            // pop root
            case 1:
                {
                    globalData2.stack.pop();
                }
                break;
            // append children
            case 2:
                {
                    const stack = globalData2.stack;
                    const children = u8Buf[u8BufPos++];
                    const parent = stack[stack.length - 1 - children];
                    for (let i = 0; i < children; i++) {
                        parent.appendChild(stack.pop());
                    }
                }
                break;
            // replace with
            case 3:
                {
                    const id = decodeId();
                    const num = decodeU32(u8BufPos + idSize);
                    get(id).replaceWith(...globalData2.stack.splice(-num));
                }
                break;
            // insert before
            case 4:
                {
                    const id = decodeId();
                    const num = decodeU32(u8BufPos + idSize);
                    get(id).before(...globalData2.stack.splice(-num));
                }
                break;
            // insert after
            case 5:
                {
                    const id = decodeId();
                    const num = decodeU32(u8BufPos + idSize);
                    const splice = globalData2.stack.splice(-num);
                    get(id).after(...splice);
                }
                break;
            // remove
            case 6:
                {
                    const id = decodeId();
                    get(id).remove();
                }
                break;
            // create text node
            case 7:
                {
                    const id = decodeId();
                    const last = document.createTextNode(utf8Decode(u8Buf[u8BufPos++]));
                    globalData2.stack.push(last);
                    if (id !== 0) {
                        globalData2.nodes[id - 1] = last;
                    }
                }
                break;
            // create element
            case 8:
                const id = decodeId();
                const el = createElement();
                globalData2.stack.push(el);
                if (id !== 0) {
                    globalData2.nodes[id - 1] = el;
                }
                break;
            // create element ns
            case 9:
                {
                    let str;
                    const id = decodeId();
                    const element = u8Buf[u8BufPos++];
                    if (element === 255) {
                        const len = u8Buf[u8BufPos++];
                        str = asciiDecode(len);
                    }
                    else {
                        str = convertElement(element);
                    }
                    const ns = asciiDecode(u8Buf[u8BufPos++]);
                    const last = document.createElementNS(ns, str);
                    globalData2.stack.push(last);
                    if (id !== 0) {
                        globalData2.nodes[id - 1] = last;
                    }
                }
                break;
            // create placeholder
            case 10:
                {
                    const id = decodeId();
                    const last = document.createElement("pre");
                    last.hidden = true;
                    globalData2.stack.push(last);
                    globalData2.nodes[id - 1] = last;
                }
                break;
            // set event listener
            case 11:
                {
                    const id = decodeId();
                    const event = u8Buf[u8BufPos++];
                    if (event === 255) {
                        const len = u8Buf[u8BufPos++];
                        str = asciiDecode(len);
                    }
                    else {
                        str = convertEvent(event);
                    }
                    console.log("todo");
                }
                break;
            // remove event listener
            case 12:
                {
                    const id = decodeId();
                    const event = u8Buf[u8BufPos++];
                    if (event === 255) {
                        const len = u8Buf[u8BufPos++];
                        str = asciiDecode(start, len);
                    }
                    else {
                        str = convertEvent(event);
                    }
                    console.log("todo");
                }
                break;
            // set text
            case 13:
                {
                    const id = decodeId();
                    get(id).textContent = utf8Decode(u8Buf[u8BufPos++]);
                }
                break;
            // set attribute
            case 14:
                {
                    const id = decodeId();
                    const attr = decodeAttribute();
                    const val = decodeValue();
                    const stack = globalData2.stack;
                    if (id === 0) {
                        stack[stack.length - 1].setAttribute(attr, val);
                    }
                    else {
                        get(id).setAttribute(attr, val);
                    }
                }
                break;
            // remove attribute
            case 15:
                {
                    let attr;
                    const id = decodeId();
                    const data = u8Buf[u8BufPos++];
                    if (data === 255) {
                        const len = u8Buf[u8BufPos++];
                        attr = asciiDecode(len);
                    }
                    else {
                        attr = convertAttribute(data);
                    }
                    get(id).removeAttribute(attr);
                }
                break;
            // remove attribute ns
            case 16:
                {
                    let attr;
                    const id = decodeId();
                    const data = u8Buf[u8BufPos++];
                    if (data === 255) {
                        const len = u8Buf[u8BufPos++];
                        attr = asciiDecode(len);
                    }
                    else {
                        attr = convertAttribute(data);
                    }
                    let len = u8Buf[u8BufPos];
                    const ns = asciiDecode(u8BufPos + 1, len);
                    get(id).removeAttributeNS(ns, attr);
                }
                break;
            // set the id size
            case 17:
                {
                    idSize = u8Buf[u8BufPos++];
                }
                break;
            // create full element
            case 18:
                {
                    const el = createFullElement();
                    globalData2.stack.push(el);
                }
                break;
            // create template
            case 19:
                {
                    let template = document.createElement("template");
                    current_template = new Template();
                    const id = decodeId();
                    const root_count = u8Buf[u8BufPos++];
                    for (let i = 0; i < root_count; i++) {
                        current_template.currentPath = [i];
                        template.content.appendChild(createFullElement());
                    }
                    current_template.template = template;
                    globalData2.templates[id] = current_template;
                    current_template = null;
                }
                break;
            // create template ref
            case 20:
                {
                    const template_id = decodeId();
                    const id = decodeId();
                    const template = globalData2.templates[template_id];
                    const ref = template.ref();
                    globalData2.stack.push(ref);
                    if (id !== 0) {
                        globalData2.nodes[id - 1] = ref;
                    }
                }
                break;
            default:
                u8BufPos--;
                console.log(`unknown opcode ${u8Buf[u8BufPos]}`);
                return;
        }
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

const batch = 100000;
const elements = 1;
export function bench(m) {
    let sum = 0;
    let block = document.createElement("blockquote");
    block.setAttribute("hidden", true);
    let div = document.createElement("div");
    block.setAttribute("class", "test");
    block.appendChild(div);
    let input = document.createElement("input");
    block.appendChild(input);
    for (let i = 0; i < batch; i++) {
        const o = performance.now();
        for (let i = 0; i < elements; i++) {
            let x = block.cloneNode(true);
            for (let i = 0; i < m; i++) {
                x.firstChild.setAttribute("class", i);
            }
        }
        const n = performance.now();
        sum += n - o;
    }

    let avg = sum / batch;
    console.log(`${avg} native js cloneNode`);
    return avg;
}

export function bench_template() {
    let sum = 0;
    for (let i = 0; i < batch; i++) {
        const o = performance.now();
        for (let i = 0; i < elements; i++) {
            let block = document.createElement("blockquote");
            block.setAttribute("hidden", true);
            let div = document.createElement("div");
            block.setAttribute("class", "test");
            block.appendChild(div);
            let input = document.createElement("input");
            block.appendChild(input);
        }
        const n = performance.now();
        sum += n - o;
    }

    console.log(`${sum / batch} native js create template`);
}
