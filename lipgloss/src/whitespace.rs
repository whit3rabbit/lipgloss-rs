use crate::color::TerminalColor;
use crate::renderer::Renderer;
use crate::security::safe_repeat;
use crate::utils::{width as display_width, width_visible as visible_width};

/// A whitespace renderer responsible for generating styled filler areas.
///
/// `Whitespace` allows you to create customizable whitespace with various styling options
/// including foreground/background colors, underline, strikethrough, and custom characters.
/// This is particularly useful for creating visual separators, padding, or decorative
/// elements in terminal user interfaces.
///
/// The renderer cycles through provided characters to fill the requested width,
/// applying any configured styling through ANSI escape sequences.
///
/// # Examples
///
/// Basic whitespace rendering:
/// ```
/// use lipgloss::whitespace::{new_whitespace, Whitespace};
/// use lipgloss::renderer::Renderer;
///
/// let renderer = Renderer::new();
/// let ws = new_whitespace(&renderer, &[]);
/// let result = ws.render(5);
/// assert_eq!(result, "     ");
/// ```
///
/// Whitespace with custom characters:
/// ```
/// use lipgloss::whitespace::{new_whitespace, with_whitespace_chars};
/// use lipgloss::renderer::Renderer;
///
/// let renderer = Renderer::new();
/// let ws = new_whitespace(&renderer, &[with_whitespace_chars(".")]);
/// let result = ws.render(3);
/// assert_eq!(result, "...");
/// ```
#[derive(Debug)]
pub struct Whitespace {
    re: Renderer,
    style: String, // termenv.Style equivalent - will store ANSI codes
    chars: String,
}

/// Creates a new whitespace renderer with the specified options.
///
/// This function constructs a `Whitespace` instance that can generate styled
/// whitespace areas. Options are applied in order, so their sequence can matter
/// for certain combinations of styling.
///
/// # Arguments
///
/// * `r` - A reference to the `Renderer` that will handle color profiles and styling
/// * `opts` - A slice of `WhitespaceOption` functions that configure the whitespace appearance
///
/// # Returns
///
/// A new `Whitespace` instance configured with the provided options.
///
/// # Examples
///
/// Creating basic whitespace:
/// ```
/// use lipgloss::whitespace::new_whitespace;
/// use lipgloss::renderer::Renderer;
///
/// let renderer = Renderer::new();
/// let ws = new_whitespace(&renderer, &[]);
/// ```
///
/// Creating colored whitespace with custom characters:
/// ```
/// use lipgloss::whitespace::{new_whitespace, with_whitespace_chars, with_whitespace_foreground};
/// use lipgloss::renderer::Renderer;
/// use lipgloss::color::Color;
///
/// let renderer = Renderer::new();
/// let ws = new_whitespace(&renderer, &[
///     with_whitespace_chars("*"),
///     with_whitespace_foreground(Color("red".to_string())),
/// ]);
/// ```
pub fn new_whitespace(r: &Renderer, opts: &[WhitespaceOption]) -> Whitespace {
    let mut w = Whitespace {
        re: r.clone(),
        style: String::new(), // Start with empty style, will be built by options
        chars: String::new(),
    };

    for opt in opts {
        (opt)(&mut w);
    }

    w
}

