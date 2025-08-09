// This example demonstrates various Lip Gloss style and layout features.

use lipgloss::security::safe_str_repeat;
use lipgloss::{
    blend_1d, join_horizontal, join_vertical, normal_border, place, rounded_border,
    whitespace::{with_whitespace_chars, with_whitespace_foreground},
    width, AdaptiveColor, Color, Style, BOTTOM, CENTER, LEFT, RIGHT, TOP,
};
use std::cmp::max;
use std::env;

const WIDTH: i32 = 96;
const COLUMN_WIDTH: i32 = 30;

// Color grid blends colors from 4 corner quadrants, into a box region using native blending
fn color_grid(
    x_steps: usize,
    y_steps: usize,
    corners: (&str, &str, &str, &str),
) -> Vec<Vec<Color>> {
    let (top_left, top_right, bottom_left, bottom_right) = corners;

    let left_colors = blend_1d(
        y_steps,
        vec![Color::from(top_left), Color::from(bottom_left)],
    );
    let right_colors = blend_1d(
        y_steps,
        vec![Color::from(top_right), Color::from(bottom_right)],
    );

    let mut grid = Vec::with_capacity(y_steps);
    for y in 0..y_steps {
        let row_colors = blend_1d(
            x_steps,
            vec![left_colors[y].clone(), right_colors[y].clone()],
        );
        grid.push(row_colors);
    }

    grid
}

