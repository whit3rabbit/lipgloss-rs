// This example demonstrates how to use the lighten and darken functions
// to create progressive brightness variations in a standalone lipgloss-rs application.

use lipgloss::{lighten, darken, Color, Style};
use std::collections::HashMap;

fn main() {
    // Base colors to demonstrate lightening and darkening
    let mut base_colors = HashMap::new();
    base_colors.insert("Red", Color::from("#FF0000"));
    base_colors.insert("Blue", Color::from("#0066FF"));
    base_colors.insert("Green", Color::from("#00FF00"));
    base_colors.insert("Gray", Color::from("#808080"));

    // Percentage to lighten/darken by
    let percentage = 0.05; // 5%

    // Number of steps to generate
    let steps = 20;

    let color_name_style = Style::new()
        .bold(true);

    println!("🌈 Color Brightness Utilities Demo\n");

    for (name, base_color) in &base_colors {
        println!("{}", color_name_style.render(name));
        println!();

        // Create lightened variations
        print!("Lightened: ");
        for i in 0..steps {
            let lightened_color = lighten(base_color, percentage * (i as f64 + 1.0));
            let style = Style::new().foreground(lightened_color);
            print!("{}", style.render("██"));
        }
        println!();

        // Create darkened variations  
        print!("Darkened:  ");
        for i in 0..steps {
            let darkened_color = darken(base_color, percentage * (i as f64 + 1.0));
            let style = Style::new().foreground(darkened_color);
            print!("{}", style.render("██"));
        }
        println!();
        println!();
    }

    println!("✨ Progressive brightness variations using lighten() and darken() functions.");
}