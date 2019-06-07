use super::node::{VirtualDomNode, VirtualElementNode, VirtualTextNode};
use wasm_bindgen::prelude::*;
use web_sys::Element;


pub fn h(node_type: &str, children: Vec<VirtualDomNode>) -> VirtualDomNode {
    VirtualDomNode::VirtualElementNode(VirtualElementNode {
        node_type: String::from(node_type),
        children: children,
    })
}

pub fn t(text: &str) -> VirtualDomNode {
    VirtualDomNode::VirtualTextNode(VirtualTextNode {
        text: String::from(text),
    })
}

pub fn create_element_from_node(node: &VirtualDomNode) -> Result<Element, JsValue> {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("Document");


    match node {
        VirtualDomNode::VirtualElementNode(vnode) => {

            let el = document.create_element(&vnode.node_type);
            // Recursively create child nodes as well

            /*
            for c in vnode.children.iter() {
                let child_element = create_element_from_node(c);
                append_element(el,child_element);
            }
            */

            el
        }
        VirtualDomNode::VirtualTextNode(text_node) => {
            let _text = document.create_text_node(&text_node.text);
            let el = document.create_element("p");
            el
        }
        VirtualDomNode::Empty => {
            let el = document.create_element("p");
            el
        }
    }

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
