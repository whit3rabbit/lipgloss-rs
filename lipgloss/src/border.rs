/// Defines all the glyphs that make up a box border for terminal user interfaces.
///
/// A `Border` contains all the Unicode characters needed to draw complete box borders,
/// including corners, edges, and junction characters for complex layouts like tables.
/// This struct mirrors the Go Lip Gloss Border type, providing the same visual styling
/// capabilities in Rust.
///
/// # Examples
///
/// ```
/// use lipgloss::Border;
///
/// // Create a custom border using ASCII characters
/// let custom_border = Border::new(
///     "-", "-", "|", "|", "+", "+", "+", "+",
///     "+", "+", "+", "+", "+",
/// );
///
/// // Or use one of the predefined borders
/// let normal = lipgloss::normal_border();
/// let rounded = lipgloss::rounded_border();
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Border {
    /// Character used for the top edge of the border
    pub top: &'static str,
    /// Character used for the bottom edge of the border
    pub bottom: &'static str,
    /// Character used for the left edge of the border
    pub left: &'static str,
    /// Character used for the right edge of the border
    pub right: &'static str,
    /// Character used for the top-left corner of the border
    pub top_left: &'static str,
    /// Character used for the top-right corner of the border
    pub top_right: &'static str,
    /// Character used for the bottom-left corner of the border
    pub bottom_left: &'static str,
    /// Character used for the bottom-right corner of the border
    pub bottom_right: &'static str,
    /// Character used for left-side junction points (for table borders)
    pub middle_left: &'static str,
    /// Character used for right-side junction points (for table borders)
    pub middle_right: &'static str,
    /// Character used for cross junction points (for table borders)
    pub middle: &'static str,
    /// Character used for top junction points (for table borders)
    pub middle_top: &'static str,
    /// Character used for bottom junction points (for table borders)
    pub middle_bottom: &'static str,
}

impl Border {
    /// Creates a new `Border` with custom characters for all border elements.
    ///
    /// This constructor allows you to define a completely custom border style by
    /// specifying characters for each part of the border: edges, corners, and
    /// junction points for table rendering.
    ///
    /// # Arguments
    ///
    /// * `top` - Character for the top edge
    /// * `bottom` - Character for the bottom edge  
    /// * `left` - Character for the left edge
    /// * `right` - Character for the right edge
    /// * `top_left` - Character for the top-left corner
    /// * `top_right` - Character for the top-right corner
    /// * `bottom_left` - Character for the bottom-left corner
    /// * `bottom_right` - Character for the bottom-right corner
    /// * `middle_left` - Character for left junction points (tables)
    /// * `middle_right` - Character for right junction points (tables)
    /// * `middle` - Character for cross junction points (tables)
    /// * `middle_top` - Character for top junction points (tables)
    /// * `middle_bottom` - Character for bottom junction points (tables)
    ///
    /// # Examples
    ///
    /// ```
    /// use lipgloss::Border;
    ///
    /// // Create a simple ASCII border
    /// let ascii_border = Border::new(
    ///     "-", "-", "|", "|", "+", "+", "+", "+",
    ///     "+", "+", "+", "+", "+",
    /// );
    ///
    /// // Create a Unicode box-drawing border
    /// let unicode_border = Border::new(
    ///     "─", "─", "│", "│", "┌", "┐", "└", "┘",
    ///     "├", "┤", "┼", "┬", "┴",
    /// );
    /// ```
    #[allow(clippy::too_many_arguments)]
    pub const fn new(
        top: &'static str,
        bottom: &'static str,
        left: &'static str,
        right: &'static str,
        top_left: &'static str,
        top_right: &'static str,
        bottom_left: &'static str,
        bottom_right: &'static str,
        middle_left: &'static str,
        middle_right: &'static str,
        middle: &'static str,
        middle_top: &'static str,
        middle_bottom: &'static str,
    ) -> Self {
        Self {
            top,
            bottom,
            left,
            right,
            top_left,
            top_right,
            bottom_left,
            bottom_right,
            middle_left,
            middle_right,
            middle,
            middle_top,
            middle_bottom,
        }
    }

