use super::node::{Attribute, Element, VirtualDomNode, VirtualElementNode, VirtualTextNode};
use web_sys::console;

pub fn h(node_type: &str, children: Vec<VirtualDomNode>, attr: Vec<Attribute>) -> VirtualDomNode {
    VirtualDomNode::ElementNode(VirtualElementNode {
        node_type: String::from(node_type),
        children,
        attributes: attr,
    })
}

pub fn attr(attribute: &str, value: &str) -> Attribute {
    Attribute {
        name: String::from(attribute),
        value: String::from(value),
    }
}

pub fn t(text: &str) -> VirtualDomNode {
    VirtualDomNode::TextNode(VirtualTextNode {
        text: String::from(text),
    })
}

pub fn create_element_from_node(parent: &mut Element, node: &VirtualDomNode) -> Option<Element> {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("Document");

    match node {
        VirtualDomNode::ElementNode(vnode) => {
            let mut el: Element = document
                .create_element(&vnode.node_type)
                .ok()
                .unwrap()
                .into();

            for attr in vnode.attributes.iter() {
                let _res = el.set_attribute(&attr.name, &attr.value);
                console::log_1(&"Setting attribute".into());
            }

            for c in vnode.children.iter() {
                let child_element: Option<Element> = create_element_from_node(&mut el, c);
                match child_element {
                    Some(mut x) => el.append_child(&mut x),
                    None => console::log_1(&"Emprty stuff".into()),
                } // vrati element
            }

            parent.append_child(&mut el);
            Some(el)
        }
        VirtualDomNode::TextNode(text_node) => {
            parent.set_text_content(&text_node.text);
            None
        }
        VirtualDomNode::Empty => {
            let mut el: Element = document.create_element("div").ok().unwrap().into();
            el.set_text_content("empty");
            Some(el)
        }
    }
}

pub fn update_element(
    parent: &mut Element,
    _child_index: usize,
    new_node: &VirtualDomNode,
    _old_node: &VirtualDomNode,
) {
    let _child = create_element_from_node(parent, &new_node);
}
