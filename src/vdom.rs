use super::{
    node::{Element, VirtualDomNode},
    render::update_element,
};

/// VirtualDom represents a virtual dom tree
pub struct VirtualDom {
    pub node: VirtualDomNode,
}

impl VirtualDom {
    /// new creates an empty VirtualDom
    pub fn new() -> VirtualDom {
        VirtualDom {
            node: VirtualDomNode::Empty,
        }
    }

    /// Compares two virtual dom tree structures and updates the real DOM
    /// then stores the new dom tree for future comparisons
    pub fn render(&mut self, root: &mut Element, new_node: VirtualDomNode) {
        update_element(root, 0, &new_node, &self.node);
        self.node = new_node;
    }
}