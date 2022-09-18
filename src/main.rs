#![allow(non_camel_case_types)]

// mod attrs;

use dioxus_interpreter_js::Interpreter;
use smallvec::{Array, SmallVec};
use std::ops::RangeInclusive;

use wasm_bindgen::prelude::*;
use web_sys::{console, Document, HtmlHeadElement, Node, Performance};

const BATCHES: usize = 1000;
const ELEMENTS: usize = 100;
const ID: Option<u64> = Some(1);

static mut PTR: u32 = 0;
static mut PTR_PTR: *const u32 = unsafe { &PTR } as *const u32;
static mut LEN_PTR: u32 = 0;
static mut LEN_PTR_PTR: *const u32 = unsafe { &LEN_PTR } as *const u32;

#[wasm_bindgen(module = "/interpreter.js")]
extern "C" {
    fn interperter_init(mem: JsValue, ptr: u32, size: u32);

    #[wasm_bindgen(js_name = "work")]
    fn work_inner();

    fn set_node(id: u64, node: Node);

    fn bench();

    fn prep();
}

fn work(data: &[u8]) {
    let ptr = data.as_ptr();
    let len = data.len();
    unsafe {
        PTR = ptr as u32;
        LEN_PTR = len as u32
    };
    work_inner();

    let _ = data;
}

// fn bench_hand(perf: &Performance) {
//     let mut sum = 0.0;
//     for _ in 0..BATCHES {
//         prep();
//         let start = perf.now();
//         const MSG: &[u8; ELEMENTS * 8] = &[
//             0,
//             0,
//             Element::blockquote as u8,
//             0,
//             0,
//             Element::div as u8,
//             3,
//             1,
//         ];
//         let ptr = MSG.as_ptr();
//         let len = MSG.len();
//         unsafe {
//             PTR = ptr as u32;
//             LEN_PTR = len as u32
//         };
//         work_inner();
//         let end = perf.now();
//         sum += end - start;
//     }
//     console::log_1(&format!("{} msg.create_element hand", sum / BATCHES as f64).into());
// }

fn bench_msg_element(perf: &Performance) {
    let mut sum = 0.0;
    for _ in 0..BATCHES {
        prep();
        let start = perf.now();
        let mut msg = MsgBuilder::new();
        for _ in 0..ELEMENTS {
            msg.create_element(Element::blockquote, ID);
            msg.set_attribute(Attribute::hidden, true, ID);
            msg.create_element(Element::div, ID.map(|id| id + 1));
            msg.set_attribute(Attribute::class, &"test", ID.map(|id| id + 1));
            msg.append_children(1);
            msg.remove_attribute(Attribute::hidden, ID.unwrap());
            msg.create_element(Element::input, None);
            msg.insert_after(ID.unwrap() + 1, 1);
        }
        msg.build();
        let end = perf.now();
        sum += end - start;
    }
    console::log_1(&format!("{} msg.create_element", sum / BATCHES as f64).into());
}

fn bench_msg_pre_alloc(perf: &Performance) {
    const LEN: usize = 8 * ELEMENTS;
    let mut sum = 0.0;
    for _ in 0..BATCHES {
        prep();
        let start = perf.now();
        let mut msg = MsgBuilder::<LEN>::default();
        for _ in 0..ELEMENTS {
            msg.create_element(Element::blockquote, ID);
            msg.create_element(Element::div, ID.map(|id| id + 1));
            msg.append_children(1);
        }
        msg.build();
        let end = perf.now();
        sum += end - start;
    }
    console::log_1(&format!("{} msg.create_element prealoc", sum / BATCHES as f64).into());
}

fn bench_msg_element_custom(perf: &Performance) {
    let mut sum = 0.0;
    for _ in 0..BATCHES {
        prep();
        let start = perf.now();
        let mut msg = MsgBuilder::new();
        for _ in 0..ELEMENTS {
            msg.create_element("blockquote", ID);
            msg.set_attribute("hidden", true, ID);
            msg.create_element("div", ID.map(|id| id + 1));
            msg.set_attribute("class", &"test", ID.map(|id| id + 1));
            msg.append_children(1);
            // msg.remove_attribute("hidden", ID.unwrap());
            msg.create_element("input", None);
            // msg.insert_after(ID.unwrap() + 1, 1);
        }
        msg.build();
        let end = perf.now();
        sum += end - start;
    }
    console::log_1(&format!("{} msg.create_element custom", sum / BATCHES as f64).into());
}

