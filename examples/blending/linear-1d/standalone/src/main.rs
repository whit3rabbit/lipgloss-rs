// This example demonstrates how to use the blend_1d function to create
// beautiful color gradients in a standalone lipgloss-rs application.

use lipgloss::{blend_1d, rounded_border, Color, Style, CENTER};

fn main() {
    // Define gradient presets matching the Go implementation
    let gradients = [
        (
            "Sunset",
            vec![
                Color::from("#FF6B6B"), // Coral
                Color::from("#FFB74D"), // Orange
                Color::from("#FFDFBA"), // Peach
            ],
        ),
        (
            "Ocean",
            vec![
                Color::from("#0077B6"), // Deep Blue
                Color::from("#48CAE4"), // Sky Blue
                Color::from("#ADE8F4"), // Light Blue
            ],
        ),
        (
            "Forest",
            vec![
                Color::from("#228B22"), // Forest Green
                Color::from("#90EE90"), // Light Green
                Color::from("#FFFFE0"), // Cream
            ],
        ),
        (
            "Purple Dream",
            vec![
                Color::from("#9370DB"), // Medium Purple
                Color::from("#DDA0DD"), // Plum
                Color::from("#FFB6C1"), // Light Pink
            ],
        ),
        (
            "Fire",
            vec![
                Color::from("#FF0000"), // Red
                Color::from("#FFA500"), // Orange
                Color::from("#FFFF00"), // Yellow
            ],
        ),
    ];

    // Create styles for light/dark background detection
    let title_style = Style::new()
        .bold(true)
        .margin_bottom(1)
        .align_horizontal(CENTER);

    let gradient_style = Style::new().border(rounded_border()).margin_bottom(1);

    println!(
        "{}",
        title_style.render("Color Gradient Examples with Blend1D")
    );
    println!();

    for (name, colors) in gradients {
        // Generate the gradient using blend_1d
        let blended_colors = blend_1d(40, colors);

        // Create the gradient bar
        let mut gradient_bar = String::new();
        for color in blended_colors {
            let block_style = Style::new().foreground(color);
            gradient_bar.push_str(&block_style.render("█"));
        }

        println!("{}", gradient_style.render(&gradient_bar));
        println!("   {}", name);
        println!();
    }

    println!("✨ These gradients use CIELAB color space for perceptually uniform blending.");
}
