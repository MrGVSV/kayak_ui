/// Controls how the cursor interacts on a given node
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum PointerEvents {
    /// Allow all pointer events on this node and its children
    All,
    /// Allow pointer events on this node but not on its children
    SelfOnly,
    /// Allow pointer events on this node's children but not on itself
    ChildrenOnly,
    /// Disallow all pointer events on this node and its children
    None,
}

impl Default for PointerEvents {
    fn default() -> Self {
        Self::All
    }
}