fn bench_msg_custom_element(perf: &Performance) {
    let mut sum = 0.0;
    for _ in 0..BATCHES {
        prep();
        let start = perf.now();
        let mut msg = MsgBuilder::new();
        for _ in 0..ELEMENTS {
            msg.create_element("blockquote", None);
            msg.create_element("div", None);
            msg.append_children(1);
        }
        msg.build();
        let end = perf.now();
        sum += end - start;
    }
    console::log_1(&format!("{} msg.create_element custom", sum / BATCHES as f64).into());
}

fn bench_msg_custom_element_alloc(perf: &Performance) {
    const LEN2: usize = ("blockquote".len() + "div".len() + 8) * ELEMENTS;
    let mut sum = 0.0;
    for _ in 0..BATCHES {
        prep();
        let start = perf.now();
        let mut msg = MsgBuilder::<LEN2>::default();
        for _ in 0..ELEMENTS {
            msg.create_element("blockquote", None);
            msg.create_element("div", None);
            msg.append_children(1);
        }
        msg.build();
        let end = perf.now();
        sum += end - start;
    }
    console::log_1(&format!("{} msg.create_element custom prealoc", sum / BATCHES as f64).into());
}

fn bench_msg_set_attribute(perf: &Performance) {
    let mut sum = 0.0;
    for _ in 0..BATCHES {
        let start = perf.now();
        let mut msg = MsgBuilder::new();
        for _ in 0..ELEMENTS {
            msg.set_attribute(Attribute::alt, &"true", None);
        }
        msg.build();
        let end = perf.now();
        sum += end - start;
    }
    console::log_1(&format!("{} msg.set_attribute", sum / BATCHES as f64).into());
}

fn bench_msg_combined(perf: &Performance) {
    let mut sum = 0.0;
    for _ in 0..BATCHES {
        let start = perf.now();
        let mut msg = MsgBuilder::new();
        for _ in 0..ELEMENTS {
            msg.create_element(Element::blockquote, None);
            msg.set_attribute(Attribute::alt, &"true", None);
        }
        msg.build();
        let end = perf.now();
        sum += end - start;
    }
    console::log_1(&format!("{} msg.combined", sum / BATCHES as f64).into());
}

fn bench_set_attribute(head: &HtmlHeadElement, perf: &Performance) {
    let mut sum = 0.0;
    for _ in 0..BATCHES {
        let start = perf.now();
        for _ in 0..ELEMENTS {
            head.set_attribute("alt", "true").unwrap();
        }
        let end = perf.now();
        sum += end - start;
    }
    console::log_1(&format!("{} set_attribute", sum / BATCHES as f64).into());
}

fn bench_create_element(doc: &Document, perf: &Performance) {
    let mut sum = 0.0;
    for _ in 0..BATCHES {
        let start = perf.now();
        for _ in 0..ELEMENTS {
            let block = doc.create_element("blockquote").unwrap();
            block.set_attribute("hidden", "true").unwrap();
            let div = doc.create_element("div").unwrap();
            div.set_attribute("class", "test").unwrap();
            block.append_child(&div).unwrap();
            // div.remove_attribute("hidden").unwrap();
            let input = doc.create_element("input").unwrap();
            // div.after_with_node_1(&input).unwrap();
        }
        let end = perf.now();
        sum += end - start;
    }
    console::log_1(&format!("{} create_element (web-sys)", sum / BATCHES as f64).into());
}

fn bench_dioxus(doc: &Document, perf: &Performance) {
    let mut sum = 0.0;
    for _ in 0..BATCHES {
        let root = doc.create_element("div").unwrap();
        let interpreter = Interpreter::new(root);
        let start = perf.now();
        for _ in 0..ELEMENTS {
            interpreter.CreateElement("blockquote", 1);
            interpreter.SetAttribute(1, "hidden", "true".into(), None);
            interpreter.CreateElement("div", 2);
            interpreter.SetAttribute(1, "class", "test".into(), None);
            interpreter.AppendChildren(1);
            // interpreter.RemoveAttribute(1, "hidden", None);
            interpreter.CreateElement("input", 3);
            // interpreter.InsertAfter(2, 1);
        }
        let end = perf.now();
        sum += end - start;
    }
    console::log_1(&format!("{} dioxus", sum / BATCHES as f64).into());
}

