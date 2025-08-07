// This example demonstrates how to use the blend_2d function to create
// beautiful 2D color gradients in a standalone lipgloss-rs application.

use lipgloss::{blend_2d, rounded_border, Color, Style, CENTER};

fn main() {
    // Define gradient presets with different angles matching the Go implementation
    let gradients = [
        (
            "Sunset Diagonal",
            vec![
                Color::from("#FF6B6B"), // Coral
                Color::from("#FFB74D"), // Orange
                Color::from("#FFDFBA"), // Peach
            ],
            45.0,
        ),
        (
            "Ocean Wave",
            vec![
                Color::from("#0077B6"), // Deep Blue
                Color::from("#48CAE4"), // Sky Blue
                Color::from("#ADE8F4"), // Light Blue
            ],
            90.0,
        ),
        (
            "Forest Mist",
            vec![
                Color::from("#228B22"), // Forest Green
                Color::from("#90EE90"), // Light Green
                Color::from("#FFFFE0"), // Cream
            ],
            135.0,
        ),
        (
            "Purple Dream",
            vec![
                Color::from("#9370DB"), // Medium Purple
                Color::from("#DDA0DD"), // Plum
                Color::from("#FFB6C1"), // Light Pink
            ],
            180.0,
        ),
        (
            "Fire Gradient",
            vec![
                Color::from("#FF0000"), // Red
                Color::from("#FFA500"), // Orange
                Color::from("#FFFF00"), // Yellow
            ],
            225.0,
        ),
    ];

    // Create styles
    let title_style = Style::new()
        .bold(true)
        .margin_bottom(1)
        .align_horizontal(CENTER);

    let gradient_style = Style::new().border(rounded_border()).margin_bottom(1);

    let gradient_name_style = Style::new().bold(true).margin_bottom(1);

    println!(
        "{}",
        title_style.render("2D Color Gradient Examples with Blend2D")
    );
    println!();

    for (name, colors, angle) in gradients {
        // Generate the gradient using blend_2d
        let width = 30;
        let height = 12;
        let blended_colors = blend_2d(width, height, angle, colors);

        // Create the gradient box using 1D row-major order
        let mut gradient_box = String::new();
        for y in 0..height {
            for x in 0..width {
                let index = y * width + x;
                let block_style = Style::new().foreground(blended_colors[index].clone());
                gradient_box.push_str(&block_style.render("█"));
            }
            if y < height - 1 {
                gradient_box.push('\n');
            }
        }

        println!(
            "{}",
            gradient_name_style.render(&format!("{} (Angle: {}°)", name, angle as i32))
        );
        println!();
        println!("{}", gradient_style.render(&gradient_box));
        println!();
    }

    println!("✨ These 2D gradients use CIELAB color space for perceptually uniform blending.");
}
