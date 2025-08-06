// Example demonstrating the new gradient functionality in lipgloss-rs
use lipgloss::{bilinear_interpolation_grid, gradient, Style};

fn main() {
    println!("🌈 Lipgloss Gradient API Demo\n");

    // Example 1: Simple gradient
    println!("1. Simple Gradient (Red to Blue):");
    let colors = gradient("#FF0000", "#0000FF", 10);
    for (i, color) in colors.iter().enumerate() {
        let block = Style::new()
            .set_string("██")
            .foreground(color.clone())
            .render("");
        print!("{}", block);
        if (i + 1) % 5 == 0 {
            println!();
        }
    }
    println!("\n");

    // Example 2: Color grid
    println!("2. 2D Color Grid (4 corner interpolation):");
    let grid = bilinear_interpolation_grid(8, 4, ("#FF0000", "#00FF00", "#0000FF", "#FFFF00"));

    for row in grid {
        for color in row {
            let block = Style::new().set_string("██").foreground(color).render("");
            print!("{}", block);
        }
        println!();
    }
    println!();

    // Example 3: Gradient text
    println!("3. Gradient Text:");
    let text_colors = gradient("#FF6B6B", "#4ECDC4", 20);
    let text = "Gradient Text Example!";
    let mut result = String::new();

    for (i, ch) in text.chars().enumerate() {
        let color = &text_colors[i % text_colors.len()];
        let styled_char = Style::new()
            .foreground(color.clone())
            .render(&ch.to_string());
        result.push_str(&styled_char);
    }
    println!("{}\n", result);

    // Example 4: Background gradient
    println!("4. Background Gradient:");
    let bg_colors = gradient("#2C3E50", "#E74C3C", 15);
    for color in bg_colors {
        let block = Style::new().set_string("  ").background(color).render("");
        print!("{}", block);
    }
    println!("\n");

    println!("✨ All gradient features working perfectly!");
}
