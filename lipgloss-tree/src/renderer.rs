//! Tree rendering engine for lipgloss-tree.
//!
//! This module provides the core rendering functionality for tree structures,
//! handling complex styling, alignment, and multiline content rendering.
//! It supports both built-in tree glyphs (├──, └──, etc.) and custom
//! enumerators with proper alignment and styling inheritance.

use crate::children::Children;
use crate::{default_enumerator, default_indenter, Enumerator, Indenter, Node, StyleFunc};
use lipgloss::{height, join_horizontal, join_vertical, Style, LEFT, TOP};
use unicode_width::UnicodeWidthStr;

/// Minimal Children implementation used to synthesize enumerator glyphs with controlled length/index.
///
/// This is used internally by the renderer to generate the correct branch characters
/// (├── vs └──) based on whether a node is the last in its sequence.
struct DummyChildren {
    /// The virtual length of this children collection
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

/// A filtered view of children that exposes only visible, non-empty nodes via an index map.
///
/// This wrapper allows the renderer to work with only the nodes that will actually
/// produce visible output, while maintaining the correct indices for style functions.
struct VisibleChildren<'a> {
    /// Reference to the underlying children collection
    base: &'a dyn Children,
    /// Mapping from visible index to actual index in the base collection
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

/// Styling configuration for tree rendering.
///
/// `TreeStyle` controls how different parts of the tree are styled, including
/// enumerators (branch characters like ├──), items (node content), and the root.
/// It supports both base styles (applied unconditionally) and function styles
/// (applied per-child based on index and context).
///
/// # Examples
///
/// ```rust
/// use lipgloss::Style;
/// use lipgloss_tree::TreeStyle;
///
/// let style = TreeStyle {
///     enumerator_func: |_, _| Style::new().foreground("blue".into()),
///     item_func: |_, _| Style::new().bold(true),
///     enumerator_base: Some(Style::new().padding_right(1)),
///     item_base: None,
///     root: Style::new().bold(true),
/// };
/// ```
#[derive(Debug, Clone)]
pub struct TreeStyle {
    /// Function style applied to enumerators (branch characters) based on child index
    pub enumerator_func: StyleFunc,
    /// Function style applied to item content based on child index  
    pub item_func: StyleFunc,
    /// Base style applied unconditionally to all enumerators before function styles
    pub enumerator_base: Option<Style>,
    /// Base style applied unconditionally to all items before function styles
    pub item_base: Option<Style>,
    /// Style applied to the root node
    pub root: Style,
}

impl Default for TreeStyle {
    /// Creates a default `TreeStyle` with Go lipgloss-compatible behavior.
    ///
    /// The default includes `padding_right(1)` on enumerators to match Go's
    /// behavior of printing a space after tree branch prefixes. Tests that
    /// need different behavior can override with no-op functions.
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

/// The main tree rendering engine.
///
/// `Renderer` is responsible for converting tree node structures into styled
/// text output. It handles complex features like:
/// - Multi-line content with proper indentation
/// - Custom enumerators and indenters  
/// - Style inheritance and override resolution
/// - Alignment of custom enumerators
/// - Branch continuation across container nodes
///
/// # Examples
///
/// ```rust
/// use lipgloss_tree::{Renderer, TreeStyle};
/// use lipgloss::Style;
///
/// let renderer = Renderer::new()
///     .style(TreeStyle::default())
///     .enumerator(|children, i| {
///         if i == children.length() - 1 { "└──".to_string() }
///         else { "├──".to_string() }
///     });
/// ```
#[derive(Clone)]
pub struct Renderer {
    /// Styling configuration for this renderer
    style: TreeStyle,
    /// Function to generate enumerator strings (branch characters)
    enumerator: Enumerator,
    /// Function to generate indentation strings for nested content
    indenter: Indenter,
}

impl Renderer {
    /// Creates a new renderer with default configuration.
    ///
    /// The default renderer uses:
    /// - Standard box-drawing characters for branches (├──, └──)
    /// - Standard indentation (│   for continuing, spaces for last)
    /// - Default styling with padding after enumerators
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lipgloss_tree::Renderer;
    ///
    /// let renderer = Renderer::new();
    /// ```
    pub fn new() -> Self {
        Self {
            style: TreeStyle::default(),
            enumerator: default_enumerator,
            indenter: default_indenter,
        }
    }