fn main() {
    let mut doc = String::new();

    // Style definitions
    let normal = Color::from("#EEEEEE");
    let subtle = AdaptiveColor {
        Light: "#D9DCCF",
        Dark: "#383838",
    };
    let highlight = AdaptiveColor {
        Light: "#874BFD",
        Dark: "#7D56F4",
    };
    let special = AdaptiveColor {
        Light: "#43BF6D",
        Dark: "#73F59F",
    };
    let blends = blend_1d(50, vec![Color::from("#F25D94"), Color::from("#EDFF82")]);

    let base = Style::new().foreground(normal);

    let divider = Style::new()
        .set_string("•")
        .padding(0, 1, 0, 1)
        .foreground(subtle.clone())
        .render("");

    let url = |s: &str| Style::new().foreground(special.clone()).render(s);

    // Tabs
    let mut active_tab_border = rounded_border();
    active_tab_border.bottom = " ";
    active_tab_border.bottom_left = "┘";
    active_tab_border.bottom_right = "└";

    let mut tab_border = rounded_border();
    tab_border.bottom = "─";
    tab_border.bottom_left = "┴";
    tab_border.bottom_right = "┴";

    let tab = Style::new()
        .border_style(tab_border)
        .border_foreground(highlight.clone())
        .padding(0, 1, 0, 1)
        .border_top(true)
        .border_left(true)
        .border_right(true)
        .border_bottom(true);

    // Active tab: keep bottom border enabled but draw it as spaces (Go parity)
    // This preserves equal height across tabs while visually hiding the bottom edge.
    let active_tab = tab.clone().border_style(active_tab_border);

    // Title
    let title_style = Style::new()
        .margin_left(1)
        .margin_right(9)
        .padding(0, 1, 0, 1)
        .italic(true)
        .foreground(Color::from("#FFFFFF"))
        .set_string("Lip Gloss");

    let desc_style = base.clone().margin_top(1);

    let info_style = base.clone();

    // Dialog
    let dialog_box_style = Style::new()
        .border_style(rounded_border())
        .border_foreground(Color::from("#874BFD"))
        .padding(1, 0, 1, 0)
        .border_top(true)
        .border_left(true)
        .border_right(true)
        .border_bottom(true);

    let button_style = Style::new()
        .foreground(Color::from("#FFF7DB"))
        .background(Color::from("#888B7E"))
        .padding(0, 3, 0, 3)
        .margin_top(1);

    // Alternatively, you can use the shorthand for better readability:
    // .padding_shorthand(&[0, 3])

    let active_button_style = button_style
        .clone()
        .foreground(Color::from("#FFF7DB"))
        .background(Color::from("#F25D94"))
        .margin_right(2)
        .underline(true);

    // List
    let list = Style::new()
        .border_style(normal_border())
        .border_top(false)
        .border_right(true)
        .border_bottom(false)
        .border_left(false)
        .border_foreground(subtle.clone())
        .padding_left(0)
        .margin_right(2)
        .height(8)
        .width(COLUMN_WIDTH + 1);

    let list_header = |title: &str| -> String {
        base.clone()
            .border_style(normal_border())
            .border_top(false)
            .border_left(false)
            .border_right(false)
            .border_bottom(true)
            .border_foreground(subtle.clone())
            .margin_right(2)
            .render(title)
    };
    let list_item = base.clone().padding_left(2);

    let check_mark = Style::new()
        .set_string("✓")
        .foreground(special.clone())
        .padding_right(1)
        .render("");

    let list_done = |s: &str| -> String {
        format!(
            "{}{}",
            check_mark,
            Style::new()
                .strikethrough(true)
                .foreground(AdaptiveColor {
                    Light: "#969B86",
                    Dark: "#696969",
                })
                .render(s)
        )
    };

    // Paragraphs/History
    let history_style = Style::new()
        .align_horizontal(LEFT)
        .foreground(Color::from("#FAFAFA"))
        .background(Color::from("#7D56F4")) // Direct purple color
        .margin(1, 3, 0, 0) // Match Go: top=1, right=3, bottom=0, left=0
        .padding(1, 2, 1, 2)
        .height(19)
        .width(COLUMN_WIDTH)
        .color_whitespace(true);

    // Status Bar
    let status_nugget = Style::new()
        .foreground(Color::from("#FFFDF5"))
        .padding(0, 1, 0, 1);

    let status_bar_style = Style::new().background(AdaptiveColor {
        Light: "#D9DCCF",
        Dark: "#353533",
    });

    let status_style = status_nugget.clone().background(Color::from("#FF5F87"));

    let encoding_style = status_nugget.clone().background(Color::from("#A550DF"));

    let fish_cake_style = status_nugget.background(Color::from("#6124DF"));

    // Page
    let doc_style = Style::new().padding(1, 2, 1, 2);

    // Tabs
    {
        // Use join_horizontal to correctly stitch multi-line bordered tabs.
        // Align on TOP so all tabs share the same top edge; the active tab
        // simply omits its bottom border without shifting downward.
        let row = join_horizontal(
            TOP,
            &[
                &active_tab.render("Lip Gloss"),
                &tab.render("Blush"),
                &tab.render("Eye Shadow"),
                &tab.render("Mascara"),
                &tab.render("Foundation"),
            ],
        );
        let gap_width = max(0, WIDTH - width(&row) as i32 - 2);
        // Match Go: use a tabGap style (no top/left/right) so only the bottom border draws
        // the trailing line at the correct vertical position.
        let tab_gap = tab
            .clone()
            .border_top(false)
            .border_left(false)
            .border_right(false);
        let gap = tab_gap.render(&safe_str_repeat(" ", gap_width as usize));
        let row = join_horizontal(BOTTOM, &[&row, &gap]);
        doc.push_str(&row);
        doc.push_str("\n\n");
    }

    // Title
    {
        // Stepped title using the same bilinear color grid as the Go demo
        // Text remains off-white per title_style (to match Go)
        let colors = color_grid(1, 5, ("#F25D94", "#EDFF82", "#643AFF", "#14F9D5"));
        let mut title_parts = Vec::new();

        for (i, v) in colors.iter().enumerate() {
            let offset = 2; // step width
                            // explicit empty spacer to guarantee blank columns between steps
            let left_spacer = Style::new().width((i * offset) as i32).render("");
            let c = v[0].clone();
            let colored = title_style
                .clone()
                .background(c)
                .foreground(Color::from("#FFFFFF"))
                .render("");
            let stepped = join_horizontal(TOP, &[&left_spacer, &colored]);
            title_parts.push(stepped);
        }

        let title = title_parts.join("\n");

        let desc_text = "Style Definitions for Nice Terminal Layouts";
        let desc_underline = Style::new()
            .foreground(subtle.clone())
            .render(&safe_str_repeat("─", desc_text.len()));

        // Changed to Rust URL from golang url
        let desc = join_vertical(
            LEFT,
            &[
                &desc_style.render(desc_text),
                &desc_underline,
                &info_style.render(&format!(
                    "Based on Charm{}{}",
                    divider,
                    url("https://github.com/whit3rabbit/lipgloss-rs")
                )),
            ],
        );

        // Add explicit spacer between the stepped title and the right-hand description
        let inter_spacer = Style::new().width(4).render("");
        let row = join_horizontal(TOP, &[&title, &inter_spacer, &desc]);
        doc.push_str(&row);
        doc.push_str("\n\n");
    }

    // Dialog
    {
        let ok_button = active_button_style.render("Yes");
        let cancel_button = button_style.render("Maybe");

        let question = Style::new()
            .width(50)
            .align_horizontal(CENTER)
            .render(&rainbow(
                Style::new(),
                "Are you sure you want to eat marmalade?",
                &blends,
            ));

        // Match Go: no manual spacer, rely on active button's right margin
        let buttons = join_horizontal(TOP, &[&ok_button, &cancel_button]);
        // Match Go: vertically center and let JoinVertical(CENTER) handle horizontal centering
        let ui = join_vertical(CENTER, &[&question, &buttons]);

        // Let Place do the centering and background fill. Do not pre-set dialog width.
        let dialog = dialog_box_style.render(&ui);

        let placed = place(
            WIDTH,
            9,
            CENTER,
            CENTER,
            &dialog,
            &[
                with_whitespace_chars("猫咪"),
                with_whitespace_foreground(subtle.clone()),
            ],
        );

        doc.push_str(&placed);
        doc.push_str("\n\n");
    }

    // Color grid
    let colors = {
        let colors = color_grid(14, 8, ("#F25D94", "#EDFF82", "#643AFF", "#14F9D5"));
        let mut b = String::new();
        for (i, x) in colors.iter().enumerate() {
            for y in x {
                let s = Style::new()
                    .set_string("  ")
                    .background(y.clone()) // use .clone() as we are iterating over a reference
                    .color_whitespace(true) // Enable background colors on whitespace
                    .render("");
                b.push_str(&s);
            }
            if i < colors.len() - 1 {
                b.push('\n'); // Only add newlines between rows
            }
        }
        // No margin needed on color grid since lists have margin_right
        b
    };

    let left_list_content = join_vertical(
        LEFT,
        &[
            &list_header("Citrus Fruits to Try"),
            &list_done("Grapefruit"),
            &list_done("Yuzu"),
            &list_item.render("Citron"),
            &list_item.render("Kumquat"),
            &list_item.render("Pomelo"),
        ],
    );

    let right_list_content = join_vertical(
        LEFT,
        &[
            &list_header("Actual Lip Gloss Vendors"),
            &list_item.render("Glossier"),
            &list_item.render("Claire's Boutique"),
            &list_done("Nyx"),
            &list_item.render("Mac"),
            &list_done("Milk"),
        ],
    );

    // Match Go version: first list uses default width (COLUMN_WIDTH + 1), second list uses COLUMN_WIDTH
    let left_list_box = list.render(&left_list_content);
    let right_list_box = list.clone().width(COLUMN_WIDTH).render(&right_list_content);
    let lists = join_horizontal(TOP, &[&left_list_box, &right_list_box]);

    doc.push_str(&join_horizontal(TOP, &[&lists, &colors]));
    doc.push_str("\n\n");

    // Marmalade history
    {
        const HISTORY_A: &str = r#"The Romans learned from the Greeks that quinces slowly cooked with honey would "set" when cool. The Apicius gives a recipe for preserving whole quinces, stems and leaves attached, in a bath of honey diluted with defrutum: Roman marmalade. Preserves of quince and lemon appear (along with rose, apple, plum and pear) in the Book of ceremonies of the Byzantine Emperor Constantine VII Porphyrogennetos."#;
        const HISTORY_B: &str = r#"Medieval quince preserves, which went by the French name cotignac, produced in a clear version and a fruit pulp version, began to lose their medieval seasoning of spices in the 16th century. In the 17th century, La Varenne provided recipes for both thick and clear cotignac."#;
        const HISTORY_C: &str = r#"In 1524, Henry VIII, King of England, received a "box of marmalade" from Mr. Hull of Exeter. This was probably marmelada, a solid quince paste from Portugal, still made and sold in southern Europe today. It became a favourite treat of Anne Boleyn and her ladies in waiting."#;

        let marmalade_section = join_horizontal(
            TOP,
            &[
                &history_style
                    .clone()
                    .align_horizontal(RIGHT)
                    .render(HISTORY_A),
                &history_style
                    .clone()
                    .align_horizontal(CENTER)
                    .render(HISTORY_B),
                &history_style
                    .clone()
                    .margin_right(0) // Remove right margin for last element
                    .render(HISTORY_C),
            ],
        );

        doc.push_str(&marmalade_section);
        doc.push_str("\n\n");
    }

    // Status bar
    {
        // Create individual colored segments
        let status_segment = status_style.render("STATUS");
        let utf8_segment = encoding_style.render("UTF-8");
        let fish_segment = fish_cake_style.render("🍥 Fish Cake");

        // Calculate the width needed for the middle "Ravishing" section
        let segments_width = width(&status_segment) + width(&utf8_segment) + width(&fish_segment);
        // Add an explicit spacer between "STATUS" and the middle section
        let status_gap = status_bar_style.clone().width(2).render("");
        let middle_width = WIDTH - segments_width as i32 - width(&status_gap) as i32;

        // Create the middle section with the main background
        let middle_section = status_bar_style
            .clone()
            .width(middle_width)
            .render("Ravishing");

        // Join all segments together (with spacer after STATUS)
        let bar = join_horizontal(
            TOP,
            &[
                &status_segment,
                &status_gap,
                &middle_section,
                &utf8_segment,
                &fish_segment,
            ],
        );

        doc.push_str(&bar);
    }

    let final_doc_style = if let Ok(w) = env::var("COLUMNS") {
        if let Ok(width_val) = w.parse::<i32>() {
            doc_style.max_width(width_val)
        } else {
            doc_style
        }
    } else {
        doc_style
    };

    // Okay, let's print it
    println!("{}", final_doc_style.render(&doc));
}

fn rainbow(base: Style, s: &str, colors: &[Color]) -> String {
    let mut result = String::new();
    for (i, ch) in s.chars().enumerate() {
        let color = &colors[i % colors.len()];
        let styled_char = base
            .clone()
            .foreground(color.clone())
            .render(&ch.to_string());
        result.push_str(&styled_char);
    }
    result
}
