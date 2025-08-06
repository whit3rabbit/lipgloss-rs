//! Rendering engine with terminal color profile detection and background detection.
//!
//! This module provides the [`Renderer`] type which manages terminal-specific rendering
//! settings such as color profiles and background brightness. It automatically detects
//! terminal capabilities and provides methods to override the detected settings.
//!
//! # Color Profiles
//!
//! The renderer supports multiple color profiles:
//! - [`ColorProfileKind::TrueColor`] - 24-bit true color support
//! - [`ColorProfileKind::ANSI256`] - 256 color support
//! - [`ColorProfileKind::ANSI`] - Basic 16 color support
//! - [`ColorProfileKind::NoColor`] - No color support
//!
//! # Examples
//!
//! ```
//! use lipgloss::renderer::{Renderer, ColorProfileKind};
//!
//! // Use the default renderer
//! let profile = lipgloss::renderer::color_profile();
//!
//! // Create a custom renderer
//! let mut renderer = Renderer::new();
//! renderer.set_color_profile(ColorProfileKind::TrueColor);
//! renderer.set_has_dark_background(true);
//! ```
//!
//! # Global Default Renderer
//!
//! The module provides a global default renderer that can be accessed via
//! [`default_renderer()`] and configured via [`set_color_profile()`] and
//! [`set_has_dark_background()`].

use std::sync::{Arc, OnceLock, RwLock};

/// Color profiles supported by the renderer. Mirrors termenv's profiles.
///
/// This enum represents the different color capabilities that a terminal may support,
/// from no color support to full 24-bit true color. The renderer automatically
/// detects the appropriate profile based on environment variables and terminal
/// capabilities.
///
/// # Examples
///
/// ```
/// use lipgloss::renderer::{Renderer, ColorProfileKind};
///
/// let mut renderer = Renderer::new();
///
/// // Set to use 256 colors
/// renderer.set_color_profile(ColorProfileKind::ANSI256);
///
/// // Check current profile
/// match renderer.color_profile() {
///     ColorProfileKind::TrueColor => println!("Full RGB color support!"),
///     ColorProfileKind::ANSI256 => println!("256 color support"),
///     ColorProfileKind::ANSI => println!("Basic 16 color support"),
///     ColorProfileKind::NoColor => println!("No color support"),
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorProfileKind {
    /// 24-bit true color support (16.7 million colors).
    ///
    /// This profile is detected when `COLORTERM` contains "truecolor" or "24bit".
    TrueColor,

    /// 256 color support.
    ///
    /// This profile is detected when `TERM` contains "256color".
    ANSI256,

    /// Basic ANSI 16 color support.
    ///
    /// This profile is detected when `TERM` contains "color" but not "256color".
    ANSI,

    /// No color support.
    ///
    /// This profile is used when `NO_COLOR` is set, the terminal doesn't support
    /// ANSI, or no color capability is detected.
    NoColor,
}

/// Renderer stores environment-specific rendering options such as
/// the detected color profile and whether the terminal background is dark.
///
/// The renderer automatically detects terminal capabilities on first use,
/// but all settings can be explicitly overridden. It uses lazy initialization
/// to avoid unnecessary environment queries until the settings are actually needed.
///
/// # Thread Safety
///
/// The `Renderer` is thread-safe and can be safely shared between threads.
/// It uses interior mutability with `Arc<RwLock<_>>` for safe concurrent access.
///
/// # Examples
///
/// ```
/// use lipgloss::renderer::{Renderer, ColorProfileKind};
///
/// // Create a new renderer with auto-detection
/// let renderer = Renderer::new();
///
/// // Check detected settings
/// println!("Color profile: {:?}", renderer.color_profile());
/// println!("Dark background: {}", renderer.has_dark_background());
///
/// // Create a renderer with custom settings
/// let mut custom_renderer = Renderer::new();
/// custom_renderer.set_color_profile(ColorProfileKind::TrueColor);
/// custom_renderer.set_has_dark_background(false);
/// ```
#[derive(Debug, Clone)]
pub struct Renderer {
    inner: Arc<RwLock<Inner>>,
}

