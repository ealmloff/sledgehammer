let u8Buf = new Uint8Array();
let ptr_ptr;
let len_ptr;

export function interperter_init(mem, _ptr_ptr, _len_ptr) {
    u8Buf = new Uint8Array(mem.buffer);
    ptr_ptr = _ptr_ptr;
    len_ptr = _len_ptr;
}

export function prep() {
    stack = [];
}

let stack = [];
let idSize = 1;
let nodes = [];
// let mock = document.createElement("div");

export function utf8Decode(inputOffset, byteLength) {
    let offset = inputOffset;
    const end = offset + byteLength;

    const out = [];
    while (offset < end) {
        const byte1 = u8Buf[offset++];
        if ((byte1 & 0x80) === 0) {
            // 1 byte
            out.push(byte1);
        } else if ((byte1 & 0xe0) === 0xc0) {
            // 2 bytes
            const byte2 = u8Buf[offset++] & 0x3f;
            out.push(((byte1 & 0x1f) << 6) | byte2);
        } else if ((byte1 & 0xf0) === 0xe0) {
            // 3 bytes
            const byte2 = u8Buf[offset++] & 0x3f;
            const byte3 = u8Buf[offset++] & 0x3f;
            out.push(((byte1 & 0x1f) << 12) | (byte2 << 6) | byte3);
        } else if ((byte1 & 0xf8) === 0xf0) {
            // 4 bytes
            const byte2 = u8Buf[offset++] & 0x3f;
            const byte3 = u8Buf[offset++] & 0x3f;
            const byte4 = u8Buf[offset++] & 0x3f;
            let unit = ((byte1 & 0x07) << 0x12) | (byte2 << 0x0c) | (byte3 << 0x06) | byte4;
            if (unit > 0xffff) {
                unit -= 0x10000;
                out.push(((unit >>> 10) & 0x3ff) | 0xd800);
                unit = 0xdc00 | (unit & 0x3ff);
            }
            out.push(unit);
        } else {
            out.push(byte1);
        }
    }

    return String.fromCharCode.apply(String, out);
}

function decodeId(idx) {
    let id = u8Buf[idx];
    if (id === 0) {
        return 0;
    }
    for (let i = 1; i < idSize; i++) {
        id |= u8Buf[idx + i] << (i * 8);
    }
    return id;
}


function decodeU32(idx) {
    let val = u8Buf[idx];
    for (let i = 1; i < 4; i++) {
        val |= u8Buf[idx + i] << (i * 8);
    }
    return val;
}