pub fn main() {
    unsafe {
        interperter_init(wasm_bindgen::memory(), PTR_PTR as u32, LEN_PTR_PTR as u32);
    }

    let win = web_sys::window().unwrap();
    let doc = win.document().unwrap();
    let perf = win.performance().unwrap();

    for _ in 0..3 {
        // bench_dioxus(&doc, &perf);

        // bench_hand(&perf);

        bench_msg_element(&perf);

        // bench_msg_element_custom(&perf);

        // bench_msg_pre_alloc(&perf);

        // bench_msg_custom_element(&perf);

        // bench_msg_custom_element_alloc(&perf);

        // bench_create_element(&doc, &perf);

        bench();
    }
}

struct MsgBuilder<const SIZE: usize = 0> {
    buf: SmallVec<[u8; SIZE]>,
    // the number of bytes an id takes
    id_size: u8,
}

impl<const SIZE: usize> Default for MsgBuilder<SIZE> {
    fn default() -> Self {
        Self {
            buf: SmallVec::new(),
            id_size: 1,
        }
    }
}

impl MsgBuilder<10> {
    fn new() -> Self {
        Self {
            buf: SmallVec::new(),
            id_size: 1,
        }
    }
}

enum Op {
    PushRoot = 0,
    PopRoot = 1,
    AppendChildren = 2,
    ReplaceWith = 3,
    InsertBefore = 4,
    InsertAfter = 5,
    Remove = 6,
    CreateTextNode = 7,
    CreateElement = 8,
    CreateElementNs = 9,
    CreatePlaceholder = 10,
    SetEventListener = 11,
    RemoveEventListener = 12,
    SetText = 13,
    SetAttribute = 14,
    RemoveAttribute = 15,
    RemoveAttributeNs = 16,
    SetIdSize = 17,
}

impl<const SIZE: usize> MsgBuilder<SIZE> {
    pub fn check_id(&mut self, id: [u8; 8]) {
        let first_contentful_byte = id.iter().rev().position(|&b| b != 0).unwrap_or(id.len());
        let contentful_size = id.len() - first_contentful_byte;
        if contentful_size > self.id_size as usize {
            self.set_id_size(contentful_size as u8);
        }
    }

    pub fn create_element(&mut self, element: impl IntoElement, id: Option<u64>) {
        let id = id.map(|id| id.to_le_bytes());
        if let Some(id) = id {
            self.check_id(id);
        }
        self.buf.push(Op::CreateElement as u8);
        if let Some(id) = id {
            let contentful_id = &id[..self.id_size as usize];
            self.buf.extend_from_slice(&contentful_id);
        } else {
            self.buf.push(0);
        }
        element.encode(&mut self.buf);
    }

    pub fn create_element_ns(&mut self, element: impl IntoElement, ns: &str, id: Option<u64>) {
        let id = id.map(|id| id.to_le_bytes());
        if let Some(id) = id {
            self.check_id(id);
        }
        self.buf.push(Op::CreateElementNs as u8);
        if let Some(id) = id {
            let contentful_id = &id[..self.id_size as usize];
            self.buf.extend_from_slice(&contentful_id);
        } else {
            self.buf.push(0);
        }
        element.encode(&mut self.buf);
        encode_str(&mut self.buf, ns);
    }

    pub fn create_placeholder(&mut self, id: u64) {
        let id = id.to_le_bytes();
        self.check_id(id);
        self.buf.push(Op::CreatePlaceholder as u8);
        let contentful_id = &id[..self.id_size as usize];
        self.buf.extend_from_slice(&contentful_id);
    }

    pub fn create_text_node(&mut self, text: &str, id: Option<u64>) {
        let id = id.map(|id| id.to_le_bytes());
        if let Some(id) = id {
            self.check_id(id);
        }
        self.buf.push(Op::CreateTextNode as u8);
        if let Some(id) = id {
            let contentful_id = &id[..self.id_size as usize];
            self.buf.extend_from_slice(&contentful_id);
        } else {
            self.buf.push(0);
        }
        encode_str(&mut self.buf, text);
    }