#[derive(Debug)]
struct Inner {
    // Output handle (placeholder for termenv Output)
    output: Option<Output>,

    // Color profile state
    color_profile: ColorProfileKind,
    explicit_color_profile: bool,
    color_profile_once: OnceLock<()>,

    // Background brightness state
    has_dark_background: bool,
    explicit_background: bool,
    has_dark_background_once: OnceLock<()>,
}

impl Default for Renderer {
    fn default() -> Self {
        // Lazy detection via OnceLock: start with placeholders and detect on
        // first getter call unless explicitly set by the user.
        let inner = Inner {
            output: Some(detect_output()),
            color_profile: ColorProfileKind::ANSI, // placeholder
            explicit_color_profile: false,
            color_profile_once: OnceLock::new(),
            has_dark_background: true, // common case; will be lazily detected
            explicit_background: false,
            has_dark_background_once: OnceLock::new(),
        };
        Self {
            inner: Arc::new(RwLock::new(inner)),
        }
    }
}

impl Renderer {
    /// Create a new renderer with automatic terminal detection.
    ///
    /// This creates a renderer that will automatically detect the terminal's
    /// color profile and background brightness on first use. The detection
    /// is lazy, meaning it won't query the environment until the settings
    /// are actually accessed.
    ///
    /// # Examples
    ///
    /// ```
    /// use lipgloss::renderer::Renderer;
    ///
    /// let renderer = Renderer::new();
    /// // Settings will be detected on first access
    /// let profile = renderer.color_profile();
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a new renderer with a specific output configuration.
    ///
    /// This allows you to create a renderer with custom output settings,
    /// useful for testing or when you need to override the default output
    /// detection.
    ///
    /// # Arguments
    ///
    /// * `o` - The output configuration to use
    ///
    /// # Examples
    ///
    /// ```
    /// use lipgloss::renderer::{Renderer, Output};
    ///
    /// let output = Output {
    ///     supports_ansi: true,
    ///     is_tty_like: true,
    /// };
    /// let renderer = Renderer::new_with_output(output);
    /// ```
    pub fn new_with_output(o: Output) -> Self {
        let r = Self::default();
        if let Ok(mut inner) = r.inner.write() {
            inner.output = Some(o);
        }
        r
    }

    /// Get the current color profile.
    ///
    /// If the color profile hasn't been explicitly set, this will trigger
    /// automatic detection based on environment variables:
    /// - `NO_COLOR` - If set, returns `NoColor`
    /// - `COLORTERM` - Checked for "truecolor" or "24bit"
    /// - `TERM` - Checked for "256color" or "color"
    ///
    /// # Returns
    ///
    /// The detected or explicitly set color profile.
    ///
    /// # Examples
    ///
    /// ```
    /// use lipgloss::renderer::Renderer;
    ///
    /// let renderer = Renderer::new();
    /// let profile = renderer.color_profile();
    /// println!("Terminal supports: {:?}", profile);
    /// ```
    pub fn color_profile(&self) -> ColorProfileKind {
        // Lazy init if not explicitly set
        let mut guard = self.inner.write().expect("renderer lock poisoned");
        if !guard.explicit_color_profile && guard.color_profile_once.get().is_none() {
            let output_ref = guard.output.as_ref();
            guard.color_profile = detect_color_profile(output_ref);
            let _ = guard.color_profile_once.set(());
        }
        guard.color_profile
    }

    /// Set the color profile explicitly.
    ///
    /// This overrides any automatic detection and marks the profile as
    /// explicitly set, preventing future automatic detection.
    ///
    /// # Arguments
    ///
    /// * `p` - The color profile to use
    ///
    /// # Examples
    ///
    /// ```
    /// use lipgloss::renderer::{Renderer, ColorProfileKind};
    ///
    /// let mut renderer = Renderer::new();
    /// renderer.set_color_profile(ColorProfileKind::ANSI256);
    /// assert_eq!(renderer.color_profile(), ColorProfileKind::ANSI256);
    /// ```
    pub fn set_color_profile(&mut self, p: ColorProfileKind) {
        let mut guard = self.inner.write().expect("renderer lock poisoned");
        guard.color_profile = p;
        guard.explicit_color_profile = true;
    }

