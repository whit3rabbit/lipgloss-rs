//! # Lipgloss Theme Showcase Example
//!
//! This example demonstrates the comprehensive theming capabilities of the lipgloss-rs
//! library, showcasing how `AdaptiveColor` automatically adjusts colors based on
//! terminal background detection (light vs dark themes).
//!
//! ## Features Demonstrated
//!
//! - **Adaptive Color System**: Colors that automatically switch between light and dark variants
//! - **Text Color Hierarchy**: Primary, muted, subtle, and header text colors
//! - **Status Colors**: Success, warning, error, and info indicators
//! - **Surface Colors**: Background colors for cards and elevated elements
//! - **Interactive Elements**: Accent colors for buttons and highlights
//! - **Component Integration**: Lists, tables, and complex layouts with unified theming
//! - **Theme Detection**: Automatic light/dark background detection
//!
//! ## Usage
//!
//! Run the example to see the theme showcase:
//! ```bash
//! cargo run --package theme-showcase
//! ```
//!
//! ### Testing Theme Adaptation
//!
//! Test different themes by setting the `COLORFGBG` environment variable:
//!
//! ```bash
//! # Light theme
//! COLORFGBG='0;15' cargo run --package theme-showcase
//!
//! # Dark theme  
//! COLORFGBG='15;0' cargo run --package theme-showcase
//! ```
//!
//! ## Color Palette
//!
//! The example uses a comprehensive color palette that adapts to both light and dark backgrounds:
//!
//! ### Text Colors
//! - **Primary**: High contrast text for main content
//! - **Muted**: Lower contrast for secondary information  
//! - **Subtle**: Minimal contrast for least important details
//! - **Header**: Maximum contrast for titles and headings
//!
//! ### Status Colors
//! - **Success**: Green tones for positive actions and completed states
//! - **Warning**: Orange/amber tones for caution and attention-needed states
//! - **Error**: Red tones for failures and critical issues
//! - **Info**: Blue tones for neutral information
//!
//! ### UI Colors
//! - **Accent Primary**: Brand/highlight color for primary actions
//! - **Accent Secondary**: Alternative highlight color
//! - **Interactive**: Color for clickable elements and links
//! - **Surface**: Background colors for cards and content areas
//! - **Border**: Edge colors for frames and separators
//!
//! ## Architecture
//!
//! The example demonstrates best practices for theme-aware terminal applications:
//!
//! 1. **Centralized Color Definitions**: All colors are defined as `AdaptiveColor` instances
//! 2. **Semantic Naming**: Colors are named by purpose, not appearance
//! 3. **Consistent Application**: Same color variables used across all components
//! 4. **Component Integration**: Shows how different lipgloss components work together
//! 5. **Layout Composition**: Demonstrates complex layouts with proper spacing and alignment

use lipgloss::{color::AdaptiveColor, Style, CENTER};
use lipgloss_list::{arabic, List};
use lipgloss_table::{Table, HEADER_ROW};

