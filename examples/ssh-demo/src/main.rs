use crossterm::terminal;
use lipgloss::color::AdaptiveColor;
use lipgloss::renderer::Renderer;
use lipgloss::whitespace::{with_whitespace_chars, with_whitespace_foreground};
use lipgloss::{height, place, Style, CENTER};

// Available styles, built against a specific Renderer (to respect profile/background)
struct Styles {
    bold: Style,
    faint: Style,
    italic: Style,
    underline: Style,
    strikethrough: Style,
    red: Style,
    green: Style,
    yellow: Style,
    blue: Style,
    magenta: Style,
    cyan: Style,
    gray: Style,
}

fn make_styles(_r: &Renderer) -> Styles {
    // Style construction uses Renderer internally during render-time; styles are
    // plain values here. We keep this aligned with the Go example semantics.
    Styles {
        bold: Style::new().set_string("bold").bold(true),
        faint: Style::new().set_string("faint").faint(true),
        italic: Style::new().set_string("italic").italic(true),
        underline: Style::new().set_string("underline").underline(true),
        strikethrough: Style::new().set_string("strikethrough").strikethrough(true),
        red: Style::new()
            .set_string("red")
            .foreground(lipgloss::Color("#E88388".into())),
        green: Style::new()
            .set_string("green")
            .foreground(lipgloss::Color("#A8CC8C".into())),
        yellow: Style::new()
            .set_string("yellow")
            .foreground(lipgloss::Color("#DBAB79".into())),
        blue: Style::new()
            .set_string("blue")
            .foreground(lipgloss::Color("#71BEF2".into())),
        magenta: Style::new()
            .set_string("magenta")
            .foreground(lipgloss::Color("#D290E4".into())),
        cyan: Style::new()
            .set_string("cyan")
            .foreground(lipgloss::Color("#66C2CD".into())),
        gray: Style::new()
            .set_string("gray")
            .foreground(lipgloss::Color("#B9BFCA".into())),
    }
}

fn main() {
    // Create a per-run renderer (auto-detects color profile and background)
    let renderer = Renderer::new();

    // Build styles against this renderer
    let styles = make_styles(&renderer);

    // Compose demo string mirroring the Go example
    let mut out = String::new();
    out.push_str("\n\n");
    out.push_str(&styles.bold.render("bold"));
    out.push(' ');
    out.push_str(&styles.faint.render("faint"));
    out.push(' ');
    out.push_str(&styles.italic.render("italic"));
    out.push(' ');
    out.push_str(&styles.underline.render("underline"));
    out.push(' ');
    out.push_str(&styles.strikethrough.render("strikethrough"));

    out.push('\n');
    out.push_str(&styles.red.render("red"));
    out.push(' ');
    out.push_str(&styles.green.render("green"));
    out.push(' ');
    out.push_str(&styles.yellow.render("yellow"));
    out.push(' ');
    out.push_str(&styles.blue.render("blue"));
    out.push(' ');
    out.push_str(&styles.magenta.render("magenta"));
    out.push(' ');
    out.push_str(&styles.cyan.render("cyan"));
    out.push(' ');
    out.push_str(&styles.gray.render("gray"));

    out.push('\n');
    out.push_str(&styles.red.render("red"));
    out.push(' ');
    out.push_str(&styles.green.render("green"));
    out.push(' ');
    out.push_str(&styles.yellow.render("yellow"));
    out.push(' ');
    out.push_str(&styles.blue.render("blue"));
    out.push(' ');
    out.push_str(&styles.magenta.render("magenta"));
    out.push(' ');
    out.push_str(&styles.cyan.render("cyan"));
    out.push(' ');
    out.push_str(&styles.gray.render("gray"));
    out.push_str("\n\n");

    let dark = renderer.has_dark_background();
    out.push_str(
        &Style::new()
            .unset_string()
            .bold(true)
            .render("Has dark background? "),
    );
    out.push_str(&format!("{}\n\n", dark));

    // Center the composed block within the terminal width using whitespace
    let cols = match terminal::size() {
        Ok((c, _r)) => c as i32,
        Err(_) => 80,
    };
    let h = height(&out) as i32;
    let block = place(
        cols,
        h,
        CENTER,
        CENTER,
        &out,
        &[
            with_whitespace_chars("/"),
            // Use AdaptiveColor for whitespace like the Go example (Light: 250, Dark: 236)
            with_whitespace_foreground(AdaptiveColor {
                Light: "250",
                Dark: "236",
            }),
        ],
    );

    println!("{}", block);
}