    /// Check if the terminal has a dark background.
    ///
    /// If not explicitly set, this will trigger automatic detection based on
    /// the `COLORFGBG` environment variable. The detection interprets ANSI
    /// color codes 0-6 as dark backgrounds and 7+ as light backgrounds.
    ///
    /// # Returns
    ///
    /// `true` if the background is dark, `false` if light. Defaults to `true`
    /// if detection fails.
    ///
    /// # Examples
    ///
    /// ```
    /// use lipgloss::renderer::Renderer;
    ///
    /// let renderer = Renderer::new();
    /// if renderer.has_dark_background() {
    ///     println!("Using colors suitable for dark backgrounds");
    /// }
    /// ```
    pub fn has_dark_background(&self) -> bool {
        let mut guard = self.inner.write().expect("renderer lock poisoned");
        if !guard.explicit_background && guard.has_dark_background_once.get().is_none() {
            guard.has_dark_background = detect_dark_background();
            let _ = guard.has_dark_background_once.set(());
        }
        guard.has_dark_background
    }

    /// Set whether the terminal has a dark background.
    ///
    /// This overrides any automatic detection and marks the background
    /// brightness as explicitly set.
    ///
    /// # Arguments
    ///
    /// * `b` - `true` for dark background, `false` for light
    ///
    /// # Examples
    ///
    /// ```
    /// use lipgloss::renderer::Renderer;
    ///
    /// let mut renderer = Renderer::new();
    /// renderer.set_has_dark_background(false); // Light background
    /// assert!(!renderer.has_dark_background());
    /// ```
    pub fn set_has_dark_background(&mut self, b: bool) {
        let mut guard = self.inner.write().expect("renderer lock poisoned");
        guard.has_dark_background = b;
        guard.explicit_background = true;
    }

    /// Return the output handle (owned clone for safe access).
    ///
    /// This returns a clone of the output configuration if one is set.
    ///
    /// # Returns
    ///
    /// An optional clone of the output configuration.
    ///
    /// # Examples
    ///
    /// ```
    /// use lipgloss::renderer::Renderer;
    ///
    /// let renderer = Renderer::new();
    /// if let Some(output) = renderer.output() {
    ///     println!("ANSI support: {}", output.supports_ansi);
    /// }
    /// ```
    pub fn output(&self) -> Option<Output> {
        self.inner
            .read()
            .ok()
            .and_then(|g| g.output.as_ref().cloned())
    }

    /// Set the output configuration.
    ///
    /// This allows changing the output configuration after renderer creation.
    ///
    /// # Arguments
    ///
    /// * `o` - The new output configuration
    ///
    /// # Examples
    ///
    /// ```
    /// use lipgloss::renderer::{Renderer, Output};
    ///
    /// let mut renderer = Renderer::new();
    /// let output = Output {
    ///     supports_ansi: false,
    ///     is_tty_like: false,
    /// };
    /// renderer.set_output(output);
    /// ```
    pub fn set_output(&mut self, o: Output) {
        if let Ok(mut guard) = self.inner.write() {
            guard.output = Some(o);
        }
    }

    /// Back-compat alias for older internal callsites (if any).
    ///
    /// This is a compatibility method that simply calls [`output()`](Self::output).
    ///
    /// # Returns
    ///
    /// An optional clone of the output configuration.
    pub fn take_output_clone(&self) -> Option<Output> {
        self.output()
    }
}

// ---- Global default renderer (package-level behavior like Go) ----

static DEFAULT_RENDERER: OnceLock<Renderer> = OnceLock::new();

fn default_renderer_cell() -> &'static Renderer {
    DEFAULT_RENDERER.get_or_init(Renderer::default)
}