/// Main entry point for the lipgloss theme showcase.
///
/// This function demonstrates a comprehensive showcase of lipgloss theming capabilities,
/// including automatic color adaptation based on terminal background detection.
/// The showcase is organized into logical sections that each demonstrate different
/// aspects of the theming system.
///
/// ## Sections Demonstrated
///
/// 1. **Theme Detection Info**: Shows the detected background type and color profile
/// 2. **Text Color Hierarchy**: Primary, muted, subtle, and header text styles
/// 3. **Accent & Interactive Colors**: Brand colors and interactive element styling
/// 4. **Status Colors**: Success, warning, error, and info indicators
/// 5. **Surfaces & Borders**: Card backgrounds and border styling
/// 6. **List Components**: Styled lists with adaptive enumerators and items
/// 7. **Table Components**: Complex tables with alternating row colors and headers
/// 8. **Complex Layouts**: Multi-panel dashboard-style layouts
/// 9. **Theme Testing Instructions**: Guide for testing different themes
///
/// ## Color Adaptation
///
/// All colors automatically adapt based on the terminal's background:
/// - **Light backgrounds**: Use darker colors for good contrast
/// - **Dark backgrounds**: Use lighter colors for good contrast
///
/// The adaptation is handled automatically by the `AdaptiveColor` type, which
/// selects the appropriate color variant based on the renderer's background detection.
///
/// ## Example Output
///
/// The showcase produces a comprehensive visual demonstration showing:
/// - Color swatches with descriptions
/// - Styled text in various hierarchies
/// - Interactive components (lists, tables)
/// - Complex layouts with proper spacing
/// - Visual examples of all theming capabilities
fn main() {
    println!("🎨 Lipgloss Theme Showcase");
    println!("==========================");
    println!();

    // Show theme detection info
    let renderer = lipgloss::renderer::default_renderer();
    let is_dark = renderer.has_dark_background();
    println!(
        "🖥️  Theme Detection: {} background detected",
        if is_dark { "Dark" } else { "Light" }
    );
    println!("🎯 Color Profile: {:?}", renderer.color_profile());
    println!();

    // Define adaptive colors for the theme
    let text_primary = AdaptiveColor {
        Light: "#262626",
        Dark: "#FAFAFA",
    };
    let text_muted = AdaptiveColor {
        Light: "#737373",
        Dark: "#A3A3A3",
    };
    let text_subtle = AdaptiveColor {
        Light: "#A3A3A3",
        Dark: "#737373",
    };
    let text_header = AdaptiveColor {
        Light: "#171717",
        Dark: "#F5F5F5",
    };

    // Text Color Showcase
    print_section("Text Color Hierarchy", || {
        let content = vec![
            (
                "Primary Text",
                "Main content that users read",
                &text_primary,
            ),
            ("Muted Text", "Secondary information", &text_muted),
            ("Subtle Text", "Least important details", &text_subtle),
            ("Header Text", "Titles and headings", &text_header),
        ];

        for (label, description, color) in content {
            let demo_text = Style::new()
                .foreground(color.clone())
                .render(&format!("{}: {}", label, description));
            println!("  {}", demo_text);
        }
    });

    // Define accent colors
    let accent_primary = AdaptiveColor {
        Light: "#7C3AED",
        Dark: "#A855F7",
    };
    let accent_secondary = AdaptiveColor {
        Light: "#0891B2",
        Dark: "#06B6D4",
    };
    let interactive = AdaptiveColor {
        Light: "#DC2626",
        Dark: "#EF4444",
    };

    // Accent Color Showcase
    print_section("Accent & Interactive Colors", || {
        let accents = vec![
            (
                "Primary Accent",
                "Main brand/highlight color",
                &accent_primary,
            ),
            (
                "Secondary Accent",
                "Alternative highlights",
                &accent_secondary,
            ),
            (
                "Interactive Elements",
                "Links, buttons, clickables",
                &interactive,
            ),
        ];

        for (label, description, color) in accents {
            let demo = Style::new()
                .foreground(color.clone())
                .bold(true)
                .render(&format!("■ {} - {}", label, description));
            println!("  {}", demo);
        }
    });

    // Define status colors
    let status_success = AdaptiveColor {
        Light: "#059669",
        Dark: "#10B981",
    };
    let status_warning = AdaptiveColor {
        Light: "#D97706",
        Dark: "#F59E0B",
    };
    let status_error = AdaptiveColor {
        Light: "#DC2626",
        Dark: "#EF4444",
    };
    let status_info = AdaptiveColor {
        Light: "#2563EB",
        Dark: "#3B82F6",
    };

    // Status Color Showcase
    print_section("Status Colors", || {
        let statuses = vec![
            ("✓ Success", "Completed, positive actions", &status_success),
            ("⚠ Warning", "Caution, needs attention", &status_warning),
            ("✗ Error", "Failed, critical issues", &status_error),
            ("ℹ Info", "Neutral information", &status_info),
        ];

        for (icon_label, description, color) in statuses {
            let demo = Style::new()
                .foreground(color.clone())
                .bold(true)
                .render(&format!("{}: {}", icon_label, description));
            println!("  {}", demo);
        }
    });

    // Define surface and border colors
    let surface_subtle = AdaptiveColor {
        Light: "#F5F5F5",
        Dark: "#262626",
    };
    let surface_elevated = AdaptiveColor {
        Light: "#FFFFFF",
        Dark: "#171717",
    };
    let border_subtle = AdaptiveColor {
        Light: "#E5E5E5",
        Dark: "#404040",
    };
    let border_prominent = AdaptiveColor {
        Light: "#D4D4D8",
        Dark: "#525252",
    };

    // Surface & Border Showcase
    print_section("Surfaces & Borders", || {
        let subtle_card = Style::new()
            .background(surface_subtle.clone())
            .foreground(text_primary.clone())
            .border(lipgloss::rounded_border())
            .border_foreground(border_subtle.clone())
            .padding_2(1, 2)
            .margin_left(2)
            .render("Subtle Surface Card");

        let elevated_card = Style::new()
            .background(surface_elevated.clone())
            .foreground(text_primary.clone())
            .border(lipgloss::rounded_border())
            .border_foreground(border_prominent.clone())
            .padding_2(1, 2)
            .margin_left(2)
            .render("Elevated Surface Card");

        println!("{}", subtle_card);
        println!("{}", elevated_card);
    });

    // Define list colors
    let list_enumerator = AdaptiveColor {
        Light: "#7C3AED",
        Dark: "#A855F7",
    };
    let list_item_primary = AdaptiveColor {
        Light: "#262626",
        Dark: "#FAFAFA",
    };
    let _list_item_secondary = AdaptiveColor {
        Light: "#0891B2",
        Dark: "#06B6D4",
    };

    // List Component Showcase
    print_section("Adaptive List Components", || {
        let shopping_list = List::new()
            .items(vec!["Apples", "Bananas", "Oranges", "Grapes"])
            .enumerator(arabic)
            .enumerator_style(
                Style::new()
                    .foreground(list_enumerator.clone())
                    .margin_right(1),
            )
            .item_style(Style::new().foreground(list_item_primary.clone()));

        let task_list = List::new()
            .items(vec!["Design API", "Write Tests", "Update Docs", "Deploy"])
            .enumerator(|_items, i| {
                if i < 2 {
                    "✓".to_string()
                } else {
                    "•".to_string()
                }
            })
            .enumerator_style(
                Style::new()
                    .foreground(status_success.clone())
                    .margin_right(1),
            )
            .item_style(Style::new().foreground(text_primary.clone()));

        println!("📋 Shopping List:");
        for line in shopping_list.to_string().lines() {
            println!("  {}", line);
        }
        println!();

        println!("✅ Task Progress:");
        for line in task_list.to_string().lines() {
            println!("  {}", line);
        }
    });

    // Define table colors
    let table_border = AdaptiveColor {
        Light: "#D4D4D8",
        Dark: "#525252",
    };
    let table_header_text = AdaptiveColor {
        Light: "#FAFAFA",
        Dark: "#171717",
    };
    let table_header_bg = AdaptiveColor {
        Light: "#7C3AED",
        Dark: "#A855F7",
    };
    let table_row_text = AdaptiveColor {
        Light: "#262626",
        Dark: "#FAFAFA",
    };
    let table_row_even_bg = AdaptiveColor {
        Light: "#F9FAFB",
        Dark: "#1F1F1F",
    };

    // Table Component Showcase
    print_section("Adaptive Table Components", || {
        let mut table = Table::new()
            .headers(vec!["Status", "Task", "Priority", "Assignee"])
            .row(vec!["✓ Done", "Fix color bug", "High", "Alice"])
            .row(vec!["⚠ Review", "Update docs", "Medium", "Bob"])
            .row(vec!["• Todo", "Add tests", "High", "Charlie"])
            .row(vec!["✗ Blocked", "Deploy", "Critical", "Diana"])
            .border(lipgloss::rounded_border())
            .border_style(Style::new().foreground(table_border.clone()))
            .style_func_boxed(Box::new(move |row, _col| match row {
                HEADER_ROW => Style::new()
                    .bold(true)
                    .foreground(table_header_text.clone())
                    .background(table_header_bg.clone())
                    .align_horizontal(CENTER),
                _ if row % 2 == 0 => Style::new()
                    .foreground(table_row_text.clone())
                    .background(table_row_even_bg.clone()),
                _ => Style::new().foreground(table_row_text.clone()),
            }));

        for line in table.render().lines() {
            println!("  {}", line);
        }
    });

    // Define additional colors for complex layout
    let highlight_background = AdaptiveColor {
        Light: "#FEF3C7",
        Dark: "#451A03",
    };

    // Complex Layout Showcase
    print_section("Complex Layout Showcase", || {
        // Create a dashboard-like layout
        let title = Style::new()
            .foreground(text_header.clone())
            .bold(true)
            .align_horizontal(CENTER)
            .width(60)
            .render("📊 SYSTEM DASHBOARD");

        let status_panel = Style::new()
            .background(surface_elevated.clone())
            .foreground(text_primary.clone())
            .border(lipgloss::rounded_border())
            .border_foreground(border_prominent.clone())
            .padding_2(1, 2)
            .width(25)
            .render(&format!(
                "{}\n{}\n{}",
                Style::new()
                    .foreground(status_success.clone())
                    .bold(true)
                    .render("● Online"),
                Style::new()
                    .foreground(status_warning.clone())
                    .render("▲ 2 Warnings"),
                Style::new()
                    .foreground(status_info.clone())
                    .render("ℹ 15 Active Users")
            ));

        let alert_panel = Style::new()
            .background(highlight_background.clone())
            .foreground(text_primary.clone())
            .border(lipgloss::rounded_border())
            .border_foreground(status_error.clone())
            .padding_2(1, 2)
            .width(30)
            .render(&format!(
                "{}\n{}",
                Style::new()
                    .foreground(status_error.clone())
                    .bold(true)
                    .render("🚨 ALERT"),
                Style::new()
                    .foreground(text_primary.clone())
                    .render("Database latency high")
            ));

        println!("{}", title);
        println!();

        // Side-by-side panels using join_horizontal
        let panels = lipgloss::join_horizontal(
            lipgloss::position::TOP,
            &[
                &status_panel,
                &Style::new().width(5).render(""), // spacer
                &alert_panel,
            ],
        );

        for line in panels.lines() {
            println!("  {}", line);
        }
    });

    // Theme Testing Instructions
    print_section("Theme Testing", || {
        println!("🧪 To test theme adaptation:");
        println!("   • Try running in different terminal themes");
        println!("   • Set COLORFGBG environment variable:");
        println!("     - Light theme: COLORFGBG='0;15' cargo run");
        println!("     - Dark theme: COLORFGBG='15;0' cargo run");
        println!("   • Colors should adapt automatically!");
        println!();
        println!("🎯 All colors use AdaptiveColor for perfect theme compatibility");
    });
}

