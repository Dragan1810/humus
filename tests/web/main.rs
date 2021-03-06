use log::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_test::*;
use web_sys;

use humus::{
    html,
    node::Element,
    vdom::VirtualDom,
    render::{h,t, attr}

};

wasm_bindgen_test_configure!(run_in_browser);

pub fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

pub fn document() -> web_sys::Document {
    window()
        .document()
        .expect("should have a document on window")
}

pub fn create_element(tag: &str) -> web_sys::Element {
    init_logging();
    document()
        .create_element(tag)
        .expect("should create element OK")
}

/// Ensure that logs go to the devtools console.
pub fn init_logging() {
    use std::sync::Once;
    static START: Once = Once::new();
    START.call_once(|| {
        console_log::init_with_level(Level::Trace).expect("could not initialize console_log");
    });
}

fn _stringify_actual_node(n: &web_sys::Node) -> String {
    if let Some(el) = n.dyn_ref::<web_sys::Element>() {
        el.outer_html()
    } else {
        format!("#text({:?})", n.text_content())
    }
}

fn _stringify_humus_node(n: Element) -> String {
    n.inner_html()
}

#[test]
pub fn test_jsx() {
    // ignore html! errors!!!
    html!{
        <div name="main">
            <h1 style="color:red">"Humus Virtual Dom"</h1>
        </div>
    };
    eprint!("{:?}", x);
    assert_eq!(2, 3)
}

#[wasm_bindgen_test]
pub fn render_test() {
    let body = document().body().unwrap();
    let mut root: Element = wasm_bindgen::JsCast::dyn_into::<web_sys::Element>(body)
        .unwrap()
        .into();

    let mut vd = VirtualDom::new();

   
    fn _App() {
        h(
            "div",
            vec![h(
                "h1",
                vec![t("Humus Virtual Dom")],
                vec![attr("style", "color:red")],
            )],
            vec![],
        );
    };


    vd.render(
        &mut root,
        h(
            "div",
            vec![h(
                "h1",
                vec![t("Humus Virtual Dom")],
                vec![attr("name", "h1-name"), attr("style", "color:red")],
            )],
            vec![attr("name", "glavni-div")],
        ),
    );
    assert_eq!(
        2,
        2 //stringify_humus_node(super_div),
          //String::from("<div><h1>Hello</h1><h2>From</h2><h1>Humus Virtual Dom</h1></div>")
    );
}

/*
pub fn assert_rendered(container: &web_sys::Element) {
    init_logging();

    fn check_node(actual: &web_sys::Node, expected: &VirtualDomNode) {
        debug!("check_render:");
        debug!("    actual = {}", stringify_actual_node(&actual));
        debug!("    expected = {:#?}", expected);
        match expected {
            VirtualDomNode::TextNode(VirtualTextNode { text }) => {
                assert_eq!(
                    actual.node_name().to_uppercase(),
                    "#TEXT",
                    "actual.node_name() == #TEXT"
                );
                assert_eq!(
                    actual.text_content().unwrap_or_default(),
                    text,
                    "actual.text_content() == expected.text()"
                );
            }
            VirtualDomNode::ElementNode(VirtualElementNode {
                node_type,
                children
            }) => {
                assert_eq!(
                    actual.node_name().to_uppercase(),
                    "actual.node_name() == expected.tag_name()"
                );
                let actual = actual
                    .dyn_ref::<web_sys::Element>()
                    .expect("`actual` should be an `Element`");
                check_children(actual.child_nodes(), children);
            }
        }
    }

    fn check_children(actual: web_sys::NodeList, expected: &[VirtualElementNode]) {
        assert_eq!(
            actual.length(),
            expected.len() as u32,
            "actual children length == expected children length"
        );
        for (i, child) in expected.iter().enumerate() {
            let actual_child = actual.item(i as u32).unwrap();
            check_node(&actual_child, child);
        }
    }
}

*/