/// Get a reference to the default renderer.
///
/// This returns a reference to the global default renderer, which is lazily
/// initialized on first access. The default renderer uses automatic detection
/// for color profile and background brightness.
///
/// # Examples
///
/// ```
/// use lipgloss::renderer::default_renderer;
///
/// let renderer = default_renderer();
/// println!("Default color profile: {:?}", renderer.color_profile());
/// ```
pub fn default_renderer() -> &'static Renderer {
    default_renderer_cell()
}

/// Replace the global default renderer.
///
/// This attempts to set a new default renderer. Due to the use of `OnceLock`,
/// this will only succeed if the default renderer hasn't been initialized yet.
/// If it has already been initialized, this function has no effect.
///
/// # Arguments
///
/// * `r` - The new renderer to use as default
///
/// # Examples
///
/// ```
/// use lipgloss::renderer::{Renderer, ColorProfileKind, set_default_renderer};
///
/// // This only works if called before any other renderer operations
/// let mut custom = Renderer::new();
/// custom.set_color_profile(ColorProfileKind::NoColor);
/// set_default_renderer(custom);
/// ```
pub fn set_default_renderer(r: Renderer) {
    // Note: OnceLock cannot be reset. If it's not set, we set it.
    // If it is set, we mutate the existing singleton to mirror replacement.
    if DEFAULT_RENDERER.set(r.clone()).is_err() {
        if let Some(existing) = DEFAULT_RENDERER.get() {
            // Copy state from provided renderer into the existing one.
            if let (Ok(mut dst), Ok(src)) = (existing.inner.write(), r.inner.read()) {
                dst.output = src.output.clone();
                dst.color_profile = src.color_profile;
                dst.explicit_color_profile = src.explicit_color_profile;
                if src.color_profile_once.get().is_some() {
                    let _ = dst.color_profile_once.set(());
                }
                dst.has_dark_background = src.has_dark_background;
                dst.explicit_background = src.explicit_background;
                if src.has_dark_background_once.get().is_some() {
                    let _ = dst.has_dark_background_once.set(());
                }
            }
        }
    }
}

/// Get the current color profile from the default renderer.
///
/// This is a convenience function that calls [`color_profile()`](Renderer::color_profile)
/// on the default renderer.
///
/// # Returns
///
/// The color profile of the default renderer.
///
/// # Examples
///
/// ```
/// use lipgloss::renderer::{color_profile, ColorProfileKind};
///
/// match color_profile() {
///     ColorProfileKind::TrueColor => println!("Terminal supports 24-bit color"),
///     ColorProfileKind::ANSI256 => println!("Terminal supports 256 colors"),
///     ColorProfileKind::ANSI => println!("Terminal supports 16 colors"),
///     ColorProfileKind::NoColor => println!("No color support"),
/// }
/// ```
pub fn color_profile() -> ColorProfileKind {
    default_renderer().color_profile()
}

/// Set the color profile on the default renderer.
///
/// This will always mutate the default renderer, regardless of initialization
/// order.
///
/// # Arguments
///
/// * `p` - The color profile to set
///
/// # Examples
///
/// ```
/// use lipgloss::renderer::{set_color_profile, ColorProfileKind};
///
/// // Call this early in your program
/// set_color_profile(ColorProfileKind::ANSI256);
/// ```
pub fn set_color_profile(p: ColorProfileKind) {
    // Ensure default exists, then mutate it regardless of initialization order.
    let _ = DEFAULT_RENDERER.get_or_init(Renderer::default);
    if let Some(r) = DEFAULT_RENDERER.get() {
        if let Ok(mut inner) = r.inner.write() {
            inner.color_profile = p;
            inner.explicit_color_profile = true;
            let _ = inner.color_profile_once.set(());
        }
    }
}

/// Query whether the default renderer assumes a dark background.
///
/// This is a convenience function that calls [`has_dark_background()`](Renderer::has_dark_background)
/// on the default renderer.
///
/// # Returns
///
/// `true` if the terminal has a dark background, `false` otherwise.
///
/// # Examples
///
/// ```
/// use lipgloss::renderer::has_dark_background;
///
/// if has_dark_background() {
///     println!("Using bright colors for dark background");
/// } else {
///     println!("Using dark colors for light background");
/// }
/// ```
pub fn has_dark_background() -> bool {
    default_renderer().has_dark_background()
}

