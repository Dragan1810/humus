use std::fmt;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::EventTarget;


pub struct Node {
    pub node: Option<web_sys::Node>,
}

impl From<web_sys::Node> for Node {
    fn from(node: web_sys::Node) -> Node {
        Node { node: Some(node) }
    }
}

/// VirtualElementNode represents an html element
//#[derive(Debug, Clone)]
pub struct VirtualElementNode {
    pub node_type: String,
    pub children: Vec<VirtualDomNode>,
    pub attributes: Vec<Attribute>,
}

impl fmt::Debug for VirtualElementNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Node_type: {} childrens: {}",
            self.node_type,
            self.children.len()
        )
    }
}

/// VirtualTextNode represents text that is mixed in with elements
#[derive(Debug, Clone)]
pub struct VirtualTextNode {
    pub text: String,
}

/// We use an enumeration to represent these two plus an empty DOM node to represent nothing
//#[derive(Debug, Clone)]
pub enum VirtualDomNode {
    Empty,
    ElementNode(VirtualElementNode),
    TextNode(VirtualTextNode),
}
#[derive(Debug, Clone)]
pub struct Element {
    pub el: Option<web_sys::Element>,
}

impl From<web_sys::Element> for Element {
    fn from(el: web_sys::Element) -> Element {
        Element { el: Some(el) }
    }
}

impl From<web_sys::EventTarget> for Element {
    fn from(el: web_sys::EventTarget) -> Element {
        let el = wasm_bindgen::JsCast::dyn_into::<web_sys::Element>(el);
        Element { el: el.ok() }
    }
}

impl From<Element> for Option<web_sys::Node> {
    fn from(obj: Element) -> Option<web_sys::Node> {
        if let Some(el) = obj.el {
            Some(el.into())
        } else {
            None
        }
    }
}

impl From<Element> for Option<EventTarget> {
    fn from(obj: Element) -> Option<EventTarget> {
        if let Some(el) = obj.el {
            Some(el.into())
        } else {
            None
        }
    }
}


impl Element {
    pub fn create_element(tag: &str) -> Option<Element> {
        if let Some(el) = web_sys::window()?.document()?.create_element(tag).ok() {
            Some(el.into())
        } else {
            None
        }
    }

    pub fn set_attribute(&self, name: &str, value: &str) -> Result<(), JsValue> {
        if let Some(el) = self.el.as_ref() {
            el.set_attribute(name, value)
        } else {
            Err(JsValue::null())
        }
    }

    pub fn query_selector(selector: &str) -> Option<Element> {
        let body: web_sys::Element = web_sys::window()?.document()?.body()?.into();
        let el = body.query_selector(selector).ok()?;
        Some(Element { el })
    }

    pub fn query_selector_from(&mut self, selector: &str) -> Option<Element> {
        let mut found_el = None;
        if let Some(el) = self.el.as_ref() {
            found_el = Some(Element {
                el: el.query_selector(selector).ok()?,
            });
        }
        found_el
    }

    pub fn set_text_content(&mut self, value: &str) {
        if let Some(el) = self.el.as_ref() {
            if let Some(node) = &el.dyn_ref::<web_sys::Node>() {
                node.set_text_content(Some(&value));
            }
        }
    }

    pub fn append_child(&mut self, child: &mut Element) {
        if let Some(el) = self.el.as_ref() {
            if let Some(node) = &el.dyn_ref::<web_sys::Node>() {
                if let Some(ref child_el) = child.el {
                    if let Some(child_node) = child_el.dyn_ref::<web_sys::Node>() {
                        node.append_child(child_node).unwrap();
                    }
                }
            }
        }
    }
    /// Gets the text content of the `self.el` element
    pub fn text_content(&mut self) -> Option<String> {
        let mut text = None;
        if let Some(el) = self.el.as_ref() {
            if let Some(node) = &el.dyn_ref::<web_sys::Node>() {
                text = node.text_content();
            }
        }
        text
    }

    pub fn outer_html(self) -> String {
        self.el.unwrap().outer_html()
    }

