use crate::children::Children;
use crate::{default_enumerator, default_indenter, Enumerator, Indenter, Node, StyleFunc};
use lipgloss::{height, join_horizontal, join_vertical, Style, LEFT, TOP};

// Minimal Children impl to synthesize enumerator glyphs with controlled length/index
struct DummyChildren {
    len: usize,
}
impl Children for DummyChildren {
    fn at(&self, _index: usize) -> Option<&dyn Node> {
        None
    }
    fn length(&self) -> usize {
        self.len
    }
}

// Children view exposing only visible, non-empty nodes via an index map
struct VisibleChildren<'a> {
    base: &'a dyn Children,
    map: Vec<usize>,
}
impl<'a> Children for VisibleChildren<'a> {
    fn at(&self, index: usize) -> Option<&dyn Node> {
        self.map.get(index).and_then(|&i| self.base.at(i))
    }
    fn length(&self) -> usize {
        self.map.len()
    }
}

/// Style is the styling applied to the tree.
#[derive(Debug, Clone)]
pub struct TreeStyle {
    // Function styles applied per-child based on visible index
    pub enumerator_func: StyleFunc,
    pub item_func: StyleFunc,
    // Base styles applied unconditionally before the function styles
    pub enumerator_base: Option<Style>,
    pub item_base: Option<Style>,
    // Root style
    pub root: Style,
}

impl Default for TreeStyle {
    fn default() -> Self {
        Self {
            // By default, Go prints a space after the prefix. Model this
            // with padding_right(1). Tests that set nil funcs expect no space
            // and will override with a no-op function.
            enumerator_func: |_children, _i| Style::new().padding_right(1),
            item_func: |_children, _i| Style::new(),
            enumerator_base: None,
            item_base: None,
            root: Style::new(),
        }
    }
}

/// Renderer is responsible for rendering trees.
#[derive(Clone)]
pub struct Renderer {
    style: TreeStyle,
    enumerator: Enumerator,
    indenter: Indenter,
}

impl Renderer {
    /// Creates a new renderer.
    pub fn new() -> Self {
        Self {
            style: TreeStyle::default(),
            enumerator: default_enumerator,
            indenter: default_indenter,
        }
    }

    /// Sets the style for this renderer.
    pub fn style(mut self, style: TreeStyle) -> Self {
        self.style = style;
        self
    }

    /// Sets the enumerator for this renderer.
    pub fn enumerator(mut self, enumerator: Enumerator) -> Self {
        self.enumerator = enumerator;
        self
    }

    /// Sets the indenter for this renderer.
    pub fn indenter(mut self, indenter: Indenter) -> Self {
        self.indenter = indenter;
        self
    }

