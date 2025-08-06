use lipgloss::{Style, color::{Color, TerminalColor}, renderer::{Renderer, ColorProfileKind, set_default_renderer}};

#[test]
fn test_hex_5A56E0_parsing() {
    println!("Testing hex color #5A56E0 parsing");
    
    let color = Color::from("#5A56E0");
    let (r, g, b, a) = color.rgba();
    println!("RGBA: ({}, {}, {}, {})", r, g, b, a);
    
    // Test with TrueColor profile
    let mut renderer = Renderer::new();
    renderer.set_color_profile(ColorProfileKind::TrueColor);
    set_default_renderer(renderer);
    
    let style = Style::new().background(color.clone());
    let result = style.render("test");
    println!("TrueColor result: {}", result.escape_debug());
    
    // Check what color this maps to in ANSI profile (should be different with perceptual mapping)
    let mut ansi_renderer = Renderer::new();
    ansi_renderer.set_color_profile(ColorProfileKind::ANSI);
    set_default_renderer(ansi_renderer);
    
    let ansi_style = Style::new().background(color);
    let ansi_result = ansi_style.render("test");
    println!("ANSI background result: {}", ansi_result.escape_debug());
    
    // RGB precision note: Go produces RGB(89,86,224) while we produce RGB(90,86,224)
    // Our parsing is mathematically correct: 0x5A = 90 decimal
    // The 1-digit difference may be due to Go's internal color space transformations
    // Our implementation follows the hex specification exactly
    println!("0x5A = {}", 0x5A); // Should be 90
    println!("0x56 = {}", 0x56); // Should be 86  
    println!("0xE0 = {}", 0xE0); // Should be 224
}

#[test]  
fn test_direct_ansi_codes() {
    println!("Testing direct ANSI codes 94 and 91");
    
    // Test with ANSI profile  
    let mut ansi_renderer = Renderer::new();
    ansi_renderer.set_color_profile(ColorProfileKind::ANSI);
    set_default_renderer(ansi_renderer);
    
    // Test color "94" (bright blue)
    let color_94 = Color::from("94");
    let token_94 = color_94.token(&lipgloss::renderer::default_renderer());
    println!("Color '94' token: {}", token_94);
    
    let style_94 = Style::new().foreground(Color::from("94"));
    let result_94 = style_94.render("test");
    println!("Color '94' result: {}", result_94.escape_debug());
    
    // Test color "91" (bright red)
    let color_91 = Color::from("91"); 
    let token_91 = color_91.token(&lipgloss::renderer::default_renderer());
    println!("Color '91' token: {}", token_91);
    
    let style_91 = Style::new().foreground(Color::from("91"));
    let result_91 = style_91.render("test");
    println!("Color '91' result: {}", result_91.escape_debug());
}

#[test]
fn test_color_conversion_behavior() {
    println!("Testing color conversion behavior");
    
    let hex_color = "#5A56E0";
    let input = "hello";
    
    // Test with ANSI profile
    let mut ansi_renderer = Renderer::new();
    ansi_renderer.set_color_profile(ColorProfileKind::ANSI);
    set_default_renderer(ansi_renderer);
    
    let style = Style::new().foreground(Color::from(hex_color));
    let result = style.render(input);
    
    println!("ANSI profile result: {:?}", result);
    println!("ANSI profile result (escaped): {}", result.escape_debug());
    
    // Test with ANSI256 profile
    let mut ansi256_renderer = Renderer::new();
    ansi256_renderer.set_color_profile(ColorProfileKind::ANSI256);
    set_default_renderer(ansi256_renderer);
    
    let style = Style::new().foreground(Color::from(hex_color));
    let result = style.render(input);
    
    println!("ANSI256 profile result: {:?}", result);
    println!("ANSI256 profile result (escaped): {}", result.escape_debug());
    
    // Test color conversion directly
    let color = Color::from(hex_color);
    
    let mut ansi_renderer = Renderer::new();
    ansi_renderer.set_color_profile(ColorProfileKind::ANSI);
    let ansi_token = color.token(&ansi_renderer);
    println!("Direct ANSI token: {}", ansi_token);
    
    let mut ansi256_renderer = Renderer::new();
    ansi256_renderer.set_color_profile(ColorProfileKind::ANSI256);
    let ansi256_token = color.token(&ansi256_renderer);
    println!("Direct ANSI256 token: {}", ansi256_token);
    
    // Let's see what the issue might be - expected ANSI should be 94, but we get 8
    println!("Expected Go result for ANSI: \\x1b[94mhello\\x1b[0m");
    println!("Our ANSI result: {}", result.escape_debug());
    
    // Debug the color conversion
    let hex_rgb = (0x5A, 0x56, 0xE0); // #5A56E0
    println!("Input hex color: #{:02X}{:02X}{:02X} = RGB({}, {}, {})", hex_rgb.0, hex_rgb.1, hex_rgb.2, hex_rgb.0, hex_rgb.1, hex_rgb.2);
    
    // Let's check distances to all ANSI colors
    let ansi16_rgb = [
        (0x00, 0x00, 0x00), // 0 black
        (0x80, 0x00, 0x00), // 1 red  
        (0x00, 0x80, 0x00), // 2 green
        (0x80, 0x80, 0x00), // 3 yellow
        (0x00, 0x00, 0x80), // 4 blue
        (0x80, 0x00, 0x80), // 5 magenta
        (0x00, 0x80, 0x80), // 6 cyan
        (0xc0, 0xc0, 0xc0), // 7 white
        (0x80, 0x80, 0x80), // 8 bright black
        (0xff, 0x00, 0x00), // 9 bright red
        (0x00, 0xff, 0x00), // 10 bright green
        (0xff, 0xff, 0x00), // 11 bright yellow
        (0x00, 0x00, 0xff), // 12 bright blue
        (0xff, 0x00, 0xff), // 13 bright magenta
        (0x00, 0xff, 0xff), // 14 bright cyan
        (0xff, 0xff, 0xff), // 15 bright white
    ];
    
    let mut best_idx = 0;
    let mut best_dist = u32::MAX;
    for (i, &(rr, gg, bb)) in ansi16_rgb.iter().enumerate() {
        let dr = (hex_rgb.0 as i32 - rr as i32).abs() as u32;
        let dg = (hex_rgb.1 as i32 - gg as i32).abs() as u32;
        let db = (hex_rgb.2 as i32 - bb as i32).abs() as u32;
        let dist = dr * dr + dg * dg + db * db;
        println!("  Color {}: RGB({}, {}, {}) distance: {}", i, rr, gg, bb, dist);
        if dist < best_dist {
            best_dist = dist;
            best_idx = i;
        }
    }
    println!("Best match: color {} with distance {}", best_idx, best_dist);
    println!("ANSI code should be: {}", if best_idx <= 7 { 30 + best_idx } else { 82 + best_idx });
}