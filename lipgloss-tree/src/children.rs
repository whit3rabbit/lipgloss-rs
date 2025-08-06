use lipgloss::Style;
use std::fmt;

/// Node defines a node in a tree.
// Helper trait to enable cloning boxed trait objects for Node
pub trait CloneNode {
    fn clone_node(&self) -> Box<dyn Node>;
}

impl<T> CloneNode for T
where
    T: 'static + Node + Clone,
{
    fn clone_node(&self) -> Box<dyn Node> {
        Box::new(self.clone())
    }
}

pub trait Node: fmt::Display + CloneNode {
    /// Returns the string value of this node.
    fn value(&self) -> String;

    /// Returns the children of this node.
    fn children(&self) -> Box<dyn Children>;

    /// Returns whether this node is hidden.
    fn hidden(&self) -> bool;

    /// Sets whether this node is hidden.
    fn set_hidden(&mut self, hidden: bool);

    /// Sets the value of this node.
    fn set_value(&mut self, value: String);

    // Optional per-node renderer overrides. Default to None.
    /// Returns the enumerator override for this node, if any.
    fn get_enumerator(&self) -> Option<&crate::Enumerator> {
        None
    }

    /// Returns the indenter override for this node, if any.
    fn get_indenter(&self) -> Option<&crate::Indenter> {
        None
    }

    /// Returns a base item Style (applied before item style func) if any.
    fn get_item_style(&self) -> Option<&Style> {
        None
    }

    /// Returns a base enumerator Style (applied before enum style func) if any.
    fn get_enumerator_style(&self) -> Option<&Style> {
        None
    }

    /// Returns the item style function for this node, if any.
    fn get_item_style_func(&self) -> Option<&crate::StyleFunc> {
        None
    }

    /// Returns the enumerator style function for this node, if any.
    fn get_enumerator_style_func(&self) -> Option<&crate::StyleFunc> {
        None
    }
}

impl Clone for NodeChildren {
    fn clone(&self) -> Self {
        let mut cloned = NodeChildren::new();
        for i in 0..self.length() {
            if let Some(n) = self.at(i) {
                cloned.append(n.clone_node());
            }
        }
        cloned
    }
}

/// Slice is a Children view that applies start/end bounds and optional hidden filtering
#[allow(dead_code)]
struct Slice {
    data: Box<dyn Children>,
    start: usize,
    end: usize,
    skip_hidden: bool,
}

#[allow(dead_code)]
impl Slice {
    fn new(data: Box<dyn Children>, start: usize, end: usize, skip_hidden: bool) -> Self {
        Self {
            data,
            start,
            end,
            skip_hidden,
        }
    }
}

impl Children for Slice {
    fn at(&self, index: usize) -> Option<&dyn Node> {
        let mut j = 0usize;
        let upper = self.end.min(self.data.length());
        for i in self.start..upper {
            if let Some(node) = self.data.at(i) {
                if self.skip_hidden && node.hidden() {
                    continue;
                }
                if j == index {
                    return Some(node);
                }
                j += 1;
            }
        }
        None
    }

    fn length(&self) -> usize {
        let mut count = 0usize;
        let upper = self.end.min(self.data.length());
        for i in self.start..upper {
            if let Some(node) = self.data.at(i) {
                if self.skip_hidden && node.hidden() {
                    continue;
                }
                count += 1;
            }
        }
        count
    }
}

/// Children is the interface that wraps the basic methods of a tree model.
pub trait Children {
    /// Returns the content item of the given index.
    fn at(&self, index: usize) -> Option<&dyn Node>;

    /// Returns the number of children in the tree.
    fn length(&self) -> usize;
}

/// NodeChildren is the implementation of the Children interface with tree Nodes.
pub struct NodeChildren {
    nodes: Vec<Box<dyn Node>>,
}

impl NodeChildren {
    /// Creates a new empty NodeChildren collection.
    pub fn new() -> Self {
        Self { nodes: Vec::new() }
    }

    /// Creates NodeChildren from a vector of boxed nodes.
    pub fn from_nodes(nodes: Vec<Box<dyn Node>>) -> Self {
        Self { nodes }
    }

    /// Appends a child to the list of children.
    pub fn append(&mut self, child: Box<dyn Node>) {
        self.nodes.push(child);
    }

    /// Removes a child from the list at the given index.
    pub fn remove(&mut self, index: usize) -> Option<Box<dyn Node>> {
        if index < self.nodes.len() {
            Some(self.nodes.remove(index))
        } else {
            None
        }
    }

    /// Returns a mutable reference to the node at the given index.
    pub fn at_mut(&mut self, index: usize) -> Option<&mut Box<dyn Node>> {
        self.nodes.get_mut(index)
    }
}

impl Default for NodeChildren {
    fn default() -> Self {
        Self::new()
    }
}

impl Children for NodeChildren {
    fn at(&self, index: usize) -> Option<&dyn Node> {
        self.nodes.get(index).map(|node| node.as_ref())
    }

    fn length(&self) -> usize {
        self.nodes.len()
    }
}

