// use web_sys::{Node, Window, Element, Text};

/// VirtualElementNode represents an html element
pub struct VirtualElementNode {
    pub node_type: String,
    pub children: Vec<VirtualDomNode>
}

/// VirtualTextNode represents text that is mixed in with elements
pub struct VirtualTextNode {
    pub text: String,
}

/// We use an enumeration to represent these two plus an empty DOM node to represent nothing
pub enum VirtualDomNode {
    Empty,
    VirtualElementNode(VirtualElementNode),
    VirtualTextNode(VirtualTextNode),
}



/*
use crate::{ RootRender };
use bumpalo::Bump;
use std::fmt;
use std::iter;
use std::mem;
use std::u32;

/// A virtual DOM node.
#[derive(Debug, Clone)]
pub struct Node<'a> {
    pub(crate) kind: NodeKind<'a>,
}

    /// A node is either a text node or an element.
    #[derive(Debug, Clone)]
    pub(crate) enum NodeKind<'a> {
        /// A text node.
        Text(TextNode<'a>),

        /// An element potentially with attributes and children.
        Element(&'a ElementNode<'a>),

        // A node in the vdom's `CachedSet`. This allows us to avoid
        // re-rendering and re-diffing subtrees.
    }

    /// Text nodes are just a string of text. They cannot have attributes or
    /// children.
    #[derive(Debug, Clone)]
    pub(crate) struct TextNode<'a> {
        pub text: &'a str,
    }

    /// Elements have a tag name, zero or more attributes, and zero or more
    /// children.
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





/// An attribute on a DOM node, such as `id="my-thing"` or
/// `href="https://example.com"`.
#[derive(Clone, Debug)]
pub struct Attribute<'a> {
    pub(crate) name: &'a str,
    pub(crate) value: &'a str,
}




impl<'a> Attribute<'a> {
    /// Get this attribute's name, such as `"id"` in `<div id="my-thing" />`.
    #[inline]
    pub fn name(&self) -> &'a str {
        self.name
    }

    /// The attribute value, such as `"my-thing"` in `<div id="my-thing" />`.
    #[inline]
    pub fn value(&self) -> &'a str {
        self.value
    }

    /// Certain attributes are considered "volatile" and can change via user
    /// input that we can't see when diffing against the old virtual DOM. For
    /// these attributes, we want to always re-set the attribute on the physical
    /// DOM node, even if the old and new virtual DOM nodes have the same value.
    #[inline]
    pub(crate) fn is_volatile(&self) -> bool {
        match self.name {
            "value" | "checked" | "selected" => true,
            _ => false,
        }
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