/// Prints a themed section with a styled title and content.
///
/// This helper function creates consistent section formatting throughout the showcase.
/// Each section has a styled header followed by the content, with appropriate spacing.
/// The title uses adaptive colors to ensure proper contrast in both light and dark themes.
///
/// ## Styling Applied
///
/// - **Color**: Adaptive header color (dark on light backgrounds, light on dark backgrounds)
/// - **Weight**: Bold text for emphasis
/// - **Decoration**: Underlined for clear section separation
/// - **Spacing**: Empty lines before and after content for visual separation
///
/// # Arguments
///
/// * `title` - The section title to display
/// * `content` - A closure that renders the section content when called
///
/// # Examples
///
/// ```rust
/// print_section("My Section", || {
///     println!("This is the content of my section");
///     println!("It can have multiple lines");
/// });
/// ```
///
/// This will output:
/// ```text
/// My Section (bold, underlined, adaptive color)
///
/// This is the content of my section
/// It can have multiple lines
///
/// ```
fn print_section<F>(title: &str, content: F)
where
    F: FnOnce(),
{
    let text_header = AdaptiveColor {
        Light: "#171717",
        Dark: "#F5F5F5",
    };
    let section_title = Style::new()
        .foreground(text_header)
        .bold(true)
        .underline(true)
        .render(title);

    println!("{}", section_title);
    println!();
    content();
    println!();
}
