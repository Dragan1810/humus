use super::node::{Element, VirtualDomNode, VirtualElementNode, VirtualTextNode};

pub fn h(node_type: &str, children: Vec<VirtualDomNode>) -> VirtualDomNode {
    VirtualDomNode::ElementNode(VirtualElementNode {
        node_type: String::from(node_type),
        children,
    })
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

            for c in vnode.children.iter() {
                let mut child_element: Element = create_element_from_node(&mut el, c).unwrap(); // vrati element
                el.append_child(&mut child_element)
            }

            parent.append_child(&mut el);
            Some(el)
        }
        VirtualDomNode::TextNode(text_node) => {
            // let _text = document.create_text_node(&text_node.text); ??
            // let mut el: Element = document.create_element("p").ok().unwrap().into();
            parent.set_text_content(&text_node.text);
            None

        }
        VirtualDomNode::Empty => {
            // let mut el: Element = document.create_element("div").ok().unwrap().into();
            parent.set_text_content("empty");
            None

        }
    }

}

pub fn update_element(
    parent: &mut Element, // body stuff?
    _child_index: usize,
    new_node: &VirtualDomNode,
    _old_node: &VirtualDomNode,
) {
    let _child = create_element_from_node(parent, &new_node);
    // parent.append_child(&mut child)
}

/*
use crate::{Node, RenderContext};
use std::any::Any;
use std::rc::Rc;
use wasm_bindgen::UnwrapThrowExt;

/// A trait for any component that can be rendered to HTML.
///
/// Takes a shared reference to `self` and generates the virtual DOM that
/// represents its rendered HTML.
///
/// ## `Bump` Allocation
///
/// `Render` implementations can use the `Bump` inside the provided
/// `RenderContext` for very fast allocation for anything that needs to be
/// temporarily allocated during rendering.
///
/// ## Example
///
/// ```no_run
/// use dodrio::{Node, Render, RenderContext};
///
/// pub struct MyComponent;
///
/// impl Render for MyComponent {
///     fn render<'a>(&self, cx: &mut RenderContext<'a>) -> Node<'a> {
///         use dodrio::builder::*;
///
///         p(&cx)
///             .children([
///                 text("This is "),
///                 strong(&cx).children([text("my component")]).finish(),
///                 text(" rendered!"),
///             ])
///             .finish()
///     }
/// }
/// ```
pub trait Render {
    /// Render `self` as a virtual DOM. Use the given context's `Bump` for
    /// temporary allocations.
    fn render<'a>(&self, cx: &mut RenderContext<'a>) -> Node<'a>;
}

impl<'r, R> Render for &'r R
where
    R: Render,
{
    fn render<'a>(&self, cx: &mut RenderContext<'a>) -> Node<'a> {
        (**self).render(cx)
    }
}

impl<R> Render for Rc<R>
where
    R: Render,
{
    fn render<'a>(&self, cx: &mut RenderContext<'a>) -> Node<'a> {
        (**self).render(cx)
    }
}

/// A `RootRender` is a render component that can be the root rendering component
/// mounted to a virtual DOM.
///
/// In addition to rendering, it must also be `'static` so that it can be owned
/// by the virtual DOM and `Any` so that it can be downcast to its concrete type
/// by event listener callbacks.
///
/// You do not need to implement this trait by hand: there is a blanket
/// implementation for all `Render` types that fulfill the `RootRender`
/// requirements.
pub trait RootRender: Any + Render {
    /// Get this `&RootRender` trait object as an `&Any` trait object reference.
    fn as_any(&self) -> &Any;

    /// Get this `&mut RootRender` trait object as an `&mut Any` trait object
    /// reference.
    fn as_any_mut(&mut self) -> &mut Any;
}

impl<T> RootRender for T
where
    T: Any + Render,
{
    fn as_any(&self) -> &Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut Any {
        self
    }
}

impl dyn RootRender {
    /// Downcast this shared `&dyn RootRender` trait object reference to its
    /// underlying concrete type.
    ///
    /// # Panics
    ///
    /// Panics if this virtual DOM's root rendering component is not an `R`
    /// instance.
    pub fn unwrap_ref<R>(&self) -> &R
    where
        R: RootRender,
    {
        self.as_any()
            .downcast_ref::<R>()
            .expect_throw("bad `RootRender::unwrap_ref` call")
    }

    /// Downcast this exclusive `&mut dyn RootRender` trait object reference to
    /// its underlying concrete type.
    ///
    /// # Panics
    ///
    /// Panics if this virtual DOM's root rendering component is not an `R`
    /// instance.
    pub fn unwrap_mut<R>(&mut self) -> &mut R
    where
        R: RootRender,
    {
        self.as_any_mut()
            .downcast_mut::<R>()
            .expect_throw("bad `RootRender::unwrap_ref` call")
    }
}

 */
