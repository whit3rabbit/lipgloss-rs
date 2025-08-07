use lipgloss::{alpha, blend_1d, blend_2d, complementary, darken, lighten, Color, Style};

fn main() {
    println!("🎨 Lipgloss Blending & Color Utilities Demo\n");

    // Demo 1: 1D blending between colors
    println!("📊 1D Color Blending:");
    let red = Color("#ff0000".to_string());
    let blue = Color("#0000ff".to_string());
    let gradient_1d = blend_1d(10, vec![red.clone(), blue.clone()]);

    for (i, color) in gradient_1d.iter().enumerate() {
        let style = Style::new().background(color.clone());
        print!("{}", style.render(&format!(" {:2} ", i)));
    }
    println!("\n   Red → Blue (10 steps)\n");

    // Demo 2: 2D blending
    println!("🌈 2D Color Blending (3x3 grid, 45° angle):");
    let green = Color("#00ff00".to_string());
    let yellow = Color("#ffff00".to_string());
    let gradient_2d = blend_2d(3, 3, 45.0, vec![green.clone(), yellow.clone()]);

    for y in 0..3 {
        for x in 0..3 {
            let color = &gradient_2d[y * 3 + x];
            let style = Style::new().background(color.clone());
            print!("{}", style.render("  "));
        }
        println!();
    }
    println!("   Green → Yellow (diagonal gradient)\n");

    // Demo 3: Color utilities
    println!("🛠️  Color Utilities:");

    // Lighten/Darken
    let base_color = Color("#666666".to_string());
    let lighter = lighten(&base_color, 0.4);
    let darker = darken(&base_color, 0.4);

    let dark_style = Style::new().background(darker);
    let base_style = Style::new().background(base_color.clone());
    let light_style = Style::new().background(lighter);

    println!("   Lighter/Darker:");
    print!("   {}", dark_style.render("  DARK  "));
    print!("{}", base_style.render("  BASE  "));
    print!("{}", light_style.render(" LIGHT "));
    println!("\n");

    // Alpha blending
    let semi_red = alpha(&red, 0.7);
    let semi_style = Style::new().background(semi_red);
    println!("   Alpha (70% opacity):");
    println!("   {}", semi_style.render("  SEMI-TRANSPARENT  "));
    println!();

    // Complementary colors
    let orange = Color("#ff8800".to_string());
    let comp = complementary(&orange);
    let orange_style = Style::new().background(orange);
    let comp_style = Style::new().background(comp);

    println!("   Complementary colors:");
    print!("   {}", orange_style.render(" ORANGE "));
    print!("{}", comp_style.render(" COMPLEMENT "));
    println!("\n");

    // Demo 4: Complex multi-color blend
    println!("🌟 Multi-color 1D Blend:");
    let colors = vec![
        Color("#ff0000".to_string()), // Red
        Color("#ff8800".to_string()), // Orange
        Color("#ffff00".to_string()), // Yellow
        Color("#00ff00".to_string()), // Green
        Color("#0088ff".to_string()), // Blue
        Color("#8800ff".to_string()), // Purple
    ];

    let rainbow = blend_1d(20, colors);
    for color in rainbow.iter() {
        let style = Style::new().background(color.clone());
        print!("{}", style.render(" "));
    }
    println!("\n   Rainbow gradient (20 steps)\n");

    println!("✨ Demo complete! The blending functions use CIELAB color space for perceptually uniform gradients.");
}