    pub fn set_attribute(
        &mut self,
        attribute: impl IntoAttribue,
        value: impl IntoValue,
        id: Option<u64>,
    ) {
        let id = id.map(|id| id.to_le_bytes());
        if let Some(id) = id {
            self.check_id(id);
        }
        self.buf.push(Op::SetAttribute as u8);
        if let Some(id) = id {
            let contentful_id = &id[..self.id_size as usize];
            self.buf.extend_from_slice(&contentful_id);
        } else {
            self.buf.push(0);
        }
        attribute.encode(&mut self.buf);
        value.encode(&mut self.buf);
    }

    pub fn remove_attribute(&mut self, attribute: impl IntoAttribue, id: u64) {
        let id = id.to_le_bytes();
        self.check_id(id);
        self.buf.push(Op::RemoveAttribute as u8);
        let contentful_id = &id[..self.id_size as usize];
        self.buf.extend_from_slice(&contentful_id);
        attribute.encode(&mut self.buf);
    }

    pub fn remove_attribute_ns(&mut self, attribute: impl IntoAttribue, ns: &str, id: u64) {
        let id = id.to_le_bytes();
        self.check_id(id);
        self.buf.push(Op::RemoveAttributeNs as u8);
        let contentful_id = &id[..self.id_size as usize];
        self.buf.extend_from_slice(&contentful_id);
        attribute.encode(&mut self.buf);
        encode_str(&mut self.buf, ns);
    }

    pub fn append_children(&mut self, children: u8) {
        self.buf.push(Op::AppendChildren as u8);
        self.buf.push(children);
    }

    pub fn push_root(&mut self, id: u64) {
        let id = id.to_le_bytes();
        self.check_id(id);
        self.buf.push(Op::PushRoot as u8);
        let contentful_id = &id[..self.id_size as usize];
        self.buf.extend_from_slice(&contentful_id);
    }

    pub fn pop_root(&mut self) {
        self.buf.push(Op::PopRoot as u8);
    }

    pub fn insert_after(&mut self, id: u64, num: u32) {
        let id = id.to_le_bytes();
        self.check_id(id);
        self.buf.push(Op::InsertAfter as u8);
        let contentful_id = &id[..self.id_size as usize];
        self.buf.extend_from_slice(&contentful_id);
        self.buf.extend_from_slice(&num.to_le_bytes());
    }

    pub fn insert_before(&mut self, id: u64, num: u32) {
        let id = id.to_le_bytes();
        self.check_id(id);
        self.buf.push(Op::InsertBefore as u8);
        let contentful_id = &id[..self.id_size as usize];
        self.buf.extend_from_slice(&contentful_id);
        self.buf.extend_from_slice(&num.to_le_bytes());
    }

    pub fn remove(&mut self, id: u64) {
        let id = id.to_le_bytes();
        self.check_id(id);
        self.buf.push(Op::Remove as u8);
        let contentful_id = &id[..self.id_size as usize];
        self.buf.extend_from_slice(&contentful_id);
    }

    pub fn set_event_listener(&mut self, event: impl IntoEvent, id: u64) {
        let id = id.to_le_bytes();
        self.check_id(id);
        self.buf.push(Op::SetEventListener as u8);
        let contentful_id = &id[..self.id_size as usize];
        self.buf.extend_from_slice(&contentful_id);
        event.encode(&mut self.buf);
    }

    pub fn remove_event_listener(&mut self, event: impl IntoEvent, id: u64) {
        let id = id.to_le_bytes();
        self.check_id(id);
        self.buf.push(Op::RemoveEventListener as u8);
        let contentful_id = &id[..self.id_size as usize];
        self.buf.extend_from_slice(&contentful_id);
        event.encode(&mut self.buf);
    }

    pub fn set_id_size(&mut self, id_size: u8) {
        self.id_size = id_size;
        self.buf.push(Op::SetIdSize as u8);
        self.buf.push(id_size);
    }

    pub fn set_node(&mut self, id: u64, node: Node) {
        set_node(id, node);
    }

