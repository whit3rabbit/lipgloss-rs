use lipgloss::{Color, Style};
use lipgloss_list::{dash, List};
use lipgloss_tree::Children;

fn checklist_enumerator(_items: &dyn Children, index: usize) -> String {
    match index {
        1 | 2 | 4 => "✓".to_string(),
        _ => "•".to_string(),
    }
}

fn checklist_enum_style(_items: &dyn Children, index: usize) -> Style {
    let special = Color::from("#73F59F");
    match index {
        1 | 2 | 4 => Style::new().foreground(special).padding_right(1),
        _ => Style::new().padding_right(1),
    }
}

fn checklist_item_style(_items: &dyn Children, index: usize) -> Style {
    let dim_color = Color::from("#696969");
    match index {
        1 | 2 | 4 => Style::new().strikethrough(true).foreground(dim_color),
        _ => Style::new(),
    }
}

fn main() {
    let purple = Style::new().foreground(Color::from("99")).margin_right(1);

    let pink = Style::new().foreground(Color::from("212")).margin_right(1);

    let l = List::new()
        .enumerator_style(purple.clone())
        .item("Lip Gloss")
        .item("Blush")
        .item("Eye Shadow")
        .item("Mascara")
        .item("Foundation")
        .item_list(
            List::new()
                .enumerator_style(pink)
                .item("Citrus Fruits to Try")
                .item_list(
                    List::new()
                        .item_style_func(checklist_item_style)
                        .enumerator_style_func(checklist_enum_style)
                        .enumerator(checklist_enumerator)
                        .item("Grapefruit")
                        .item("Yuzu")
                        .item("Citron")
                        .item("Kumquat")
                        .item("Pomelo"),
                )
                .item("Actual Lip Gloss Vendors")
                .item_list(
                    List::new()
                        .item_style_func(checklist_item_style)
                        .enumerator_style_func(checklist_enum_style)
                        .enumerator(checklist_enumerator)
                        .item("Glossier")
                        .item("Claire's Boutique")
                        .item("Nyx")
                        .item("Mac")
                        .item("Milk")
                        .item_list(
                            List::new()
                                .enumerator_style(purple)
                                .enumerator(dash)
                                .item("Lip Gloss")
                                .item("Lip Gloss")
                                .item("Lip Gloss")
                                .item("Lip Gloss")
                                .item("Style Definitions for Nice Terminal Layouts"),
                        ),
                )
                .item("List"),
        )
        .item("xoxo, Charm_™");

    println!("{}", l);
}