impl Whitespace {
    /// Renders whitespace of the specified width with applied styling.
    ///
    /// This method generates a string of the requested width by cycling through
    /// the configured characters (defaulting to spaces) and applying any configured
    /// styling through ANSI escape sequences.
    ///
    /// The algorithm handles multi-width characters correctly and ensures the
    /// output matches the requested width exactly, padding with spaces if necessary.
    ///
    /// # Arguments
    ///
    /// * `width` - The desired width in terminal columns for the rendered whitespace
    ///
    /// # Returns
    ///
    /// A styled string of the requested width. If styling is applied, the string
    /// will include ANSI escape sequences and a reset sequence at the end.
    ///
    /// # Examples
    ///
    /// Basic rendering:
    /// ```
    /// use lipgloss::whitespace::new_whitespace;
    /// use lipgloss::renderer::Renderer;
    ///
    /// let renderer = Renderer::new();
    /// let ws = new_whitespace(&renderer, &[]);
    /// let result = ws.render(5);
    /// assert_eq!(result, "     ");
    /// ```
    ///
    /// Rendering with custom characters:
    /// ```
    /// use lipgloss::whitespace::{new_whitespace, with_whitespace_chars};
    /// use lipgloss::renderer::Renderer;
    ///
    /// let renderer = Renderer::new();
    /// let ws = new_whitespace(&renderer, &[with_whitespace_chars("ab")]);
    /// let result = ws.render(5);
    /// assert_eq!(result, "ababa");
    /// ```
    pub fn render(&self, width: usize) -> String {
        let chars = if self.chars.is_empty() {
            " "
        } else {
            &self.chars
        };

        let runes: Vec<char> = chars.chars().collect();
        let mut j: usize = 0;
        let mut output = String::new();

        // Cycle through runes and print them into the whitespace.
        let mut i: usize = 0;
        while i < width {
            let ch = runes[j];
            // Determine the width of the next rune before appending to avoid overshoot.
            let ch_width = display_width(&ch.to_string());
            if i + ch_width > width {
                break;
            }
            output.push(ch);
            j += 1;
            if j >= runes.len() {
                j = 0;
            }
            i += ch_width;
        }

        // Fill any extra gaps with spaces. This might be necessary if any runes
        // are more than one cell wide, which could leave a one-rune gap.
        let content_width = visible_width(&output);
        let short = width.saturating_sub(content_width);
        if short > 0 {
            output.push_str(&safe_repeat(' ', short));
        }

        // Apply styling like Go's w.style.Styled(b.String())
        if !self.style.is_empty() {
            format!("{}{}\x1b[0m", self.style, output)
        } else {
            output
        }
    }
}

/// A configuration option for customizing whitespace appearance and behavior.
///
/// `WhitespaceOption` is a type alias for boxed closures that modify a `Whitespace`
/// instance. This design allows options to capture values (like colors or characters)
/// and apply them when the whitespace is being configured.
///
/// Options can be combined and are applied in the order they are provided to
/// `new_whitespace()`. Some options may build upon each other (like combining
/// foreground and background colors).
///
/// # Examples
///
/// Creating a custom option:
/// ```
/// use lipgloss::whitespace::{WhitespaceOption, Whitespace};
///
/// fn custom_option() -> WhitespaceOption {
///     Box::new(|w: &mut Whitespace| {
///         // Custom configuration logic here
///     })
/// }
/// ```
pub type WhitespaceOption = Box<dyn Fn(&mut Whitespace)>;

/// Sets the foreground (text) color for whitespace characters.
///
/// This function creates a `WhitespaceOption` that applies the specified color
/// to the characters rendered in the whitespace. The color is applied using
/// ANSI escape sequences compatible with the terminal's color profile.
///
/// # Arguments
///
/// * `c` - Any type implementing `TerminalColor`, such as `Color` or `AdaptiveColor`
///
/// # Returns
///
/// A `WhitespaceOption` that can be passed to `new_whitespace()`.
///
/// # Examples
///
/// ```
/// use lipgloss::whitespace::{new_whitespace, with_whitespace_foreground};
/// use lipgloss::renderer::Renderer;
/// use lipgloss::color::Color;
///
/// let renderer = Renderer::new();
/// let ws = new_whitespace(&renderer, &[
///     with_whitespace_foreground(Color("red".to_string()))
/// ]);
/// let result = ws.render(3);
/// // Result will include ANSI color codes for red text
/// ```
pub fn with_whitespace_foreground<C: TerminalColor + 'static>(c: C) -> WhitespaceOption {
    Box::new(move |w: &mut Whitespace| {
        let fg_color = c.token(&w.re);
        if fg_color.is_empty() {
            // NoColor profile returns empty token; do not add malformed SGR.
            return;
        }
        // Decide between 24-bit truecolor and indexed color based on token format.
        // token starting with '#' denotes hex RGB (e.g. "#383838").
        let new_seg = if let Some(hex) = fg_color.strip_prefix('#') {
            // Parse #RRGGBB or #RRGGBBAA; ignore alpha if present.
            // Fallback to 0 on parse errors.
            let (r, g, b) = if hex.len() >= 6 {
                let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(0);
                let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(0);
                let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(0);
                (r, g, b)
            } else {
                (0, 0, 0)
            };
            format!("38;2;{};{};{}", r, g, b)
        } else {
            // Numeric index for ANSI/ANSI256
            format!("38;5;{}", fg_color)
        };

        if w.style.is_empty() {
            w.style = format!("\x1b[{}m", new_seg);
        } else {
            let base = w.style.trim_end_matches('m');
            w.style = format!("{};{}m", base, new_seg);
        }
    })
}