    pub fn inner_html(self) -> String {
        self.el.unwrap().inner_html()
    }

    /// Given another `Element` it will remove that child from the DOM from this element
    /// Consumes `child` so it can't be used after it's removal.
    pub fn remove_child(&mut self, mut child: Element) {
        if let Some(child_el) = child.el.take() {
            if let Some(el) = self.el.take() {
                if let Some(el_node) = el.dyn_ref::<web_sys::Node>() {
                    let child_node: web_sys::Node = child_el.into();
                    el_node.remove_child(&child_node).unwrap();
                }
                self.el = Some(el);
            }
        }
    }

    pub fn add_event_listener<T>(&mut self, event_name: &str, handler: T)
    where
        T: 'static + FnMut(web_sys::Event),
    {
        let cb = Closure::wrap(Box::new(handler) as Box<dyn FnMut(_)>);
        if let Some(el) = self.el.take() {
            let el_et: EventTarget = el.into();
            el_et
                .add_event_listener_with_callback(event_name, cb.as_ref().unchecked_ref())
                .unwrap();
            cb.forget();

            if let Ok(el) = el_et.dyn_into::<web_sys::Element>() {
                self.el = Some(el);
            }
        }
    }


    /// Delegate an event to a selector
    pub fn delegate<T>(
        &mut self,
        selector: &'static str,
        event: &str,
        mut handler: T,
        use_capture: bool,
    ) where
        T: 'static + FnMut(web_sys::Event) -> (),
    {
        let el = match self.el.take() {
            Some(e) => e,
            None => return,
        };
        if let Some(dyn_el) = &el.dyn_ref::<EventTarget>() {
            if let Some(window) = web_sys::window() {
                if let Some(document) = window.document() {
                    // TODO document selector to the target element
                    let tg_el = document;

                    let cb = Closure::wrap(Box::new(move |event: web_sys::Event| {
                        if let Some(target_element) = event.target() {
                            let dyn_target_el: Option<&web_sys::Node> =
                                wasm_bindgen::JsCast::dyn_ref(&target_element);
                            if let Some(target_element) = dyn_target_el {
                                if let Ok(potential_elements) = tg_el.query_selector_all(selector) {
                                    let mut has_match = false;
                                    for i in 0..potential_elements.length() {
                                        if let Some(el) = potential_elements.get(i) {
                                            if target_element.is_equal_node(Some(&el)) {
                                                has_match = true;
                                            }
                                            break;
                                        }
                                    }

                                    if has_match {
                                        handler(event);
                                    }
                                }
                            }
                        }
                    }) as Box<dyn FnMut(_)>);

                    dyn_el
                        .add_event_listener_with_callback_and_bool(
                            event,
                            cb.as_ref().unchecked_ref(),
                            use_capture,
                        )
                        .unwrap();
                    cb.forget(); // TODO cycle collect
                }
            }
        }
        self.el = Some(el);
    }

    /// Removes a class list item from the element
    ///
    /// ```
    /// e.class_list_remove(String::from("clickable"));
    /// // removes the class 'clickable' from e.el
    /// ```
    pub fn class_list_remove(&mut self, value: &str) {
        if let Some(el) = self.el.take() {
            el.class_list().remove_1(&value).unwrap();
            self.el = Some(el);
        }
    }

    pub fn class_list_add(&mut self, value: &str) {
        if let Some(el) = self.el.take() {
            el.class_list().add_1(&value).unwrap();
            self.el = Some(el);
        }
    }

    /// Sets the whole class value for `self.el`
    pub fn set_class_name(&mut self, class_name: &str) {
        if let Some(el) = self.el.take() {
            el.set_class_name(&class_name);
            self.el = Some(el);
        }
    }
}

/// An attribute on a DOM node, such as `id="my-thing"` or
/// `href="https://example.com"`.
#[derive(Clone, Debug)]
pub struct Attribute {
    pub(crate) name: String,
    pub(crate) value: String,
}