    pub fn replace_with(&mut self, id: u64, num: u32) {
        let id = id.to_le_bytes();
        self.check_id(id);
        self.buf.push(Op::ReplaceWith as u8);
        let contentful_id = &id[..self.id_size as usize];
        self.buf.extend_from_slice(&contentful_id);
        self.buf.extend_from_slice(&num.to_le_bytes());
    }

    pub fn set_text(&mut self, id: u64, text: &str) {
        let id = id.to_le_bytes();
        self.check_id(id);
        self.buf.push(Op::SetText as u8);
        let contentful_id = &id[..self.id_size as usize];
        self.buf.extend_from_slice(&contentful_id);
        encode_str(&mut self.buf, text);
    }

    pub fn build(&self) {
        work(&self.buf)
    }
}

fn encode_str<A: Array<Item = u8>>(buf: &mut SmallVec<A>, s: &str) {
    let b = s.as_bytes();
    let len = b.len();
    buf.push(len as u8);
    buf.extend_from_slice(b);
}

trait IntoValue {
    const LEN: RangeInclusive<Option<usize>>;

    fn len(&self) -> usize;
    fn encode<A: Array<Item = u8>>(self, v: &mut SmallVec<A>);
}

impl IntoValue for bool {
    const LEN: RangeInclusive<Option<usize>> = RangeInclusive::new(Some(1), Some(1));

    fn len(&self) -> usize {
        1
    }

    fn encode<A: Array<Item = u8>>(self, v: &mut SmallVec<A>) {
        v.push(if self { 255 } else { 0 });
    }
}

impl<S: AsRef<str>> IntoValue for &S {
    const LEN: RangeInclusive<Option<usize>> = RangeInclusive::new(Some(2), Some(256));

    fn len(&self) -> usize {
        self.as_ref().as_bytes().len()
    }

    fn encode<A: Array<Item = u8>>(self, v: &mut SmallVec<A>) {
        encode_str(v, self.as_ref());
    }
}

trait IntoElement {
    const LEN: RangeInclusive<Option<usize>>;

    fn len(&self) -> usize;
    fn encode<A: Array<Item = u8>>(self, v: &mut SmallVec<A>);
}

impl IntoElement for Element {
    const LEN: RangeInclusive<Option<usize>> = Some(1)..=Some(1);

    fn len(&self) -> usize {
        1
    }

    fn encode<A: Array<Item = u8>>(self, v: &mut SmallVec<A>) {
        v.push(self as u8)
    }
}

impl<S: AsRef<str>> IntoElement for S {
    const LEN: RangeInclusive<Option<usize>> = Some(2)..=None;

    fn len(&self) -> usize {
        self.as_ref().len() + 2
    }

    fn encode<A: Array<Item = u8>>(self, v: &mut SmallVec<A>) {
        v.push(255);
        encode_str(v, self.as_ref());
    }
}

trait IntoAttribue {
    const LEN: RangeInclusive<Option<usize>>;

    fn len(&self) -> usize;
    fn encode<A: Array<Item = u8>>(self, v: &mut SmallVec<A>);
}

impl IntoAttribue for Attribute {
    const LEN: RangeInclusive<Option<usize>> = Some(1)..=Some(1);

    fn len(&self) -> usize {
        1
    }

    fn encode<A: Array<Item = u8>>(self, v: &mut SmallVec<A>) {
        v.push(self as u8)
    }
}

impl<S: AsRef<str>> IntoAttribue for S {
    const LEN: RangeInclusive<Option<usize>> = Some(2)..=None;

    fn len(&self) -> usize {
        self.as_ref().len()
    }

    fn encode<A: Array<Item = u8>>(self, v: &mut SmallVec<A>) {
        v.push(255);
        encode_str(v, self.as_ref());
    }
}

trait IntoEvent {
    const LEN: RangeInclusive<Option<usize>>;

    fn len(&self) -> usize;
    fn encode<A: Array<Item = u8>>(self, v: &mut SmallVec<A>);
}

impl IntoEvent for Event {
    const LEN: RangeInclusive<Option<usize>> = Some(1)..=Some(1);

    fn len(&self) -> usize {
        1
    }