/// Sets the background color for whitespace characters.
///
/// This function creates a `WhitespaceOption` that applies the specified background
/// color to the whitespace area. The color is applied using ANSI escape sequences
/// compatible with the terminal's color profile.
///
/// # Arguments
///
/// * `c` - Any type implementing `TerminalColor`, such as `Color` or `AdaptiveColor`
///
/// # Returns
///
/// A `WhitespaceOption` that can be passed to `new_whitespace()`.
///
/// # Examples
///
/// ```
/// use lipgloss::whitespace::{new_whitespace, with_whitespace_background};
/// use lipgloss::renderer::Renderer;
/// use lipgloss::color::Color;
///
/// let renderer = Renderer::new();
/// let ws = new_whitespace(&renderer, &[
///     with_whitespace_background(Color("blue".to_string()))
/// ]);
/// let result = ws.render(3);
/// // Result will include ANSI color codes for blue background
/// ```
pub fn with_whitespace_background<C: TerminalColor + 'static>(c: C) -> WhitespaceOption {
    Box::new(move |w: &mut Whitespace| {
        let bg_color = c.token(&w.re);
        if bg_color.is_empty() {
            // NoColor profile returns empty token; do not add malformed SGR.
            return;
        }
        let new_seg = if let Some(hex) = bg_color.strip_prefix('#') {
            let (r, g, b) = if hex.len() >= 6 {
                let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(0);
                let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(0);
                let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(0);
                (r, g, b)
            } else {
                (0, 0, 0)
            };
            format!("48;2;{};{};{}", r, g, b)
        } else {
            format!("48;5;{}", bg_color)
        };

        if w.style.is_empty() {
            w.style = format!("\x1b[{}m", new_seg);
        } else {
            let base = w.style.trim_end_matches('m');
            w.style = format!("{};{}m", base, new_seg);
        }
    })
}

/// Applies underline styling to whitespace characters.
///
/// This function creates a `WhitespaceOption` that adds underline text decoration
/// to the rendered whitespace using ANSI SGR (Select Graphic Rendition) codes.
/// The underline will be visible in terminals that support this styling.
///
/// # Returns
///
/// A `WhitespaceOption` that can be passed to `new_whitespace()`.
///
/// # Examples
///
/// ```
/// use lipgloss::whitespace::{new_whitespace, with_whitespace_underline, with_whitespace_chars};
/// use lipgloss::renderer::Renderer;
///
/// let renderer = Renderer::new();
/// let ws = new_whitespace(&renderer, &[
///     with_whitespace_chars("─"),
///     with_whitespace_underline()
/// ]);
/// let result = ws.render(5);
/// // Result will include ANSI codes for underlined characters
/// ```
pub fn with_whitespace_underline() -> WhitespaceOption {
    Box::new(move |w: &mut Whitespace| {
        if w.style.is_empty() {
            w.style = "\x1b[4m".to_string();
        } else {
            let base = w.style.trim_end_matches('m');
            w.style = format!("{};4m", base);
        }
    })
}

/// Applies strikethrough styling to whitespace characters.
///
/// This function creates a `WhitespaceOption` that adds strikethrough text decoration
/// to the rendered whitespace using ANSI SGR (Select Graphic Rendition) codes.
/// The strikethrough will be visible in terminals that support this styling.
///
/// # Returns
///
/// A `WhitespaceOption` that can be passed to `new_whitespace()`.
///
/// # Examples
///
/// ```
/// use lipgloss::whitespace::{new_whitespace, with_whitespace_strikethrough, with_whitespace_chars};
/// use lipgloss::renderer::Renderer;
///
/// let renderer = Renderer::new();
/// let ws = new_whitespace(&renderer, &[
///     with_whitespace_chars("text"),
///     with_whitespace_strikethrough()
/// ]);
/// let result = ws.render(8);
/// // Result will include ANSI codes for strikethrough text
/// ```
pub fn with_whitespace_strikethrough() -> WhitespaceOption {
    Box::new(move |w: &mut Whitespace| {
        if w.style.is_empty() {
            w.style = "\x1b[9m".to_string();
        } else {
            let base = w.style.trim_end_matches('m');
            w.style = format!("{};9m", base);
        }
    })
}