/// Set dark background flag on the default renderer.
///
/// This attempts to set the background brightness on the default renderer. If the
/// default renderer has already been initialized, this function has no effect.
/// To ensure this works, call it before any other renderer operations.
///
/// # Arguments
///
/// * `b` - `true` for dark background, `false` for light
///
/// # Examples
///
/// ```
/// use lipgloss::renderer::set_has_dark_background;
///
/// // Call this early in your program
/// set_has_dark_background(false); // Light background
/// ```
pub fn set_has_dark_background(b: bool) {
    let _ = DEFAULT_RENDERER.get_or_init(Renderer::default);
    if let Some(r) = DEFAULT_RENDERER.get() {
        if let Ok(mut inner) = r.inner.write() {
            inner.has_dark_background = b;
            inner.explicit_background = true;
            let _ = inner.has_dark_background_once.set(());
        }
    }
}

// --- Concrete Output abstraction (termenv-like) ---
/// Output configuration for terminal capabilities.
///
/// This struct represents the capabilities of an output stream, such as
/// whether it supports ANSI escape codes and whether it behaves like a TTY.
///
/// # Examples
///
/// ```
/// use lipgloss::renderer::Output;
///
/// // Create an output for a terminal that supports colors
/// let terminal_output = Output {
///     supports_ansi: true,
///     is_tty_like: true,
/// };
///
/// // Create an output for a file or pipe
/// let file_output = Output {
///     supports_ansi: false,
///     is_tty_like: false,
/// };
/// ```
#[derive(Debug, Clone)]
pub struct Output {
    /// Whether the output supports ANSI escape sequences.
    ///
    /// This is typically `true` for terminals and `false` for files or pipes.
    pub supports_ansi: bool,

    /// Whether the output behaves like a TTY (terminal).
    ///
    /// This affects various behaviors such as whether to use colors by default.
    pub is_tty_like: bool,
}

// ---- Minimal environment detection helpers ----

fn detect_color_profile(output: Option<&Output>) -> ColorProfileKind {
    // Respect NO_COLOR first.
    let no_color = std::env::var("NO_COLOR").ok().is_some();
    if no_color {
        return ColorProfileKind::NoColor;
    }

    let colorterm = std::env::var("COLORTERM")
        .unwrap_or_default()
        .to_lowercase();
    let term = std::env::var("TERM").unwrap_or_default().to_lowercase();

    // Check environment variables first for color capability
    if colorterm.contains("truecolor") || colorterm.contains("24bit") {
        return ColorProfileKind::TrueColor;
    }
    if term.contains("256color") {
        return ColorProfileKind::ANSI256;
    }
    if term.contains("color") {
        return ColorProfileKind::ANSI;
    }

    // Only check output support if no color capability was detected from env vars
    if let Some(out) = output {
        if !out.supports_ansi {
            return ColorProfileKind::NoColor;
        }
    }

    ColorProfileKind::NoColor
}

fn detect_dark_background() -> bool {
    // Use COLORFGBG if available (format: "<fg>;<bg>") where bg 0-7 are ANSI
    // basic colors. Treat 0-6 as dark, 7 as light. If more than two values,
    // use the last as background.
    if let Ok(val) = std::env::var("COLORFGBG") {
        let parts: Vec<&str> = val.split(';').collect();
        if let Some(bg_str) = parts.last() {
            if let Ok(bg) = bg_str.parse::<u8>() {
                // 0..=6 dark, >=7 light (7 is white)
                return bg <= 6;
            }
        }
    }
    // Default to dark background, matching common terminals and Go fallback.
    true
}

fn detect_output() -> Output {
    // Check if stdout is a terminal
    use std::io::IsTerminal;
    let is_tty_like = std::io::stdout().is_terminal();

    // For ANSI support, check if it's a terminal and NO_COLOR is not set
    let no_color = std::env::var("NO_COLOR").is_ok();
    let supports_ansi = is_tty_like && !no_color;

    Output {
        supports_ansi,
        is_tty_like,
    }
}