    /// Returns the maximum character width of the top edge components.
    ///
    /// This method calculates the visual width needed for the top edge of the border
    /// by finding the maximum width among the top-left corner, top edge, and
    /// top-right corner characters. This is important for proper alignment when
    /// working with Unicode characters that may have different display widths.
    ///
    /// # Returns
    ///
    /// The maximum character width in terminal columns needed for the top edge.
    ///
    /// # Examples
    ///
    /// ```
    /// use lipgloss::{normal_border, Border};
    ///
    /// let border = normal_border();
    /// assert_eq!(border.get_top_size(), 1);
    ///
    /// // Wide Unicode character example
    /// let wide_border = Border::new(
    ///     "太", "-", "|", "|", "+", "+", "+", "+",
    ///     "+", "+", "+", "+", "+",
    /// );
    /// assert!(wide_border.get_top_size() >= 2);
    /// ```
    pub fn get_top_size(&self) -> usize {
        get_border_edge_width(&[self.top_left, self.top, self.top_right])
    }

    /// Returns the maximum character width of the right edge components.
    ///
    /// This method calculates the visual width needed for the right edge of the border
    /// by finding the maximum width among the top-right corner, right edge, and
    /// bottom-right corner characters.
    ///
    /// # Returns
    ///
    /// The maximum character width in terminal columns needed for the right edge.
    ///
    /// # Examples
    ///
    /// ```
    /// use lipgloss::normal_border;
    ///
    /// let border = normal_border();
    /// assert_eq!(border.get_right_size(), 1);
    /// ```
    pub fn get_right_size(&self) -> usize {
        get_border_edge_width(&[self.top_right, self.right, self.bottom_right])
    }

    /// Returns the maximum character width of the bottom edge components.
    ///
    /// This method calculates the visual width needed for the bottom edge of the border
    /// by finding the maximum width among the bottom-left corner, bottom edge, and
    /// bottom-right corner characters.
    ///
    /// # Returns
    ///
    /// The maximum character width in terminal columns needed for the bottom edge.
    ///
    /// # Examples
    ///
    /// ```
    /// use lipgloss::normal_border;
    ///
    /// let border = normal_border();
    /// assert_eq!(border.get_bottom_size(), 1);
    /// ```
    pub fn get_bottom_size(&self) -> usize {
        get_border_edge_width(&[self.bottom_left, self.bottom, self.bottom_right])
    }

    /// Returns the maximum character width of the left edge components.
    ///
    /// This method calculates the visual width needed for the left edge of the border
    /// by finding the maximum width among the top-left corner, left edge, and
    /// bottom-left corner characters.
    ///
    /// # Returns
    ///
    /// The maximum character width in terminal columns needed for the left edge.
    ///
    /// # Examples
    ///
    /// ```
    /// use lipgloss::normal_border;
    ///
    /// let border = normal_border();
    /// assert_eq!(border.get_left_size(), 1);
    /// ```
    pub fn get_left_size(&self) -> usize {
        get_border_edge_width(&[self.top_left, self.left, self.bottom_left])
    }
}

/// Returns a border using standard single-line box-drawing characters.
///
/// This creates a clean, professional-looking border using Unicode box-drawing
/// characters. It's the most commonly used border style and works well in most
/// terminal environments that support Unicode.
///
/// The border uses these characters:
/// - Horizontal lines: `─`
/// - Vertical lines: `│`
/// - Corners: `┌┐└┘`
/// - Junction points: `├┤┼┬┴`
///
/// # Examples
///
/// ```
/// use lipgloss::normal_border;
///
/// let border = normal_border();
/// println!("Corner: {}", border.top_left); // prints: ┌
/// ```
pub const fn normal_border() -> Border {
    Border::new(
        "─", "─", "│", "│", "┌", "┐", "└", "┘", "├", "┤", "┼", "┬", "┴",
    )
}