    /// Renders a tree node to a string.
    pub fn render(&self, node: &dyn Node, root: bool, prefix: &str) -> String {
        if node.hidden() {
            return String::new();
        }

        // Debug: uncomment for debugging
        // eprintln!("RENDER_START: root={}, prefix='{}', prefix_len={}", root, prefix.replace('\n', "\\n"), prefix.len());

        let mut strs = Vec::new();
        let children = node.children();
        // Prefer per-node overrides for enumerator/indenter when present, otherwise use renderer config
        let enumerator = node.get_enumerator().copied().unwrap_or(self.enumerator);
        let indenter = node.get_indenter().copied().unwrap_or(self.indenter);

        // Print the root node name if it's not empty
        if !node.value().is_empty() && root {
            strs.push(self.style.root.render(&node.value()));
        }

        // Build a filtered view of direct children that will render a line (non-hidden, non-empty)
        let mut visible_nodes: Vec<Box<dyn Node>> = Vec::new();
        for i in 0..children.length() {
            if let Some(child) = children.at(i) {
                if child.hidden() || child.value().is_empty() {
                    continue;
                }
                visible_nodes.push(child.clone_node());
            }
        }
        let filtered_children = crate::children::NodeChildren::from_nodes(visible_nodes);

        // Helper: does this node or any of its descendants render a visible line?
        fn has_visible_line(node: &dyn Node) -> bool {
            if node.hidden() {
                return false;
            }
            if !node.value().is_empty() {
                return true;
            }
            let ch = node.children();
            for i in 0..ch.length() {
                if let Some(n) = ch.at(i) {
                    if has_visible_line(n) {
                        return true;
                    }
                }
            }
            false
        }

        // Precompute which visible direct child is last in the overall visual sequence.
        // A visible child is last if there is no later sibling (direct) whose subtree would
        // render any visible line.
        let mut is_last_vec: Vec<bool> = vec![true; filtered_children.length()];
        #[allow(clippy::needless_range_loop)]
        for vi in 0..filtered_children.length() {
            let mut last = true;
            // locate this visible child among all direct children
            let mut seen = 0usize;
            for i in 0..children.length() {
                if let Some(ch) = children.at(i) {
                    if ch.hidden() {
                        continue;
                    }
                    if !ch.value().is_empty() {
                        if seen == vi {
                            // check later siblings
                            for j in (i + 1)..children.length() {
                                if let Some(next) = children.at(j) {
                                    if has_visible_line(next) {
                                        last = false;
                                        break;
                                    }
                                }
                            }
                            break;
                        }
                        seen += 1;
                    }
                }
            }
            is_last_vec[vi] = last;
        }

        // Prepare a visible-children view for style functions
        let mut vis_map: Vec<usize> = Vec::new();
        for i in 0..children.length() {
            if let Some(ch) = children.at(i) {
                if !ch.hidden() && !ch.value().is_empty() {
                    vis_map.push(i);
                }
            }
        }
        let vis_children = VisibleChildren {
            base: &*children,
            map: vis_map,
        };

        // Helper to detect built-in branch glyphs
        let is_branch = |s: &str| s == "├──" || s == "└──" || s == "╰──";

        // Skip width calculation entirely - Go doesn't seem to do alignment padding

        // Render children
        let mut last_display_indent = String::new();
        for i in 0..children.length() {
            if let Some(child) = children.at(i) {
                if child.hidden() {
                    continue;
                }

                // Determine display index for visible children by counting prior visible siblings
                let mut display_idx_opt: Option<usize> = None;
                if !child.value().is_empty() {
                    let mut count = 0usize;
                    for j in 0..i {
                        if let Some(prev) = children.at(j) {
                            if !prev.hidden() && !prev.value().is_empty() {
                                count += 1;
                            }
                        }
                    }
                    display_idx_opt = Some(count);
                }
                let idx = display_idx_opt.unwrap_or(0);
                // Build effective styles for this node, respecting node overrides first
                let enum_style_func = node
                    .get_enumerator_style_func()
                    .copied()
                    .unwrap_or(self.style.enumerator_func);
                let item_style_func = node
                    .get_item_style_func()
                    .copied()
                    .unwrap_or(self.style.item_func);

                let enum_base = node
                    .get_enumerator_style()
                    .cloned()
                    .or_else(|| self.style.enumerator_base.clone());
                let item_base = node
                    .get_item_style()
                    .cloned()
                    .or_else(|| self.style.item_base.clone());


                // Compute indent: for visible children use indenter(filtered, display_idx);
                // for container (empty value) nodes, reuse the last visible indent so nested
                // content attaches under the previous item.
                let raw_indent = if let Some(di) = display_idx_opt {
                    indenter(&filtered_children, di)
                } else {
                    last_display_indent.clone()
                };
                let indent = raw_indent.clone();
                // Don't apply base style to indent here - we'll apply it to the final prefix instead
                // This prevents double styling when we use the indent in extension logic

                // Compute enumerator only for visible children
                // Base branch according to position in overall visual sequence
                let user_pref = enumerator(&vis_children, idx);
                let is_custom_enum = !is_branch(&user_pref);
                let mut node_prefix = if !is_custom_enum {
                    let dc = DummyChildren { len: 2 };
                    if is_last_vec[idx] {
                        enumerator(&dc, 1)
                    } else {
                        enumerator(&dc, 0)
                    }
                } else {
                    user_pref.clone()
                };
                // FIXED: Apply either base style OR function style, not both
                // This is the key fix for the double-spacing issue
                if let Some(base) = &enum_base {
                    // If we have a base style, use it instead of the function
                    // This matches Go's behavior where EnumeratorStyle() replaces the function
                    node_prefix = base.render(&node_prefix);
                } else {
                    // Only apply the function style if no base style is set
                    let enum_style_result = enum_style_func(&vis_children, idx);
                    let enum_lead = enum_style_result.render("");
                    // Check if enum_lead is just padding (whitespace only) vs actual set_string content
                    if !enum_lead.is_empty() && !enum_lead.trim().is_empty() {
                        // This is a true set_string style with actual content (like "+")
                        // Apply default padding to the tree structure first, then combine
                        let default_styled = Style::new().padding_right(1).render(&node_prefix);
                        if !enum_lead.ends_with(' ') {
                            node_prefix = format!("{} {}", enum_lead, default_styled);
                        } else {
                            node_prefix = format!("{}{}", enum_lead, default_styled);
                        }
                    } else {
                        // Apply the style function to the tree structure directly (padding-only or no style)
                        node_prefix = enum_style_result.render(&node_prefix);
                    }
                }
                // Note: Alignment padding disabled - Go doesn't align prefixes to same width
                // Debug: uncomment for debugging
                // eprintln!("RENDER: idx={}, prefix='{}', width={}, max_len={}, final='{}'", idx, node_prefix.replace('\n', "\\n"), prefix_width, max_len, node_prefix.replace('\n', "\\n"));
                // Apply item styling: base style OR function style, not both
                let mut item = child.value();
                if let Some(base) = &item_base {
                    item = base.render(&item);
                } else {
                    // Only apply function style if no base style is set
                    let item_style_result = item_style_func(&vis_children, idx);
                    let item_lead = item_style_result.render("");
                    if !item_lead.is_empty() {
                        // This is a style with a string set via set_string (like Go's SetString)
                        if !item_lead.ends_with(' ') {
                            item = format!("{} {}", item_lead, item);
                        } else {
                            item = format!("{}{}", item_lead, item);
                        }
                    } else {
                        // Apply the style function to the item directly
                        item = item_style_result.render(&item);
                    }
                }
                let mut multiline_prefix = prefix.to_string();

                // Handle multiline prefixes and items
                let item_height = height(&item);
                let mut node_prefix_height = height(&node_prefix);

                // Extend node prefix if item is taller
                while item_height > node_prefix_height {
                    // Use raw indent for multiline extension - no styling needed
                    let extension_indent = indent.clone();
                    node_prefix = join_vertical(LEFT, &[&node_prefix, &extension_indent]);
                    node_prefix_height = height(&node_prefix);
                }

                // Extend multiline prefix if node prefix is taller
                let mut multiline_prefix_height = height(&multiline_prefix);
                while node_prefix_height > multiline_prefix_height {
                    multiline_prefix = join_vertical(LEFT, &[&multiline_prefix, prefix]);
                    multiline_prefix_height = height(&multiline_prefix);
                }

                // Only emit a line if the child has a non-empty value; empty-value nodes are containers for sublists
                if !child.value().is_empty() {
                    let line = join_horizontal(TOP, &[&multiline_prefix, &node_prefix, &item]);
                    strs.push(line);
                    // Remember raw indent for subsequent container nodes (before styling)
                    last_display_indent = raw_indent.clone();
                }

                // Recursively render children
                if child.children().length() > 0 {
                    // Even if the child has an empty value (container), we still need to
                    // indent its children so they appear nested under the current item.
                    // Use raw indent without enum styling - indenter provides the spacing structure
                    // The enum styling should only apply to tree branch glyphs, not indenter spacing
                    let styled_indent = indent.clone();
                    let child_prefix = format!("{}{}", prefix, styled_indent);
                    
                    // Detect if child has style overrides (indicating it's a styled tree vs plain container)
                    let has_style_overrides = child.get_enumerator_style().is_some() 
                        || child.get_item_style().is_some()
                        || child.get_enumerator_style_func().is_some()
                        || child.get_item_style_func().is_some();
                    
                    let mut child_renderer = if has_style_overrides {
                        // Child has style overrides: use tree defaults for behavior but child styles
                        Renderer::new()
                    } else {
                        // Child has no overrides: inherit parent's behavior
                        Renderer::new()
                            .enumerator(self.enumerator)  
                            .indenter(self.indenter)
                    };
                    
                    // Apply any explicit functional overrides from the child node itself
                    if let Some(e) = child.get_enumerator() {
                        child_renderer = child_renderer.enumerator(*e);
                    }
                    if let Some(i) = child.get_indenter() {
                        child_renderer = child_renderer.indenter(*i);
                    }
                    
                    // For style functions, use tree defaults unless child has specific overrides
                    // Children should inherit parent styles when they don't have their own
                    let style = TreeStyle {
                        enumerator_func: child.get_enumerator_style_func().copied().unwrap_or(|_, _| Style::new().padding_right(1)),
                        item_func: child.get_item_style_func().copied().unwrap_or(|_, _| Style::new()),
                        root: Style::default(),
                        // Inherit parent's base styles if child doesn't have overrides
                        enumerator_base: child.get_enumerator_style().cloned()
                            .or_else(|| self.style.enumerator_base.clone()),
                        item_base: child.get_item_style().cloned()
                            .or_else(|| self.style.item_base.clone()),
                    };
                    child_renderer = child_renderer.style(style);
                    
                    let mut child_output = child_renderer.render(child, false, &child_prefix);
                    // If this child is an unnamed container and there are later siblings that
                    // will render visible lines, ensure the container's last visible branch uses
                    // the mid-branch glyph to visually continue the vertical line across
                    // containers (e.g., the 5th Qux before subsequent Quux entries).
                    if child.value().is_empty() {
                        let mut future_exists = false;
                        for j in (i + 1)..children.length() {
                            if let Some(next) = children.at(j) {
                                // Only consider later unnamed containers, which will visually
                                // continue under the same prior item indent.
                                if next.value().is_empty() && has_visible_line(next) {
                                    future_exists = true;
                                    break;
                                }
                            }
                        }
                        if future_exists {
                            // Replace the deepest last-branch at this level with a mid-branch.
                            let dc = DummyChildren { len: 2 };
                            let last_branch = enumerator(&dc, 1);
                            let mid_branch = enumerator(&dc, 0);
                            let look_for = format!("{}{}", child_prefix, last_branch);
                            if let Some(pos) = child_output.rfind(&look_for) {
                                // Ensure we are at line start; find preceding newline or start
                                let line_start =
                                    child_output[..pos].rfind('\n').map(|p| p + 1).unwrap_or(0);
                                if line_start == pos {
                                    child_output.replace_range(
                                        pos..pos + look_for.len(),
                                        &format!("{}{}", child_prefix, mid_branch),
                                    );
                                }
                            }
                        }
                    }
                    if !child_output.is_empty() {
                        strs.push(child_output);
                    }
                }
            }
        }

        strs.join("\n")
    }
}

impl Default for Renderer {
    fn default() -> Self {
        Self::new()
    }
}

/// Creates a new renderer.
pub fn new_renderer() -> Renderer {
    Renderer::new()
}