    /// Sets the styling configuration for this renderer.
    ///
    /// This replaces the entire `TreeStyle`, affecting how enumerators,
    /// items, and the root are styled.
    ///
    /// # Arguments
    ///
    /// * `style` - The new `TreeStyle` configuration to use
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lipgloss_tree::{Renderer, TreeStyle};
    /// use lipgloss::Style;
    ///
    /// let custom_style = TreeStyle {
    ///     root: Style::new().bold(true),
    ///     ..TreeStyle::default()
    /// };
    ///
    /// let renderer = Renderer::new().style(custom_style);
    /// ```
    pub fn style(mut self, style: TreeStyle) -> Self {
        self.style = style;
        self
    }

    /// Sets the enumerator function for this renderer.
    ///
    /// The enumerator function generates the branch characters (like ├──, └──)
    /// based on the child's position in its parent's children collection.
    ///
    /// # Arguments
    ///
    /// * `enumerator` - Function that takes `(children, index)` and returns a string
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lipgloss_tree::Renderer;
    ///
    /// let renderer = Renderer::new()
    ///     .enumerator(|children, i| {
    ///         if i == children.length() - 1 {
    ///             "╰──".to_string()  // Rounded last branch
    ///         } else {
    ///             "├──".to_string()  // Standard continuing branch
    ///         }
    ///     });
    /// ```
    pub fn enumerator(mut self, enumerator: Enumerator) -> Self {
        self.enumerator = enumerator;
        self
    }

    /// Sets the indenter function for this renderer.
    ///
    /// The indenter function generates indentation strings for child content
    /// that appears below parent nodes, providing the visual connection
    /// between parent and nested children.
    ///
    /// # Arguments
    ///
    /// * `indenter` - Function that takes `(children, index)` and returns indentation string
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lipgloss_tree::Renderer;
    ///
    /// let renderer = Renderer::new()
    ///     .indenter(|children, i| {
    ///         if i == children.length() - 1 {
    ///             "    ".to_string()  // Spaces for last child
    ///         } else {
    ///             "│   ".to_string()  // Vertical line for continuing
    ///         }
    ///     });
    /// ```
    pub fn indenter(mut self, indenter: Indenter) -> Self {
        self.indenter = indenter;
        self
    }

    /// Renders a tree node and its children to a formatted string.
    ///
    /// This is the main rendering method that converts a tree structure into
    /// styled text output. It handles complex scenarios like multi-line content,
    /// style inheritance, custom enumerators, and proper indentation.
    ///
    /// # Arguments
    ///
    /// * `node` - The tree node to render
    /// * `root` - Whether this is the root node (affects root style application)
    /// * `prefix` - String prefix to prepend to each line (for nested rendering)
    ///
    /// # Returns
    ///
    /// A formatted string representation of the tree with proper styling and indentation
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lipgloss_tree::{Renderer, Tree};
    ///
    /// let tree = Tree::new()
    ///     .root("My Tree")
    ///     .child(vec!["Item 1".to_string(), "Item 2".to_string()]);
    ///
    /// let renderer = Renderer::new();
    /// let output = renderer.render(&tree, true, "");
    /// println!("{}", output);
    /// ```
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