/// Leaf is a node without children.
#[derive(Debug, Clone)]
pub struct Leaf {
    value: String,
    hidden: bool,
}

impl Leaf {
    /// Creates a new Leaf node.
    pub fn new(value: impl Into<String>, hidden: bool) -> Self {
        Self {
            value: value.into(),
            hidden,
        }
    }
}

impl Node for Leaf {
    fn value(&self) -> String {
        self.value.clone()
    }

    fn children(&self) -> Box<dyn Children> {
        Box::new(NodeChildren::new())
    }

    fn hidden(&self) -> bool {
        self.hidden
    }

    fn set_hidden(&mut self, hidden: bool) {
        self.hidden = hidden;
    }

    fn set_value(&mut self, value: String) {
        self.value = value;
    }

    fn get_enumerator(&self) -> Option<&crate::Enumerator> {
        None
    }

    fn get_indenter(&self) -> Option<&crate::Indenter> {
        None
    }
}

impl fmt::Display for Leaf {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

/// Tree implements a Node that can have children.
#[derive(Clone)]
pub struct Tree {
    value: String,
    hidden: bool,
    offset: [usize; 2],
    children: NodeChildren,

    // Style and rendering properties
    enumerator: Option<crate::Enumerator>,
    indenter: Option<crate::Indenter>,
    root_style: Option<Style>,
    item_style: Option<Style>,
    enumerator_style: Option<Style>,
    item_style_func: Option<crate::StyleFunc>,
    enumerator_style_func: Option<crate::StyleFunc>,
}

impl Tree {
    /// Creates a new Tree.
    pub fn new() -> Self {
        Self {
            value: String::new(),
            hidden: false,
            offset: [0, 0],
            children: NodeChildren::new(),
            enumerator: None,
            indenter: None,
            root_style: None,
            item_style: None,
            enumerator_style: None,
            item_style_func: None,
            enumerator_style_func: None,
        }
    }

    /// Sets the root value of this tree.
    pub fn root(mut self, root: impl Into<String>) -> Self {
        self.value = root.into();
        self
    }

    /// Hides or shows this tree.
    pub fn hide(mut self, hide: bool) -> Self {
        self.hidden = hide;
        self
    }

    /// Sets the offset for children (start, end).
    pub fn offset(mut self, start: usize, end: usize) -> Self {
        let (start, end) = if start > end {
            (end, start)
        } else {
            (start, end)
        };

        let end = if end > self.children.length() {
            self.children.length()
        } else {
            end
        };

        self.offset = [start, end];
        self
    }

    /// Adds children to this tree. Matches Go's Child(...any) method.
    pub fn child(mut self, children: Vec<Box<dyn Node>>) -> Self {
        for child in children {
            self.children.append(child);
        }
        self
    }

    /// Adds a single child to this tree.
    pub fn add_child(mut self, child: impl Into<Box<dyn Node>>) -> Self {
        self.children.append(child.into());
        self
    }

    /// Sets the enumerator for this tree.
    pub fn enumerator(mut self, enumerator: crate::Enumerator) -> Self {
        self.enumerator = Some(enumerator);
        self
    }

    /// Sets the indenter for this tree.
    pub fn indenter(mut self, indenter: crate::Indenter) -> Self {
        self.indenter = Some(indenter);
        self
    }

    /// Sets the root style.
    pub fn root_style(mut self, style: Style) -> Self {
        self.root_style = Some(style);
        self
    }

    /// Sets the item style for all items.
    pub fn item_style(mut self, style: Style) -> Self {
        self.item_style = Some(style);
        self
    }

    /// Sets the enumerator style for all enumerators.
    pub fn enumerator_style(mut self, style: Style) -> Self {
        self.enumerator_style = Some(style);
        self
    }

    /// Sets the item style function for conditional styling.
    pub fn item_style_func(mut self, func: crate::StyleFunc) -> Self {
        self.item_style_func = Some(func);
        self
    }

    /// Sets the enumerator style function for conditional styling.
    pub fn enumerator_style_func(mut self, func: crate::StyleFunc) -> Self {
        self.enumerator_style_func = Some(func);
        self
    }

    /// Gets the enumerator for this tree.
    pub fn get_enumerator(&self) -> Option<&crate::Enumerator> {
        self.enumerator.as_ref()
    }

    /// Gets the indenter for this tree.
    pub fn get_indenter(&self) -> Option<&crate::Indenter> {
        self.indenter.as_ref()
    }

    /// Gets the root style.
    pub fn get_root_style(&self) -> Option<&Style> {
        self.root_style.as_ref()
    }

    /// Gets the item style function.
    pub fn get_item_style_func(&self) -> Option<&crate::StyleFunc> {
        self.item_style_func.as_ref()
    }

    /// Gets the enumerator style function.
    pub fn get_enumerator_style_func(&self) -> Option<&crate::StyleFunc> {
        self.enumerator_style_func.as_ref()
    }

    /// Gets the item style.
    pub fn get_item_style(&self) -> Option<&Style> {
        self.item_style.as_ref()
    }