/// Returns a border with rounded corners using box-drawing characters.
///
/// This creates a softer, more modern-looking border by using rounded corner
/// characters while keeping the same horizontal and vertical lines as the normal border.
/// The rounded corners give a friendlier appearance compared to sharp corners.
///
/// The border uses these characters:
/// - Horizontal lines: `─`
/// - Vertical lines: `│`
/// - Rounded corners: `╭╮╰╯`
/// - Junction points: `├┤┼┬┴`
///
/// # Examples
///
/// ```
/// use lipgloss::rounded_border;
///
/// let border = rounded_border();
/// println!("Rounded corner: {}", border.top_left); // prints: ╭
/// ```
pub const fn rounded_border() -> Border {
    Border::new(
        "─", "─", "│", "│", "╭", "╮", "╰", "╯", "├", "┤", "┼", "┬", "┴",
    )
}

/// Returns a solid block border using full block characters.
///
/// This creates a very bold, solid border using block characters (`█`) for all
/// border elements. This style is useful for creating highly visible frames or
/// when you want maximum visual impact. The border appears as a solid rectangle.
///
/// All border components use the same character: `█`
///
/// # Examples
///
/// ```
/// use lipgloss::block_border;
///
/// let border = block_border();
/// println!("Block: {}", border.top); // prints: █
/// ```
pub const fn block_border() -> Border {
    Border::new(
        "█", "█", "█", "█", "█", "█", "█", "█", "█", "█", "█", "█", "█",
    )
}

/// Returns a border using thick box-drawing characters.
///
/// This creates a bold, heavy border using thick Unicode box-drawing characters.
/// It's more visually prominent than the normal border while maintaining the
/// same structural layout. Useful when you need borders that stand out more.
///
/// The border uses these characters:
/// - Horizontal lines: `━`
/// - Vertical lines: `┃`
/// - Corners: `┏┓┗┛`
/// - Junction points: `┣┫╋┳┻`
///
/// # Examples
///
/// ```
/// use lipgloss::thick_border;
///
/// let border = thick_border();
/// println!("Thick line: {}", border.top); // prints: ━
/// ```
pub const fn thick_border() -> Border {
    Border::new(
        "━", "━", "┃", "┃", "┏", "┓", "┗", "┛", "┣", "┫", "╋", "┳", "┻",
    )
}

/// Returns a border using double-line box-drawing characters.
///
/// This creates an elegant border using double-line Unicode box-drawing characters.
/// The double lines provide a distinctive, formal appearance that's less heavy
/// than thick borders but more prominent than normal borders.
///
/// The border uses these characters:
/// - Horizontal lines: `═`
/// - Vertical lines: `║`
/// - Corners: `╔╗╚╝`
/// - Junction points: `╠╣╬╦╩`
///
/// # Examples
///
/// ```
/// use lipgloss::double_border;
///
/// let border = double_border();
/// println!("Double line: {}", border.top); // prints: ═
/// ```
pub const fn double_border() -> Border {
    Border::new(
        "═", "═", "║", "║", "╔", "╗", "╚", "╝", "╠", "╣", "╬", "╦", "╩",
    )
}

/// Returns an invisible border made entirely of space characters.
///
/// This creates a "border" that reserves space for border positioning and sizing
/// calculations but doesn't draw any visible lines. Useful for layout purposes
/// when you want consistent spacing and positioning without visible borders.
///
/// All border components use space characters: ` `
///
/// # Examples
///
/// ```
/// use lipgloss::hidden_border;
///
/// let border = hidden_border();
/// println!("Hidden: '{}'!", border.top); // prints: ' '!
///
/// // Useful for maintaining layout consistency
/// let visible_style = lipgloss::Style::new().border(lipgloss::normal_border());
/// let hidden_style = lipgloss::Style::new().border(hidden_border());
/// // Both styles will have the same dimensions, but only one draws borders
/// ```
pub const fn hidden_border() -> Border {
    Border::new(
        " ", " ", " ", " ", " ", " ", " ", " ", " ", " ", " ", " ", " ",
    )
}