        // Calculate alignment padding for custom enumerators (not built-in branch glyphs)
        let mut max_enum_width = 0;
        for i in 0..filtered_children.length() {
            let user_pref = enumerator(&vis_children, i);
            if !is_branch(&user_pref) {
                // Only consider custom enumerators for alignment
                let width = user_pref.width();
                if width > max_enum_width {
                    max_enum_width = width;
                }
            }
        }

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
                // Apply styling to indent based on the type of indenter
                let indent = if raw_indent.trim().is_empty() {
                    // Standard whitespace indenter - apply item_base style if present (for padding)
                    if let Some(base) = &item_base {
                        base.render(&raw_indent)
                    } else {
                        raw_indent.clone()
                    }
                } else {
                    // Custom indenter (like "->") - apply enum_base style if present (for colors/styling)
                    if let Some(base) = &enum_base {
                        base.render(&raw_indent)
                    } else {
                        raw_indent.clone()
                    }
                };

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
                
                // Apply alignment padding for custom enumerators
                if !is_custom_enum {
                    // Built-in branch glyph - no alignment needed
                } else if max_enum_width > 0 {
                    // Custom enumerator - apply alignment padding
                    let current_width = node_prefix.width();
                    let padding_needed = max_enum_width.saturating_sub(current_width);
                    if padding_needed > 0 {
                        node_prefix = format!("{}{}", " ".repeat(padding_needed), node_prefix);
                    }
                }
                
