use super::node::VirtualDomNode;
use web_sys::Node;


/// VirtualDom represents a virtual dom tree
pub struct VirtualDom {
    node: VirtualDomNode,
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
    pub fn render(&mut self, _el: Node, new_node: VirtualDomNode) {
        // TODO: some magical comparisons that updates the contents of el
        self.node = new_node;
    }
}