    /// Gets the enumerator style.
    pub fn get_enumerator_style(&self) -> Option<&Style> {
        self.enumerator_style.as_ref()
    }

}

impl Default for Tree {
    fn default() -> Self {
        Self::new()
    }
}

impl Node for Tree {
    fn value(&self) -> String {
        self.value.clone()
    }

    fn children(&self) -> Box<dyn Children> {
        // Return a subset of children based on offset, cloning nodes to preserve structure
        let start = self.offset[0];
        let end = if self.offset[1] == 0 {
            self.children.length()
        } else {
            self.children.length().saturating_sub(self.offset[1])
        };

        let mut filtered_children = NodeChildren::new();
        for i in start..end.min(self.children.length()) {
            if let Some(node) = self.children.at(i) {
                if node.hidden() {
                    continue;
                }
                filtered_children.append(node.clone_node());
            }
        }
        Box::new(filtered_children)
    }

    fn hidden(&self) -> bool {
        self.hidden
    }

    fn set_hidden(&mut self, hidden: bool) {
        self.hidden = hidden;
    }

    fn set_value(&mut self, value: String) {
        self.value = value;
    }

    fn get_enumerator(&self) -> Option<&crate::Enumerator> {
        self.enumerator.as_ref()
    }

    fn get_indenter(&self) -> Option<&crate::Indenter> {
        self.indenter.as_ref()
    }

    fn get_item_style(&self) -> Option<&Style> {
        self.item_style.as_ref()
    }

    fn get_enumerator_style(&self) -> Option<&Style> {
        self.enumerator_style.as_ref()
    }

    fn get_item_style_func(&self) -> Option<&crate::StyleFunc> {
        self.item_style_func.as_ref()
    }

    fn get_enumerator_style_func(&self) -> Option<&crate::StyleFunc> {
        self.enumerator_style_func.as_ref()
    }
}

impl fmt::Display for Tree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut renderer = crate::renderer::Renderer::new()
            .enumerator(self.enumerator.unwrap_or(crate::default_enumerator))
            .indenter(self.indenter.unwrap_or(crate::default_indenter));

        // Build a TreeStyle snapshot from this Tree and apply to renderer
        let style = crate::renderer::TreeStyle {
            enumerator_func: self
                .enumerator_style_func
                .unwrap_or(|_, _| Style::new().padding_right(1)),
            item_func: self.item_style_func.unwrap_or(|_, _| Style::new()),
            // Root style if any
            root: self.root_style.clone().unwrap_or_default(),
            // Base styles
            enumerator_base: self.enumerator_style.clone(),
            item_base: self.item_style.clone(),
        };
        renderer = renderer.style(style);

        let output = renderer.render(self, true, "");
        write!(f, "{}", output)
    }
}

// Direct conversions to Box<dyn Node> for the simplified API
impl From<&str> for Box<dyn Node> {
    fn from(s: &str) -> Self {
        Box::new(Leaf::new(s.to_string(), false))
    }
}

impl From<String> for Box<dyn Node> {
    fn from(s: String) -> Self {
        Box::new(Leaf::new(s, false))
    }
}

impl From<Tree> for Box<dyn Node> {
    fn from(tree: Tree) -> Self {
        Box::new(tree)
    }
}

impl From<Leaf> for Box<dyn Node> {
    fn from(leaf: Leaf) -> Self {
        Box::new(leaf)
    }
}

/// Creates a new Tree with a root value.
pub fn root(root: impl Into<String>) -> Tree {
    Tree::new().root(root)
}

/// Creates NodeChildren from string data.
pub fn new_string_data(data: &[&str]) -> NodeChildren {
    let mut result = NodeChildren::new();
    for &item in data {
        result.append(Box::new(Leaf::new(item, false)));
    }
    result
}

/// Filter applies a filter on some data.
pub struct Filter {
    data: Box<dyn Children>,
    filter: Option<Box<dyn Fn(usize) -> bool>>,
}

impl Filter {
    /// Creates a new Filter.
    pub fn new(data: Box<dyn Children>) -> Self {
        Self { data, filter: None }
    }

    /// Sets the filter function.
    pub fn filter<F>(mut self, f: F) -> Self
    where
        F: Fn(usize) -> bool + 'static,
    {
        self.filter = Some(Box::new(f));
        self
    }
}

impl Children for Filter {
    fn at(&self, index: usize) -> Option<&dyn Node> {
        if let Some(ref filter_func) = self.filter {
            let mut j = 0;
            for i in 0..self.data.length() {
                if filter_func(i) {
                    if j == index {
                        return self.data.at(i);
                    }
                    j += 1;
                }
            }
            None
        } else {
            self.data.at(index)
        }
    }

    fn length(&self) -> usize {
        if let Some(ref filter_func) = self.filter {
            let mut count = 0;
            for i in 0..self.data.length() {
                if filter_func(i) {
                    count += 1;
                }
            }
            count
        } else {
            self.data.length()
        }
    }
}
