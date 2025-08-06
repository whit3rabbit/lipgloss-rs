use std::env;
use std::sync::{Mutex, OnceLock};

use lipgloss::renderer::*;

// Global mutex to serialize env-var dependent tests.
fn env_lock() -> &'static Mutex<()> {
    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(()))
}

fn with_env_lock<F: FnOnce()>(f: F) {
    let _g = env_lock()
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    f();
}

fn clear_env(keys: &[&str]) {
    for k in keys {
        env::remove_var(k);
    }
}

#[test]
fn detects_no_color_when_no_color_set() {
    with_env_lock(|| {
        // Arrange
        clear_env(&["COLORTERM", "TERM", "COLORFGBG"]);
        env::set_var("NO_COLOR", "1");

        // Act: use a fresh renderer
        let r = Renderer::new();
        let profile = r.color_profile();

        // Assert
        assert_eq!(profile, ColorProfileKind::NoColor);

        // Cleanup
        env::remove_var("NO_COLOR");
    });
}

#[test]
fn detects_truecolor_from_colorterm() {
    with_env_lock(|| {
        clear_env(&["NO_COLOR", "TERM", "COLORFGBG"]);
        env::set_var("COLORTERM", "truecolor");

        let r = Renderer::new();
        assert_eq!(r.color_profile(), ColorProfileKind::TrueColor);

        env::remove_var("COLORTERM");
    });
}

#[test]
fn detects_256_from_term() {
    with_env_lock(|| {
        clear_env(&["NO_COLOR", "COLORTERM", "COLORFGBG"]);
        env::set_var("TERM", "xterm-256color");

        let r = Renderer::new();
        assert_eq!(r.color_profile(), ColorProfileKind::ANSI256);

        env::remove_var("TERM");
    });
}

#[test]
fn detects_basic_color_from_term() {
    with_env_lock(|| {
        clear_env(&["NO_COLOR", "COLORTERM", "COLORFGBG"]);
        env::set_var("TERM", "xterm-color");

        let r = Renderer::new();
        assert_eq!(r.color_profile(), ColorProfileKind::ANSI);

        env::remove_var("TERM");
    });
}

#[test]
fn detects_dark_background_from_colorfgbg() {
    with_env_lock(|| {
        clear_env(&["NO_COLOR", "COLORTERM", "TERM"]);
        // bg=0 (black) => dark
        env::set_var("COLORFGBG", "15;0");
        let r = Renderer::new();
        assert!(r.has_dark_background());
        env::remove_var("COLORFGBG");
    });
}

#[test]
fn detects_light_background_from_colorfgbg() {
    with_env_lock(|| {
        clear_env(&["NO_COLOR", "COLORTERM", "TERM"]);
        // bg=7 (white) => light
        env::set_var("COLORFGBG", "0;7");
        let r = Renderer::new();
        assert!(!r.has_dark_background());
        env::remove_var("COLORFGBG");
    });
}

#[test]
fn explicit_setters_override_detection() {
    with_env_lock(|| {
        clear_env(&["NO_COLOR", "COLORTERM", "TERM", "COLORFGBG"]);
        env::set_var("TERM", "xterm-256color");

        let mut r = Renderer::new();
        // Explicitly override to NoColor and light background
        r.set_color_profile(ColorProfileKind::NoColor);
        r.set_has_dark_background(false);

        assert_eq!(r.color_profile(), ColorProfileKind::NoColor);
        assert!(!r.has_dark_background());

        env::remove_var("TERM");
    });
}

#[test]
fn default_renderer_replacement_copies_state() {
    with_env_lock(|| {
        clear_env(&["NO_COLOR", "COLORTERM", "TERM", "COLORFGBG"]);

        // Initialize default
        let _ = default_renderer();

        // Replace default with explicit settings
        let mut custom = Renderer::new();
        custom.set_color_profile(ColorProfileKind::ANSI256);
        custom.set_has_dark_background(false);
        set_default_renderer(custom);

        assert_eq!(color_profile(), ColorProfileKind::ANSI256);
        assert!(!has_dark_background());
    });
}

#[test]
fn global_setters_always_mutate_default() {
    with_env_lock(|| {
        clear_env(&["NO_COLOR", "COLORTERM", "TERM", "COLORFGBG"]);

        // Force initialization of default
        let _ = default_renderer();

        // Mutate via helpers
        set_color_profile(ColorProfileKind::ANSI);
        set_has_dark_background(true);

        assert_eq!(color_profile(), ColorProfileKind::ANSI);
        assert!(has_dark_background());
    });
}