/// Sets custom characters to be used for rendering whitespace.
///
/// This function creates a `WhitespaceOption` that replaces the default space
/// characters with the specified string. The characters will be cycled through
/// to fill the requested width. This is useful for creating decorative borders,
/// patterns, or visual separators.
///
/// # Arguments
///
/// * `s` - A string containing the characters to cycle through. Can be a single
///   character or multiple characters that will be repeated in sequence.
///
/// # Returns
///
/// A `WhitespaceOption` that can be passed to `new_whitespace()`.
///
/// # Examples
///
/// Single character pattern:
/// ```
/// use lipgloss::whitespace::{new_whitespace, with_whitespace_chars};
/// use lipgloss::renderer::Renderer;
///
/// let renderer = Renderer::new();
/// let ws = new_whitespace(&renderer, &[with_whitespace_chars(".")]);
/// let result = ws.render(5);
/// assert_eq!(result, ".....");
/// ```
///
/// Multi-character pattern:
/// ```
/// use lipgloss::whitespace::{new_whitespace, with_whitespace_chars};
/// use lipgloss::renderer::Renderer;
///
/// let renderer = Renderer::new();
/// let ws = new_whitespace(&renderer, &[with_whitespace_chars("ab")]);
/// let result = ws.render(5);
/// assert_eq!(result, "ababa");
/// ```
pub fn with_whitespace_chars(s: &str) -> WhitespaceOption {
    let chars = s.to_string();
    Box::new(move |w: &mut Whitespace| {
        w.chars = chars.clone();
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::Color;
    use crate::renderer::{ColorProfileKind, Renderer};

    #[test]
    fn test_whitespace_basic_render() {
        let renderer = Renderer::new();
        let ws = new_whitespace(&renderer, &[]);
        let result = ws.render(5);
        assert_eq!(result, "     ");
    }

    #[test]
    fn test_whitespace_with_custom_chars() {
        let renderer = Renderer::new();
        let ws = new_whitespace(&renderer, &[with_whitespace_chars(".")]);
        let result = ws.render(3);
        assert_eq!(result, "...");
    }

    #[test]
    fn test_whitespace_with_foreground_color() {
        let mut renderer = Renderer::new();
        renderer.set_color_profile(ColorProfileKind::ANSI256);
        let color = Color("9".to_string());
        let ws = new_whitespace(&renderer, &[with_whitespace_foreground(color)]);
        let result = ws.render(3);
        // Should include ANSI color codes and reset
        assert!(result.starts_with("\x1b[38;5;9m"));
        assert!(result.ends_with("\x1b[0m"));
        assert!(result.contains("   "));
    }

    #[test]
    fn test_whitespace_matches_go_algorithm() {
        let renderer = Renderer::new();
        let ws = new_whitespace(&renderer, &[with_whitespace_chars("ab")]);
        // Should cycle through "ab" characters
        let result = ws.render(5);
        assert_eq!(result, "ababa");
    }

    #[test]
    fn test_whitespace_struct_matches_go() {
        // Test that our struct fields match the Go implementation conceptually
        let renderer = Renderer::new();
        let ws = new_whitespace(
            &renderer,
            &[
                with_whitespace_chars("*"),
                with_whitespace_foreground(Color("1".to_string())),
            ],
        );

        // Should have renderer reference (re field in Go)
        // Should have style information (style field in Go)
        // Should have chars information (chars field in Go)
        let result = ws.render(2);
        assert!(result.len() > 2); // Should include ANSI codes
        assert!(result.contains("**"));
    }

    #[test]
    fn test_whitespace_combined_fg_bg_builds_complete_sgr() {
        let mut renderer = Renderer::new();
        renderer.set_color_profile(ColorProfileKind::ANSI256);
        let ws = new_whitespace(
            &renderer,
            &[
                with_whitespace_foreground(Color("1".to_string())),
                with_whitespace_background(Color("2".to_string())),
            ],
        );
        let out = ws.render(1);
        // Expect prefix ESC[38;5;1;48;5;2m, then at least one char, then ESC[0m
        let prefix = "\x1b[38;5;1;48;5;2m";
        assert!(out.starts_with(prefix));
        assert!(out.ends_with("\x1b[0m"));
        assert!(out.len() > prefix.len() + "\x1b[0m".len());
    }
}
