use lipgloss::{Style, Color};

fn main() {
    println!("Testing background colors...");
    
    // Test 1: Simple background color
    let simple_style = Style::new()
        .background(Color::from("#7D56F4"))
        .foreground(Color::from("#FAFAFA"))
        .color_whitespace(true);
    
    println!("Test 1 - Simple: '{}'", simple_style.render("Hello World"));
    
    // Test 2: With padding
    let padded_style = Style::new()
        .background(Color::from("#7D56F4"))
        .foreground(Color::from("#FAFAFA"))
        .padding(1, 2, 1, 2)
        .color_whitespace(true);
    
    println!("Test 2 - Padded: '{}'", padded_style.render("Hello World"));
    
    // Test 3: With width
    let width_style = Style::new()
        .background(Color::from("#7D56F4"))
        .foreground(Color::from("#FAFAFA"))
        .width(20)
        .color_whitespace(true);
    
    println!("Test 3 - Width: '{}'", width_style.render("Hello World"));
    
    // Test 4: Multi-line like the purple boxes
    let multiline_style = Style::new()
        .background(Color::from("#7D56F4"))
        .foreground(Color::from("#FAFAFA"))
        .width(30)
        .height(10)
        .padding(1, 2, 1, 2)
        .color_whitespace(true);
    
    println!("Test 4 - Multi-line: '{}'", multiline_style.render("This is a longer text that should wrap and have a purple background like the history boxes"));
}