    fn encode<A: Array<Item = u8>>(self, v: &mut SmallVec<A>) {
        v.push(self as u8)
    }
}

impl<S: AsRef<str>> IntoEvent for S {
    const LEN: RangeInclusive<Option<usize>> = Some(2)..=None;

    fn len(&self) -> usize {
        self.as_ref().len()
    }

    fn encode<A: Array<Item = u8>>(self, v: &mut SmallVec<A>) {
        v.push(255);
        encode_str(v, self.as_ref());
    }
}

pub enum Element {
    a,
    abbr,
    acronym,
    address,
    applet,
    area,
    article,
    aside,
    audio,
    b,
    base,
    bdi,
    bdo,
    bgsound,
    big,
    blink,
    blockquote,
    body,
    br,
    button,
    canvas,
    caption,
    center,
    cite,
    code,
    col,
    colgroup,
    content,
    data,
    datalist,
    dd,
    del,
    details,
    dfn,
    dialog,
    dir,
    div,
    dl,
    dt,
    em,
    embed,
    fieldset,
    figcaption,
    figure,
    font,
    footer,
    form,
    frame,
    frameset,
    head,
    header,
    h1,
    hgroup,
    hr,
    html,
    i,
    iframe,
    image,
    img,
    input,
    ins,
    kbd,
    keygen,
    label,
    legend,
    li,
    link,
    main,
    map,
    mark,
    marquee,
    menu,
    menuitem,
    meta,
    meter,
    nav,
    nobr,
    noembed,
    noframes,
    noscript,
    object,
    ol,
    optgroup,
    option,
    output,
    p,
    param,
    picture,
    plaintext,
    portal,
    pre,
    progress,
    q,
    rb,
    rp,
    rt,
    rtc,
    ruby,
    s,
    samp,
    script,
    section,
    select,
    shadow,
    slot,
    small,
    source,
    spacer,
    span,
    strike,
    strong,
    style,
    sub,
    summary,
    sup,
    table,
    tbody,
    td,
    template,
    textarea,
    tfoot,
    th,
    thead,
    time,
    title,
    tr,
    track,
    tt,
    u,
    ul,
    var,
    video,
    wbr,
    xmp,
}

pub enum Attribute {
    accesskey,
    action,
    align,
    allow,
    alt,
    r#async,
    autocapitalize,
    autocomplete,
    autofocus,
    autoplay,
    background,
    bgcolor,
    border,
    buffered,
    capture,
    challenge,
    charset,
    checked,
    cite,
    class,
    code,
    codebase,
    color,
    cols,
    colspan,
    content,
    contenteditable,
    contextmenu,
    controls,
    coords,
    crossorigin,
    csp,
    data,
    datetime,
    decoding,
    default,
    defer,
    dir,
    dirname,
    disabled,
    download,
    draggable,
    enctype,
    enterkeyhint,
    r#for,
    form,
    formaction,
    formenctype,
    formmethod,
    formnovalidate,
    formtarget,
    headers,
    height,
    hidden,
    high,
    href,
    hreflang,
    r#http_equiv,
    icon,
    id,
    importance,
    inputmode,
    integrity,
    intrinsicsize,
    ismap,
    itemprop,
    keytype,
    kind,
    label,
    lang,
    language,
    list,
    loading,
    r#loop,
    low,
    manifest,
    max,
    maxlength,
    media,
    method,
    min,
    minlength,
    multiple,
    muted,
    name,
    novalidate,
    open,
    optimum,
    pattern,
    ping,
    placeholder,
    poster,
    preload,
    radiogroup,
    readonly,
    referrerpolicy,
    rel,
    required,
    reversed,
    role,
    rows,
    rowspan,
    sandbox,
    scope,
    scoped,
    selected,
    shape,
    size,
    sizes,
    slot,
    span,
    spellcheck,
    src,
    srcdoc,
    srclang,
    srcset,
    start,
    step,
    style,
    summary,
    tabindex,
    target,
    title,
    translate,
    r#type,
    usemap,
    value,
    width,
    wrap,
}

