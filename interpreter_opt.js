let t, e, s, i, r, o, u, n, a, d, f, l, h, g, c, P, b, p, B, m, w, v, U, N; export function work_last_created() { P.Work() } export function last_needs_memory() { return !b.byteLength } export function update_last_memory(t) { P.UpdateMemory(t) } function exOp() { switch (t & 31) { case 0: P.lastNode = P.lastNode.firstChild; break; case 1: P.lastNode = P.lastNode.nextSibling; break; case 2: P.lastNode = P.lastNode.parentNode; break; case 3: P.nodes[P.view.getUint32(P.u8BufPos, true)] = P.lastNode; P.u8BufPos += 4; break; case 4: P.lastNode = P.nodes[P.view.getUint32(P.u8BufPos, true)]; P.u8BufPos += 4; break; case 5: return true; case 6: P.lastNode = P.createFullElement(); break; case 7: if (t & 32) { B = P.nodes[P.view.getUint32(P.u8BufPos, true)]; P.u8BufPos += 4 } else { B = P.lastNode } if (t & 64) { e = P.decodeU32(); for (r = 0; r < e; r++) { B.appendChild(P.nodes[P.view.getUint32(P.u8BufPos, true)]); P.u8BufPos += 4 } } else { B.appendChild(P.nodes[P.view.getUint32(P.u8BufPos, true)]); P.u8BufPos += 4 } break; case 8: if (t & 32) { B = P.nodes[P.view.getUint32(P.u8BufPos, true)]; P.u8BufPos += 4 } else { B = P.lastNode } if (t & 64) { e = P.decodeU32(); m = []; for (r = 0; r < e; r++) { m.push(P.nodes[P.view.getUint32(P.u8BufPos, true)]); P.u8BufPos += 4 } B.replaceWith(...m) } else { B.replaceWith(P.nodes[P.view.getUint32(P.u8BufPos, true)]); P.u8BufPos += 4 } break; case 9: if (t & 32) { B = P.nodes[P.view.getUint32(P.u8BufPos, true)]; P.u8BufPos += 4 } else { B = P.lastNode } if (t & 64) { e = P.decodeU32(); m = []; for (r = 0; r < e; r++) { m.push(P.nodes[P.view.getUint32(P.u8BufPos, true)]); P.u8BufPos += 4 } B.after(...m) } else { B.after(P.nodes[P.view.getUint32(P.u8BufPos, true)]); P.u8BufPos += 4 } break; case 10: if (t & 32) { B = P.nodes[P.view.getUint32(P.u8BufPos, true)]; P.u8BufPos += 4 } else { B = P.lastNode } if (t & 64) { e = P.decodeU32(); m = []; for (r = 0; r < e; r++) { m.push(P.nodes[P.view.getUint32(P.u8BufPos, true)]); P.u8BufPos += 4 } B.before(...m) } else { B.before(P.nodes[P.view.getUint32(P.u8BufPos, true)]); P.u8BufPos += 4 } break; case 11: if (t & 32) { P.nodes[P.view.getUint32(P.u8BufPos, true)].remove(); P.u8BufPos += 4 } else { P.lastNode.remove() } break; case 12: P.lastNode = document.createTextNode(P.strings.substring(P.strPos, P.strPos += P.view.getUint16(P.u8BufPos, true))); P.u8BufPos += 2; if (t & 32) { P.nodes[P.view.getUint32(P.u8BufPos, true)] = P.lastNode; P.u8BufPos += 4 } break; case 13: P.lastNode = P.createElement(); if (t & 32) { P.nodes[P.view.getUint32(P.u8BufPos, true)] = P.lastNode; P.u8BufPos += 4 } break; case 14: if (t & 32) { U = P.view.getUint32(P.u8BufPos, true); P.u8BufPos += 4; P.nodes[U].textContent = P.strings.substring(P.strPos, P.strPos += P.view.getUint16(P.u8BufPos, true)); P.u8BufPos += 2 } else { P.lastNode.textContent = P.strings.substring(P.strPos, P.strPos += P.view.getUint16(P.u8BufPos, true)); P.u8BufPos += 2 } break; case 15: if (t & 32) { w = P.nodes[P.view.getUint32(P.u8BufPos, true)]; P.u8BufPos += 4 } else { w = P.lastNode } if (t & 64) { r = P.view.getUint32(P.u8BufPos, true); P.u8BufPos += 4; i = P.strings.substring(P.strPos, P.strPos += r & 65535); if (t & 128) { w.setAttributeNS(P.strings.substring(P.strPos, P.strPos += (r & 4294901760) >>> 16), i, P.strings.substring(P.strPos, P.strPos += P.view.getUint16(P.u8BufPos, true))); P.u8BufPos += 2 } else { w.setAttribute(i, P.strings.substring(P.strPos, P.strPos += (r & 4294901760) >>> 16)) } } else { r = P.view.getUint32(P.u8BufPos, true); P.u8BufPos += 3; if (t & 128) { s = P.strings.substring(P.strPos, P.strPos += r & 65535); w.setAttributeNS(s, y[(r & 16711680) >>> 16], P.strings.substring(P.strPos, P.strPos += P.view.getUint16(P.u8BufPos, true))); P.u8BufPos += 2 } else { w.setAttribute(y[r & 255], P.strings.substring(P.strPos, P.strPos += (r & 16776960) >>> 8)) } } break; case 16: if (t & 32) { w = P.nodes[P.view.getUint32(P.u8BufPos, true)]; P.u8BufPos += 4 } else { w = P.lastNode } if (t & 64) { if (t & 128) { r = P.view.getUint32(P.u8BufPos, true); P.u8BufPos += 4; i = P.strings.substring(P.strPos, P.strPos += r & 65535); w.removeAttributeNS(P.strings.substring(P.strPos, P.strPos += (r & 4294901760) >>> 16), i) } else { w.removeAttribute(P.strings.substring(P.strPos, P.strPos += P.view.getUint16(P.u8BufPos, true))); P.u8BufPos += 2 } } else { if (t & 128) { r = P.view.getUint32(P.u8BufPos, true); P.u8BufPos += 3; i = y[r & 255]; w.removeAttributeNS(P.strings.substring(P.strPos, P.strPos += (r & 16776960) >>> 8), i) } else { w.removeAttribute(y[P.view.getUint8(P.u8BufPos++)]) } } break; case 17: if (t & 32) { w = P.nodes[P.view.getUint32(P.u8BufPos, true)]; P.u8BufPos += 4 } else { w = P.lastNode } r = P.view.getUint32(P.u8BufPos, true); P.u8BufPos += 4; w.style.setProperty(P.strings.substring(P.strPos, P.strPos += r & 65535), P.strings.substring(P.strPos, P.strPos += (r & 4294901760) >>> 16)); break; case 18: if (t & 32) { w = P.nodes[P.view.getUint32(P.u8BufPos, true)]; P.u8BufPos += 4 } else { w = P.lastNode } w.style.removeProperty(P.strings.substring(P.strPos, P.strPos += P.view.getUint16(P.u8BufPos, true))); P.u8BufPos += 2; break; case 19: if (t & 32) { P.lastNode = P.nodes[P.view.getUint32(P.u8BufPos, true)].cloneNode(true); P.u8BufPos += 4 } else { P.lastNode = P.lastNode.cloneNode(true) } if (t & 64) { P.nodes[P.view.getUint32(P.u8BufPos, true)] = P.lastNode; P.u8BufPos += 4 } break; case 20: if (t & 32) { w = P.nodes[P.view.getUint32(P.u8BufPos, true)].cloneNode(true).firstChild; P.u8BufPos += 4 } else { w = P.lastNode.cloneNode(true).firstChild } for (; w !== null; w = w.nextSibling) { if (P.view.getUint8(P.u8BufPos++) === 1) { P.nodes[P.view.getUint32(P.u8BufPos, true)] = w; P.u8BufPos += 4 } } break; default: break } } export class JsInterpreter { constructor(t, e, s, i, r) { this.lastNode; this.nodes = []; this.parents = []; this.UpdateMemory(t); this.last_start_pos; this.metadata_ptr = e; this.ptr_ptr = s; this.str_ptr_ptr = i; this.str_len_ptr = r; this.strings = ""; this.strPos = 0; this.decoder = new TextDecoder; this.idSize = 1; P = this } NeedsMemory() { return this.view.buffer.byteLength === 0 } UpdateMemory(t) { this.view = new DataView(t.buffer); b = t.buffer } Work() { p = this.view.getUint8(this.metadata_ptr); if (p & 1) { this.last_start_pos = this.view.getUint32(this.ptr_ptr, true) } this.u8BufPos = this.last_start_pos; if (p & 2) { e = this.view.getUint32(this.str_len_ptr, true); a = this.view.getUint32(this.str_ptr_ptr, true); if (e < 100) { if (p & 4) { this.strings = this.batchedAsciiDecode(a, e) } else { this.strings = this.utf8Decode(a, e) } } else { this.strings = this.decoder.decode(new DataView(this.view.buffer, a, e)) } this.strPos = 0 } for (; ;) { t = this.view.getUint32(this.u8BufPos, true); this.u8BufPos += 4; if (exOp()) return; t >>>= 8; if (exOp()) return; t >>>= 8; if (exOp()) return; t >>>= 8; if (exOp()) return } } createElement() { o = this.view.getUint32(this.u8BufPos, true); n = o & 255; switch (n) { case 255: this.u8BufPos += 4; n = document.createElement(k[(o & 65280) >>> 8], this.strings.substring(this.strPos, this.strPos += (o & 4294901760) >>> 16)); return n; case 254: this.u8BufPos += 3; n = document.createElement(this.strings.substring(this.strPos, this.strPos += (o & 16776960) >>> 8)); return n; case 253: this.u8BufPos += 3; n = this.strings.substring(this.strPos, this.strPos += (o & 16776960) >>> 8); n = document.createElementNS(this.strings.substring(this.strPos, this.strPos += this.view.getUint16(this.u8BufPos, true)), n); this.u8BufPos += 2; return n; default: this.u8BufPos++; return document.createElement(k[n]) } } createFullElement() { const t = this.decodeMaybeIdByteBool(), e = this.createElement(); o = this.view.getUint16(this.u8BufPos, true); this.u8BufPos += 2; g = o & 255; const n = (o & 65280) >>> 8; for (r = 0; r < g; r++) { o = this.view.getUint32(this.u8BufPos, true); i = o & 255; switch (i) { case 255: this.u8BufPos += 4; i = y[this.view.getUint8((o & 65280) >>> 8)]; e.setAttributeNS(this.strings.substring(this.strPos, this.strPos += (o & 4294901760) >>> 16), i); break; case 254: this.u8BufPos++; o = this.view.getUint32(this.u8BufPos, true); this.u8BufPos += 4; i = this.strings.substring(this.strPos, this.strPos += o & 65535); e.setAttribute(i, this.strings.substring(this.strPos, this.strPos += (o & 4294901760) >>> 16)); break; case 253: this.u8BufPos += 3; i = this.strings.substring(this.strPos, this.strPos += (o & 16776960) >>> 8); o = this.view.getUint32(this.u8BufPos, true); this.u8BufPos += 4; s = this.strings.substring(this.strPos, this.strPos += o & 65535); u = this.strings.substring(this.strPos, this.strPos += (o & 4294901760) >>> 16); e.setAttributeNS(s, i, u); break; default: this.u8BufPos += 3; e.setAttribute(y[i], this.strings.substring(this.strPos, this.strPos += (o & 16776960) >>> 8)); break } } for (let a = 0; a < n; a++) { e.appendChild(this.createFullElement()) } if (t !== null) { this.nodes[t] = e } return e } decodeMaybeIdByteBool() { if (this.view.getUint8(this.u8BufPos++) === 0) { return null } else { const t = this.view.getUint32(this.u8BufPos, true); this.u8BufPos += 4; return t } } decodeU32() { this.u8BufPos += 4; return this.view.getUint32(this.u8BufPos - 4, true) } SetNode(t, e) { this.nodes[t] = e } GetNode(t) { return this.nodes[t] } utf8Decode(t, e) { d = t; f = d + e; l = ""; while (d < f) { h = this.view.getUint8(d++); if ((h & 128) === 0) { l += String.fromCharCode(h) } else if ((h & 224) === 192) { l += String.fromCharCode((h & 31) << 6 | this.view.getUint8(d++) & 63) } else if ((h & 240) === 224) { l += String.fromCharCode((h & 31) << 12 | (this.view.getUint8(d++) & 63) << 6 | this.view.getUint8(d++) & 63) } else if ((h & 248) === 240) { let s = (h & 7) << 18 | (this.view.getUint8(d++) & 63) << 12 | (this.view.getUint8(d++) & 63) << 6 | this.view.getUint8(d++) & 63; if (s > 65535) { s -= 65536; l += String.fromCharCode(s >>> 10 & 1023 | 55296); s = 56320 | s & 1023 } l += String.fromCharCode(s) } else { l += String.fromCharCode(h) } } return l } batchedAsciiDecode(t, e) { d = t; f = d + e; l = ""; c = d + (e / 4 | 0) * 4; while (d < c) { h = this.view.getUint32(d); l += String.fromCharCode(h >> 24, (h & 16711680) >> 16, (h & 65280) >> 8, h & 255); d += 4 } while (d < f) { l += String.fromCharCode(this.view.getUint8(d++)) } return l } } const k = ["a", "abbr", "acronym", "address", "applet", "area", "article", "aside", "audio", "b", "base", "bdi", "bdo", "bgsound", "big", "blink", "blockquote", "body", "br", "button", "canvas", "caption", "center", "cite", "code", "col", "colgroup", "content", "data", "datalist", "dd", "del", "details", "dfn", "dialog", "dir", "div", "dl", "dt", "em", "embed", "fieldset", "figcaption", "figure", "font", "footer", "form", "frame", "frameset", "h1", "head", "header", "hgroup", "hr", "html", "i", "iframe", "image", "img", "input", "ins", "kbd", "keygen", "label", "legend", "li", "link", "main", "map", "mark", "marquee", "menu", "menuitem", "meta", "meter", "nav", "nobr", "noembed", "noframes", "noscript", "object", "ol", "optgroup", "option", "output", "p", "param", "picture", "plaintext", "portal", "pre", "progress", "q", "rb", "rp", "rt", "rtc", "ruby", "s", "samp", "script", "section", "select", "shadow", "slot", "small", "source", "spacer", "span", "strike", "strong", "style", "sub", "summary", "sup", "table", "tbody", "td", "template", "textarea", "tfoot", "th", "thead", "time", "title", "tr", "track", "tt", "u", "ul", "var", "video", "wbr", "xmp"]; const y = ["accept-charset", "accept", "accesskey", "action", "align", "allow", "alt", "aria-atomic", "aria-busy", "aria-controls", "aria-current", "aria-describedby", "aria-description", "aria-details", "aria-disabled", "aria-dropeffect", "aria-errormessage", "aria-flowto", "aria-grabbed", "aria-haspopup", "aria-hidden", "aria-invalid", "aria-keyshortcuts", "aria-label", "aria-labelledby", "aria-live", "aria-owns", "aria-relevant", "aria-roledescription", "async", "autocapitalize", "autocomplete", "autofocus", "autoplay", "background", "bgcolor", "border", "buffered", "capture", "challenge", "charset", "checked", "cite", "class", "code", "codebase", "color", "cols", "colspan", "content", "contenteditable", "contextmenu", "controls", "coords", "crossorigin", "csp", "data", "datetime", "decoding", "default", "defer", "dir", "dirname", "disabled", "download", "draggable", "enctype", "enterkeyhint", "for", "form", "formaction", "formenctype", "formmethod", "formnovalidate", "formtarget", "headers", "height", "hidden", "high", "href", "hreflang", "http-equiv", "icon", "id", "importance", "inputmode", "integrity", "intrinsicsize", "ismap", "itemprop", "keytype", "kind", "label", "lang", "language", "list", "loading", "loop", "low", "manifest", "max", "maxlength", "media", "method", "min", "minlength", "multiple", "muted", "name", "novalidate", "open", "optimum", "pattern", "ping", "placeholder", "poster", "preload", "radiogroup", "readonly", "referrerpolicy", "rel", "required", "reversed", "role", "rows", "rowspan", "sandbox", "scope", "scoped", "selected", "shape", "size", "sizes", "slot", "span", "spellcheck", "src", "srcdoc", "srclang", "srcset", "start", "step", "style", "summary", "tabindex", "target", "title", "translate", "type", "usemap", "value", "width", "wrap"];