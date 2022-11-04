use sledgehammer::{builder::MaybeId, *};
use wasm_bindgen::JsCast;

fn main() {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let body = document.body().unwrap();

    // create an element using web-sys
    let div = document.create_element("div").unwrap();
    let web_sys_element = document.create_element("p").unwrap();
    web_sys_element
        .set_attribute("style", "color: blue")
        .unwrap();
    web_sys_element.set_text_content(Some("Hello from web-sys!"));
    let svg = document
        .create_element_ns(Some("http://www.w3.org/2000/svg"), "svg")
        .unwrap();
    svg.set_attribute_ns(Some("http://www.w3.org/2000/svg"), "width", "100%")
        .unwrap();
    div.append_child(&web_sys_element).unwrap();
    div.append_child(&svg).unwrap();

    // append the new node to the body
    body.append_child(&div).unwrap();

    let mut channel = MsgChannel::default();

    // assign the NodeId(0) to the body element from web-sys
    channel.set_node(NodeId(0), JsCast::dyn_into(body).unwrap());

    // create an element using sledgehammer
    channel.build_full_element(
        ElementBuilder::new(Element::div.any_element())
            .id(NodeId(1))
            .children(&[
                ElementBuilder::new(Element::p.any_element())
                    .id(NodeId(2))
                    .attrs(&[(Attribute::style.any_attr(), "color: blue")]),
                ElementBuilder::new(
                    "svg"
                        .in_namespace("http://www.w3.org/2000/svg")
                        .any_element(),
                )
                .attrs(&[(
                    "width"
                        .in_namespace("http://www.w3.org/2000/svg")
                        .any_attr(),
                    "100%",
                )]),
            ]),
    );

    channel.set_text("Hello from sledehammer!", MaybeId::Node(NodeId(2)));

    // append the new node to the body
    channel.append_child(MaybeId::Node(NodeId(0)), NodeId(1));

    // execute the queued operations
    channel.flush();

    // we can also get web-sys nodes out of sledgehammer
    let element = channel.get_node(NodeId(2));
    let text = element.text_content().map(|t| t + " + web-sys");
    element.set_text_content(text.as_deref());
}