export function work() {
    let i = decodeU32(ptr_ptr);
    const end = i + decodeU32(len_ptr);
    while (i < end) {
        const op = u8Buf[i];
        switch (op) {
            // push root
            case 0:
                {
                    const id = decodeId(i + 1);
                    stack.push(id);
                    i += 1 + idSize;
                }
                break;
            // pop root
            case 1:
                {
                    stack.pop();
                    i += 1;
                }
                break;
            // append children
            case 2:
                {
                    const children = u8Buf[i + 1];
                    const parent = stack[stack.length - 1 - children];
                    for (let i = 0; i < children; i++) {
                        parent.appendChild(stack.pop());
                    }
                    i += 2;
                }
                break;
            // replace with
            case 3:
                {
                    const id = decodeId(i + 1);
                    const num = decodeU32(i + 1 + idSize);
                    nodes[id - 1].replaceWith(...stack.splice(-num));
                    i += 5 + idSize;
                }
                break;
            // insert before
            case 4:
                {
                    const id = decodeId(i + 1);
                    const num = decodeU32(i + 1 + idSize);
                    nodes[id - 1].before(...stack.splice(-num));
                    i += 5 + idSize;
                }
                break;
            // insert after
            case 5:
                {
                    const id = decodeId(i + 1);
                    const num = decodeU32(i + 1 + idSize);
                    const splice = stack.splice(-num);
                    nodes[id - 1].after(...splice);
                    i += 5 + idSize;
                }
                break;
            // remove
            case 6:
                {
                    const id = decodeId(i + 1);
                    nodes[id - 1].remove();
                }
                break;
            // create text node
            case 7:
                {
                    const id = decodeId(i + 1);
                    if (id === 0) {
                        i += 2;
                    }
                    else {
                        i += 1 + idSize;
                    }
                    const last = document.createTextNode(utf8Decode(i + 1, u8Buf[i]));
                    stack.push(last);
                    if (id !== 0) {
                        nodes[id - 1] = last;
                    }
                }
                break;
            // create element
            case 8:
                {
                    let str;
                    const id = decodeId(i + 1);
                    if (id === 0) {
                        i += 2;
                    }
                    else {
                        i += 1 + idSize;
                    }
                    const element = u8Buf[i];
                    if (element === 255) {
                        const len = u8Buf[i + 1];
                        const start = i + 2;
                        str = utf8Decode(start, len);
                        i = start + len;
                    }
                    else {
                        str = convertElement(element);
                        i += 1;
                    }
                    const last = document.createElement(str);
                    stack.push(last);
                    if (id !== 0) {
                        nodes[id - 1] = last;
                    }
                }
                break;
            // create element ns
            case 9:
                {
                    let str;
                    const id = decodeId(i + 1);
                    if (id === 0) {
                        i += 2;
                    }
                    else {
                        i += 1 + idSize;
                    }
                    const element = u8Buf[i];
                    if (element === 255) {
                        const len = u8Buf[i + 1];
                        const start = i + 2;
                        str = utf8Decode(start, len);
                        i = start + len;
                    }
                    else {
                        str = convertElement(element);
                        i += 1;
                    }
                    const ns = utf8Decode(i + 1, u8Buf[i]);
                    const last = document.createElementNS(ns, str);
                    stack.push(last);
                    if (id !== 0) {
                        nodes[id - 1] = last;
                    }
                }
                break;
            // create placeholder
            case 10:
                {
                    const id = decodeId(i + 1);
                    i += 1 + idSize;
                    const last = document.createElement("pre");
                    last.hidden = true;
                    stack.push(last);
                    nodes[id - 1] = last;
                }
                break;
            // set event listener
            case 11:
                {
                    const id = decodeId(i + 1);
                    const event = u8Buf[i + 1 + idSize];
                    if (event === 255) {
                        const len = u8Buf[i + 2 + idSize];
                        const start = i + 3 + idSize;
                        str = utf8Decode(start, len);
                        i = start + len;
                    }
                    else {
                        str = convertEvent(event);
                        i += 2 + idSize;
                    }
                    console.log("todo");
                }
                break;
            // remove event listener
            case 12:
                {
                    const id = decodeId(i + 1);
                    const event = u8Buf[i + 1 + idSize];
                    if (event === 255) {
                        const len = u8Buf[i + 2 + idSize];
                        const start = i + 3 + idSize;
                        str = utf8Decode(start, len);
                        i = start + len;
                    }
                    else {
                        str = convertEvent(event);
                        i += 2 + idSize;
                    }
                    console.log("todo");
                }
                break;
            // set text
            case 13:
                {
                    const id = decodeId(i + 1);
                    i += 1 + idSize;
                    nodes[id - 1].textContent = utf8Decode(i + 1, u8Buf[i]);
                }
                break;
            // set attribute
            case 14:
                {
                    let attr;
                    const id = decodeId(i + 1);
                    if (id === 0) {
                        i += 2;
                    }
                    else {
                        i += 1 + idSize;
                    }
                    const data = u8Buf[i];
                    if (data === 255) {
                        const len = u8Buf[i + 1];
                        const start = i + 2;
                        attr = utf8Decode(start, len);
                        i = start + len;
                    }
                    else {
                        attr = convertAttribute(data);
                        i += 1;
                    }
                    const identifier = u8Buf[i];
                    let val;
                    if (identifier === 255) {
                        val = true;
                        i += 1;
                    }
                    else if (identifier === 0) {
                        val = false;
                        i += 1;
                    }
                    else {
                        const start = i + 1;
                        val = utf8Decode(start, identifier);
                        i = start + identifier;
                    }
                    if (id === 0) {
                        stack[stack.length - 1].setAttribute(attr, val);
                    }
                    else {
                        nodes[id - 1].setAttribute(attr, val);
                    }
                }
                break;
            // remove attribute
            case 15:
                {
                    let attr;
                    const id = decodeId(i + 1);
                    i += 1 + idSize;
                    const data = u8Buf[i];
                    if (data === 255) {
                        const len = u8Buf[i + 1];
                        const start = i + 2;
                        attr = utf8Decode(start, len);
                        i = start + len;
                    }
                    else {
                        attr = convertAttribute(data);
                        i += 1;
                    }
                    nodes[id - 1].removeAttribute(attr);
                }
                break;
            // remove attribute ns
            case 16:
                {
                    let attr;
                    const id = decodeId(i + 1);
                    i += 1 + idSize;
                    const data = u8Buf[i];
                    if (data === 255) {
                        const len = u8Buf[i + 1];
                        const start = i + 2;
                        attr = utf8Decode(start, len);
                        i = start + len;
                    }
                    else {
                        attr = convertAttribute(data);
                        i += 2;
                    }
                    let len = u8Buf[i];
                    const ns = utf8Decode(i + 1, len);
                    i += 1 + len;
                    nodes[id - 1].removeAttributeNS(ns, attr);
                }
                break;
            // set the id size
            case 17:
                {
                    idSize = u8Buf[i + 1];
                    i += 2;
                }
                break;
            default:
                console.log(`unknown opcode ${u8Buf[i]}`);
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
    "head",
    "header",
    "h1",
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
    "accept",
    "accept-charset",
    "accesskey",
    "action",
    "align",
    "allow",
    "alt",
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
    "data-*",
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

export function bench() {
    const batch = 1000;
    const elements = 100;
    {
        let sum = 0;
        for (let i = 0; i < batch; i++) {
            const o = performance.now();
            for (let i = 0; i < elements; i++) {
                let block = document.createElement("blockquote");
                block.setAttribute("hidden", true);
                let div = document.createElement("div");
                block.setAttribute("class", "test");
                block.appendChild(div);
                block.removeAttribute("hidden");
                let input = document.createElement("input");
                block.after(input);
            }
            const n = performance.now();
            sum += n - o;
        }

        console.log(`${sum / batch} native js`);
    }

    // {
    //     let sum = 0;
    //     const head = document.head;
    //     for (let i = 0; i < batch; i++) {
    //         const o = performance.now();
    //         for (let i = 0; i < elements; i++) {
    //             head.setAttribute("alt", "true");
    //         }
    //         const n = performance.now();
    //         sum += n - o;
    //     }

    //     console.log(`${sum / batch} native js`);
    // }
}