impl Attribute {
    /// Get this attribute's name, such as `"id"` in `<div id="my-thing" />`.
    #[inline]
    pub fn name(&self) -> String {
        self.name.clone()
    }

    /// The attribute value, such as `"my-thing"` in `<div id="my-thing" />`.
    #[inline]
    pub fn value(&self) -> String {
        self.value.clone()
    }
    /*
    /// Certain attributes are considered "volatile" and can change via user
    /// input that we can't see when diffing against the old virtual DOM. For
    /// these attributes, we want to always re-set the attribute on the physical
    /// DOM node, even if the old and new virtual DOM nodes have the same value.
    #[inline]
    pub(crate) fn _is_volatile(&self) -> bool {
        match self.name {
            "value" | "checked" | "selected" => true,
            _ => false,
        }
    }
    */
}

/*

    #[derive(Debug, Clone)]
    pub(crate) enum NodeKind<'a> {
        Element(&'a ElementNode<'a>),
    }

    #[derive(Debug, Clone)]
    pub(crate) struct ElementNode<'a> {
        pub key: NodeKey,
        pub tag_name: &'a str,
        pub attributes: &'a [Attribute<'a>],
        pub children: &'a [Node<'a>],
        pub namespace: Option<&'a str>,
    }


/// The key for keyed children.
///
/// Keys must be unique among siblings.
///
/// If any sibling is keyed, then they all must be keyed.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct NodeKey(pub(crate) u32);

impl Default for NodeKey {
    fn default() -> NodeKey {
        NodeKey::NONE
    }
}

impl NodeKey {
    /// The default, lack of a key.
    pub const NONE: NodeKey = NodeKey(u32::MAX);

    /// Is this key `NodeKey::NONE`?
    #[inline]
    pub fn is_none(&self) -> bool {
        *self == Self::NONE
    }

    /// Is this key not `NodeKey::NONE`?
    #[inline]
    pub fn is_some(&self) -> bool {
        !self.is_none()
    }

    /// Create a new `NodeKey`.
    ///
    /// `key` must not be `u32::MAX`.
    #[inline]
    pub fn new(key: u32) -> Self {
        debug_assert_ne!(key, u32::MAX);
        NodeKey(key)
    }
}

impl<'a> Node<'a> {
    /// Low-level constructor for making a new `Node` of type element with given
    /// parts.
    ///
    /// This is primarily intended for JSX and templating proc-macros to compile
    /// down into. If you are building nodes by-hand, prefer using the
    /// `dodrio::builder::*` APIs.
    #[inline]
    pub fn element(
        bump: &'a Bump,
        key: NodeKey,
        tag_name: &'a str,
        attributes: &'a [Attribute<'a>],
        children: &'a [Node<'a>],
        namespace: Option<&'a str>,
    ) -> Node<'a> {
        let element = bump.alloc_with(|| ElementNode {
            key,
            tag_name,
            attributes,
            children,
            namespace,
        });

        Node {
            kind: NodeKind::Element(element),
        }
    }

    /// Construct a new text node with the given text.
    #[inline]
    pub(crate) fn text(text: &'a str) -> Node<'a> {
        Node {
            kind: NodeKind::Text(TextNode { text }),
        }
    }

    #[inline]
    pub(crate) fn key(&self) -> NodeKey {
        match &self.kind {
            NodeKind::Text(_) => NodeKey::NONE,
            NodeKind::Element(e) => e.key,
        }
    }
}

/// A node can become an iterator that yields the node itself once.
///
/// This implementation of `IntoIterator` mostly exists to improve the
/// `typed-html` ergonomics, where the macro invokes `.into_iter()` on the child
/// contents of a tag. By implementing `IntoIterator` here, we avoid having to
/// do nasty shenanigans like `<p>vec![$contents]</p>` instead of plain old
/// `<p>$contents</p>`.
impl<'a> IntoIterator for Node<'a> {
    type Item = Node<'a>;
    type IntoIter = iter::Once<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        iter::once(self)
    }
}


 */
