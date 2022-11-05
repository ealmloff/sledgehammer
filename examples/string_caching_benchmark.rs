use sledgehammer::{builder::MaybeId, *};
use web_sys::{console, window, Performance};

fn main() {
    let perf = window().unwrap().performance().unwrap();

    let mut channel = MsgChannel::default();
    let channel_ptr: *mut _ = &mut channel;

    let bench = bench(
        &perf,
        || {
            // create an element using sledgehammer
            unsafe {
                (*channel_ptr).build_full_element(
                    ElementBuilder::new(Element::div.any_element()).id(NodeId(0)),
                );
            }
        },
        || {
            unsafe {
                (*channel_ptr).set_text("Hello from sledehammer!", MaybeId::Node(NodeId(0)));

                // execute the queued operations
                (*channel_ptr).flush();
            }
        },
    );

    console::log_1(&format!("{}", bench).into());
}

fn bench(perf: &Performance, mut setup: impl FnMut(), mut f: impl FnMut()) -> f64 {
    let mut sum = 0.0;
    const N: usize = 1000000;
    for _ in 0..N {
        setup();
        let start = perf.now();
        f();
        let end = perf.now();
        sum += end - start;
    }
    sum / N as f64
}