pub enum Event {
    abort,
    activate,
    addstream,
    addtrack,
    afterprint,
    afterscriptexecute,
    animationcancel,
    animationend,
    animationiteration,
    animationstart,
    appinstalled,
    audioend,
    audioprocess,
    audiostart,
    auxclick,
    beforeinput,
    beforeprint,
    beforescriptexecute,
    beforeunload,
    beginEvent,
    blocked,
    blur,
    boundary,
    bufferedamountlow,
    cancel,
    canplay,
    canplaythrough,
    change,
    click,
    close,
    closing,
    complete,
    compositionend,
    compositionstart,
    compositionupdate,
    connect,
    connectionstatechange,
    contentdelete,
    contextmenu,
    copy,
    cuechange,
    cut,
    datachannel,
    dblclick,
    devicechange,
    devicemotion,
    deviceorientation,
    DOMActivate,
    DOMContentLoaded,
    DOMMouseScroll,
    drag,
    dragend,
    dragenter,
    dragleave,
    dragover,
    dragstart,
    drop,
    durationchange,
    emptied,
    end,
    ended,
    endEvent,
    enterpictureinpicture,
    error,
    focus,
    focusin,
    focusout,
    formdata,
    fullscreenchange,
    fullscreenerror,
    gamepadconnected,
    gamepaddisconnected,
    gatheringstatechange,
    gesturechange,
    gestureend,
    gesturestart,
    gotpointercapture,
    hashchange,
    icecandidate,
    icecandidateerror,
    iceconnectionstatechange,
    icegatheringstatechange,
    input,
    inputsourceschange,
    install,
    invalid,
    keydown,
    keypress,
    keyup,
    languagechange,
    leavepictureinpicture,
    load,
    loadeddata,
    loadedmetadata,
    loadend,
    loadstart,
    lostpointercapture,
    mark,
    merchantvalidation,
    message,
    messageerror,
    mousedown,
    mouseenter,
    mouseleave,
    mousemove,
    mouseout,
    mouseover,
    mouseup,
    mousewheel,
    msContentZoom,
    MSGestureChange,
    MSGestureEnd,
    MSGestureHold,
    MSGestureStart,
    MSGestureTap,
    MSInertiaStart,
    MSManipulationStateChanged,
    mute,
    negotiationneeded,
    nomatch,
    notificationclick,
    offline,
    online,
    open,
    orientationchange,
    pagehide,
    pageshow,
    paste,
    pause,
    payerdetailchange,
    paymentmethodchange,
    play,
    playing,
    pointercancel,
    pointerdown,
    pointerenter,
    pointerleave,
    pointerlockchange,
    pointerlockerror,
    pointermove,
    pointerout,
    pointerover,
    pointerup,
    popstate,
    progress,
    push,
    pushsubscriptionchange,
    ratechange,
    readystatechange,
    rejectionhandled,
    removestream,
    removetrack,
    removeTrack,
    repeatEvent,
    reset,
    resize,
    resourcetimingbufferfull,
    result,
    resume,
    scroll,
    search,
    seeked,
    seeking,
    select,
    selectedcandidatepairchange,
    selectend,
    selectionchange,
    selectstart,
    shippingaddresschange,
    shippingoptionchange,
    show,
    signalingstatechange,
    slotchange,
    soundend,
    soundstart,
    speechend,
    speechstart,
    squeeze,
    squeezeend,
    squeezestart,
    stalled,
    start,
    statechange,
    storage,
    submit,
    success,
    suspend,
    timeout,
    timeupdate,
    toggle,
    tonechange,
    touchcancel,
    touchend,
    touchmove,
    touchstart,
    track,
    transitioncancel,
    transitionend,
    transitionrun,
    transitionstart,
    unhandledrejection,
    unload,
    unmute,
    upgradeneeded,
    versionchange,
    visibilitychange,
    voiceschanged,
    volumechange,
    vrdisplayactivate,
    vrdisplayblur,
    vrdisplayconnect,
    vrdisplaydeactivate,
    vrdisplaydisconnect,
    vrdisplayfocus,
    vrdisplaypointerrestricted,
    vrdisplaypointerunrestricted,
    vrdisplaypresentchange,
    waiting,
    webglcontextcreationerror,
    webglcontextlost,
    webglcontextrestored,
    webkitmouseforcechanged,
    webkitmouseforcedown,
    webkitmouseforceup,
    webkitmouseforcewillbegin,
    wheel,
}