                // CRITICAL: Apply either base style OR function style, NEVER both
                // This prevents double-padding issues where both base and function styles add spacing
                //
                // PADDING_RIGHT BEHAVIOR:
                // - TreeStyle::default() sets enumerator_func to add padding_right(1) 
                // - If enumerator_style (base) is set, it REPLACES the function entirely
                // - The base style can include its own padding_right(1)
                // - Go behavior: EnumeratorStyle() method replaces the default function
                //
                // SPACING CALCULATION:
                // - Tree symbols: "├──", "└──" are 3 chars wide
                // - padding_right(1) adds 1 space → "├── " (4 chars total)
                // - This aligns with default_indenter: "│   " (4 chars) and "    " (4 spaces)
                if let Some(base) = &enum_base {
                    // Base style set via .enumerator_style() - use ONLY this style
                    // Example: Style::new().foreground(color).padding_right(1)
                    node_prefix = base.render(&node_prefix);
                } else {
                    // No base style - use the function style (default or custom)
                    let enum_style_result = enum_style_func(&vis_children, idx);
                    let enum_lead = enum_style_result.render("");
                    
                    // Check if this is a set_string style vs padding-only style
                    if !enum_lead.is_empty() && !enum_lead.trim().is_empty() {
                        // Set_string style with actual content (e.g., "+" prefix)
                        // Apply default padding to tree structure, then prepend the content
                        let default_styled = Style::new().padding_right(1).render(&node_prefix);
                        if !enum_lead.ends_with(' ') {
                            node_prefix = format!("{} {}", enum_lead, default_styled);
                        } else {
                            node_prefix = format!("{}{}", enum_lead, default_styled);
                        }
                    } else {
                        // Padding-only style or no additional content
                        // Apply style function directly to tree structure
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
                    
                    // Check if this is a true set_string style vs padding-only style
                    // Padding-only styles render to whitespace-only strings (spaces, tabs, newlines)
                    // Set_string styles have non-whitespace content
                    let is_padding_only = item_lead.chars().all(|c| c.is_whitespace());
                    
                    if is_padding_only {
                        // Apply the style function to the item directly (padding-only or no style)
                        item = item_style_result.render(&item);
                    } else {
                        // This is a style with a string set via set_string (like Go's SetString)
                        if !item_lead.ends_with(' ') {
                            item = format!("{} {}", item_lead, item);
                        } else {
                            item = format!("{}{}", item_lead, item);
                        }
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
                    // FINAL LINE ASSEMBLY:
                    // multiline_prefix: parent indentation (e.g., 10 spaces from nested list level)
                    // node_prefix: tree symbol with styling (e.g., "[color]├──[reset] " with padding_right)
                    // item: content with any item styling applied
                    //
                    // KNOWN ISSUE: When tree is nested in list with 2-space list_indenter,
                    // an extra space appears after tree symbols. This suggests list_indenter
                    // affects tree content spacing beyond just the multiline_prefix.
                    
                    // DEBUG: Uncomment to debug spacing issues
                    // eprintln!("DEBUG: multiline_prefix='{}', node_prefix='{}', item='{}'", 
                    //           multiline_prefix.replace(' ', "·"), 
                    //           node_prefix.replace(' ', "·"), 
                    //           item.replace(' ', "·"));
                    
                    let line = join_horizontal(TOP, &[&multiline_prefix, &node_prefix, &item]);
                    strs.push(line);
                    // Remember raw indent for subsequent container nodes (before styling)
                    last_display_indent = raw_indent.clone();
                }

                // Recursively render children
                if child.children().length() > 0 {
                    // Even if the child has an empty value (container), we still need to
                    // indent its children so they appear nested under the current item.
                    // Use styled indent with enum styling applied to indenter characters
                    // This ensures custom indenters (like "->") get the same styling as enumerators
                    let styled_indent = indent.clone();
                    
                    // SMART INDENTATION LOGIC:  
                    // Trees nested in lists should not inherit the list's indenter to avoid double indentation.
                    let dummy_children = crate::children::NodeChildren::new();
                    let parent_indent_sample = indenter(&dummy_children, 0);
                    let is_parent_list_indenter = parent_indent_sample.trim() == "" && parent_indent_sample.len() == 2;
                    
                    let child_prefix = if let Some(child_indenter) = child.get_indenter() {
                        let child_indent_sample = child_indenter(&dummy_children, 0);
                        let is_child_tree_indenter = child_indent_sample.contains('│') || child_indent_sample.len() == 4;
                        
                        // DEBUG: uncomment for debugging tree indentation
                        // if child.get_enumerator_style().is_some() {
                        //     eprintln!("DEBUG TREE: parent_indent='{}', child_indent='{}', is_parent_list={}, is_child_tree={}, prefix_len={}", 
                        //         parent_indent_sample.replace(' ', "·"), child_indent_sample.replace(' ', "·"), is_parent_list_indenter, is_child_tree_indenter, prefix.len());
                        // }
                        
                        // Always provide proper nesting prefix - the issue is tree's internal indentation, not prefix
                        format!("{}{}", prefix, styled_indent)
                    } else {
                        // Child has no specific indenter - inherit parent's indentation
                        format!("{}{}", prefix, styled_indent)
                    };
                    
                    // Detect if child has style overrides (indicating it's a styled tree vs plain container)
                    let has_style_overrides = child.get_enumerator_style().is_some() 
                        || child.get_item_style().is_some()
                        || child.get_enumerator_style_func().is_some()
                        || child.get_item_style_func().is_some();
                    
                    // Special case: if this child is a tree with its own indenter, use fresh renderer
                    // to prevent list indenter from affecting tree's internal rendering
                    let child_uses_tree_indenter = if let Some(child_indenter) = child.get_indenter() {
                        let dummy = crate::children::NodeChildren::new();
                        let sample = child_indenter(&dummy, 0);
                        sample.contains('│') || sample.len() == 4
                    } else {
                        false
                    };
                    
                    let mut child_renderer = if child_uses_tree_indenter {
                        // Tree child: use fresh renderer to avoid inheriting list behavior
                        Renderer::new()
                    } else if has_style_overrides {
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

/// Creates a new renderer with default configuration.
///
/// This is a convenience function equivalent to `Renderer::new()`.
/// Provided for API compatibility with other lipgloss libraries.
///
/// # Returns
///
/// A new `Renderer` instance with default settings
///
/// # Examples
///
/// ```rust
/// use lipgloss_tree::new_renderer;
///
/// let renderer = new_renderer();
/// ```
pub fn new_renderer() -> Renderer {
    Renderer::new()
}