/// Returns a border compatible with Markdown table syntax.
///
/// This creates a simple border using ASCII characters that matches standard
/// Markdown table formatting. The resulting borders will be compatible with
/// Markdown renderers and plain text environments.
///
/// The border uses these characters:
/// - Horizontal lines: `-`
/// - Vertical lines: `|`
/// - All corners and junctions: `|`
///
/// # Examples
///
/// ```
/// use lipgloss::markdown_border;
///
/// let border = markdown_border();
/// println!("Markdown: {}", border.left); // prints: |
///
/// // Perfect for generating Markdown-compatible tables
/// ```
pub const fn markdown_border() -> Border {
    Border::new(
        "-", "-", "|", "|", "|", "|", "|", "|", "|", "|", "|", "|", "|",
    )
}

/// Returns a border using only ASCII characters.
///
/// This creates a simple, universally compatible border using only basic ASCII
/// characters. This border style works in any terminal or text environment,
/// making it ideal for maximum compatibility across different systems and
/// terminal emulators.
///
/// The border uses these characters:
/// - Horizontal lines: `-`
/// - Vertical lines: `|`
/// - All corners and junctions: `+`
///
/// # Examples
///
/// ```
/// use lipgloss::ascii_border;
///
/// let border = ascii_border();
/// println!("ASCII corner: {}", border.top_left); // prints: +
///
/// // Guaranteed to work in any terminal environment
/// ```
pub const fn ascii_border() -> Border {
    Border::new(
        "-", "-", "|", "|", "+", "+", "+", "+", "+", "+", "+", "+", "+",
    )
}

/// Returns a border using half-block characters positioned outside the content frame.
///
/// This creates a unique border style using Unicode half-block characters that
/// appear to sit outside the content area. The border creates a distinctive
/// 3D-like effect and is useful for creating borders that appear to "wrap around"
/// content rather than frame it.
///
/// The border uses these characters:
/// - Top/bottom: `▀▄`
/// - Left/right: `▌▐`  
/// - Corners: `▛▜▙▟`
/// - Junction points are empty (not typically used with this style)
///
/// # Examples
///
/// ```
/// use lipgloss::outer_half_block_border;
///
/// let border = outer_half_block_border();
/// println!("Top: {}", border.top); // prints: ▀
/// ```
pub const fn outer_half_block_border() -> Border {
    Border::new("▀", "▄", "▌", "▐", "▛", "▜", "▙", "▟", "", "", "", "", "")
}

/// Returns a border using half-block characters positioned inside the content frame.
///
/// This creates a subtle border style using Unicode half-block characters that
/// appear to sit inside the content area. The border creates an inset effect
/// and is useful for creating borders that appear to be "carved into" the
/// content rather than drawn around it.
///
/// The border uses these characters:
/// - Top/bottom: `▄▀`
/// - Left/right: `▐▌`
/// - Corners: `▗▖▝▘`
/// - Junction points are empty (not typically used with this style)
///
/// # Examples
///
/// ```
/// use lipgloss::inner_half_block_border;
///
/// let border = inner_half_block_border();
/// println!("Top: {}", border.top); // prints: ▄
/// ```
pub const fn inner_half_block_border() -> Border {
    Border::new("▄", "▀", "▐", "▌", "▗", "▖", "▝", "▘", "", "", "", "", "")
}

use unicode_width::UnicodeWidthChar;

/// Returns the maximum display width of any character in the given string.
///
/// This function iterates through all characters in a string and returns the
/// width of the widest character as it would appear in a terminal. This is
/// important for Unicode characters that may take up more than one column
/// (like CJK characters or certain symbols).
///
/// # Arguments
///
/// * `s` - The string to analyze
///
/// # Returns
///
/// The maximum character width in terminal columns, or 0 if all characters
/// have zero width or the string is empty.
fn max_rune_width(s: &str) -> usize {
    let mut maxw = 0usize;
    for ch in s.chars() {
        let w = UnicodeWidthChar::width(ch).unwrap_or(0);
        if w > maxw {
            maxw = w;
        }
    }
    maxw
}

/// Returns the maximum character width among multiple border components.
///
/// This function takes a slice of border component strings and returns the
/// maximum display width needed to accommodate any of them. This ensures
/// proper alignment when border components have different visual widths.
///
/// # Arguments
///
/// * `parts` - Slice of border component strings to analyze
///
/// # Returns
///
/// The maximum character width in terminal columns needed for any component.
fn get_border_edge_width(parts: &[&str]) -> usize {
    let mut maxw = 0usize;
    for p in parts {
        let w = max_rune_width(p);
        if w > maxw {
            maxw = w;
        }
    }
    maxw
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normal_border_fields_match_go() {
        let b = normal_border();
        assert_eq!(b.top, "─");
        assert_eq!(b.bottom, "─");
        assert_eq!(b.left, "│");
        assert_eq!(b.right, "│");
        assert_eq!(b.top_left, "┌");
        assert_eq!(b.top_right, "┐");
        assert_eq!(b.bottom_left, "└");
        assert_eq!(b.bottom_right, "┘");
        assert_eq!(b.middle_left, "├");
        assert_eq!(b.middle_right, "┤");
        assert_eq!(b.middle, "┼");
        assert_eq!(b.middle_top, "┬");
        assert_eq!(b.middle_bottom, "┴");
    }

    #[test]
    fn double_border_fields_match_go() {
        let b = double_border();
        assert_eq!(b.top, "═");
        assert_eq!(b.bottom, "═");
        assert_eq!(b.left, "║");
        assert_eq!(b.right, "║");
        assert_eq!(b.top_left, "╔");
        assert_eq!(b.top_right, "╗");
        assert_eq!(b.bottom_left, "╚");
        assert_eq!(b.bottom_right, "╝");
        assert_eq!(b.middle_left, "╠");
        assert_eq!(b.middle_right, "╣");
        assert_eq!(b.middle, "╬");
        assert_eq!(b.middle_top, "╦");
        assert_eq!(b.middle_bottom, "╩");
    }

    #[test]
    fn ascii_border_fields_match_go() {
        let b = ascii_border();
        assert_eq!(b.top, "-");
        assert_eq!(b.bottom, "-");
        assert_eq!(b.left, "|");
        assert_eq!(b.right, "|");
        assert_eq!(b.top_left, "+");
        assert_eq!(b.top_right, "+");
        assert_eq!(b.bottom_left, "+");
        assert_eq!(b.bottom_right, "+");
        assert_eq!(b.middle_left, "+");
        assert_eq!(b.middle_right, "+");
        assert_eq!(b.middle, "+");
        assert_eq!(b.middle_top, "+");
        assert_eq!(b.middle_bottom, "+");
    }

    #[test]
    fn hidden_border_is_spaces() {
        let b = hidden_border();
        assert_eq!(b.top, " ");
        assert_eq!(b.bottom, " ");
        assert_eq!(b.left, " ");
        assert_eq!(b.right, " ");
        assert_eq!(b.top_left, " ");
        assert_eq!(b.top_right, " ");
        assert_eq!(b.bottom_left, " ");
        assert_eq!(b.bottom_right, " ");
        assert_eq!(b.middle_left, " ");
        assert_eq!(b.middle_right, " ");
        assert_eq!(b.middle, " ");
        assert_eq!(b.middle_top, " ");
        assert_eq!(b.middle_bottom, " ");
    }

    #[test]
    fn edge_sizes_width_one_for_single_cell_borders() {
        for b in [
            normal_border(),
            rounded_border(),
            block_border(),
            thick_border(),
            double_border(),
            ascii_border(),
            markdown_border(),
            outer_half_block_border(),
            inner_half_block_border(),
        ] {
            assert_eq!(b.get_top_size(), 1);
            assert_eq!(b.get_right_size(), 1);
            assert_eq!(b.get_bottom_size(), 1);
            assert_eq!(b.get_left_size(), 1);
        }
    }

    #[test]
    fn edge_sizes_account_for_wide_runes() {
        // Use a CJK character (expected width 2 in most terminals)
        let wide = "太"; // width 2 via unicode-width
        let b = Border::new(
            wide, "-", "|", "|", "+", "+", "+", "+", "+", "+", "+", "+", "+",
        );
        assert!(
            b.get_top_size() >= 2,
            "expected top size to be >= 2 for wide rune"
        );
        // Others should be width 1
        assert_eq!(b.get_right_size(), 1);
        assert_eq!(b.get_bottom_size(), 1);
        assert_eq!(b.get_left_size(), 1);
    }

    #[test]
    fn half_block_borders_fields() {
        let outer = outer_half_block_border();
        assert_eq!(outer.top, "▀");
        assert_eq!(outer.bottom, "▄");
        assert_eq!(outer.left, "▌");
        assert_eq!(outer.right, "▐");
        assert_eq!(outer.top_left, "▛");
        assert_eq!(outer.top_right, "▜");
        assert_eq!(outer.bottom_left, "▙");
        assert_eq!(outer.bottom_right, "▟");

        let inner = inner_half_block_border();
        assert_eq!(inner.top, "▄");
        assert_eq!(inner.bottom, "▀");
        assert_eq!(inner.left, "▐");
        assert_eq!(inner.right, "▌");
        assert_eq!(inner.top_left, "▗");
        assert_eq!(inner.top_right, "▖");
        assert_eq!(inner.bottom_left, "▝");
        assert_eq!(inner.bottom_right, "▘");
    }

    #[test]
    fn edge_size_methods_match_manual_computation() {
        fn manual_max(parts: &[&str]) -> usize {
            let mut m = 0;
            for p in parts {
                let w = max_rune_width(p);
                if w > m {
                    m = w;
                }
            }
            m
        }
        let borders = [
            normal_border(),
            thick_border(),
            double_border(),
            ascii_border(),
            markdown_border(),
            outer_half_block_border(),
        ];
        for b in borders {
            assert_eq!(
                b.get_top_size(),
                manual_max(&[b.top_left, b.top, b.top_right])
            );
            assert_eq!(
                b.get_right_size(),
                manual_max(&[b.top_right, b.right, b.bottom_right])
            );
            assert_eq!(
                b.get_bottom_size(),
                manual_max(&[b.bottom_left, b.bottom, b.bottom_right])
            );
            assert_eq!(
                b.get_left_size(),
                manual_max(&[b.top_left, b.left, b.bottom_left])
            );
        }
    }

    #[test]
    fn joiners_match_go_for_remaining_presets() {
        // Rounded
        let r = rounded_border();
        assert_eq!(r.middle_left, "├");
        assert_eq!(r.middle_right, "┤");
        assert_eq!(r.middle, "┼");
        assert_eq!(r.middle_top, "┬");
        assert_eq!(r.middle_bottom, "┴");

        // Thick
        let t = thick_border();
        assert_eq!(t.middle_left, "┣");
        assert_eq!(t.middle_right, "┫");
        assert_eq!(t.middle, "╋");
        assert_eq!(t.middle_top, "┳");
        assert_eq!(t.middle_bottom, "┻");

        // Block
        let b = block_border();
        assert_eq!(b.middle_left, "█");
        assert_eq!(b.middle_right, "█");
        assert_eq!(b.middle, "█");
        assert_eq!(b.middle_top, "█");
        assert_eq!(b.middle_bottom, "█");

        // Half-blocks should have empty joiners
        let hb_outer = outer_half_block_border();
        assert_eq!(hb_outer.middle_left, "");
        assert_eq!(hb_outer.middle_right, "");
        assert_eq!(hb_outer.middle, "");
        assert_eq!(hb_outer.middle_top, "");
        assert_eq!(hb_outer.middle_bottom, "");
        let hb_inner = inner_half_block_border();
        assert_eq!(hb_inner.middle_left, "");
        assert_eq!(hb_inner.middle_right, "");
        assert_eq!(hb_inner.middle, "");
        assert_eq!(hb_inner.middle_top, "");
        assert_eq!(hb_inner.middle_bottom, "");
    }
}
