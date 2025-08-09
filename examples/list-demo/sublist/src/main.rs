use lipgloss::{
    color::{
        ACCENT_PRIMARY, ACCENT_SECONDARY, LIST_ITEM_PRIMARY, LIST_ITEM_SECONDARY, STATUS_SUCCESS,
        TEXT_MUTED, TEXT_PRIMARY, TEXT_SUBTLE,
    },
    Style,
};
use lipgloss::{CENTER, RIGHT};
use lipgloss_list::{dash, List};
use lipgloss_table::{Table, HEADER_ROW};
use lipgloss_tree::Children;

fn checklist_enumerator(_items: &dyn Children, index: usize) -> String {
    match index {
        1 | 2 | 4 => "✓".to_string(),
        _ => "•".to_string(),
    }
}

fn checklist_enum_style(_items: &dyn Children, index: usize) -> Style {
    match index {
        1 | 2 | 4 => Style::new().foreground(STATUS_SUCCESS).padding_right(1),
        _ => Style::new().padding_right(1),
    }
}

fn checklist_item_style(_items: &dyn Children, index: usize) -> Style {
    match index {
        1 | 2 | 4 => Style::new().strikethrough(true).foreground(TEXT_SUBTLE),
        _ => Style::new(),
    }
}

const SELECTED: usize = 1;

fn documents_enumerator(_items: &dyn Children, i: usize) -> String {
    if i == SELECTED {
        "│\n│".to_string()
    } else {
        " ".to_string()
    }
}

fn documents_item_style(_items: &dyn Children, i: usize) -> Style {
    let base_style = Style::new().margin_bottom(1).margin_left(1);

    if SELECTED == i {
        base_style.foreground(ACCENT_PRIMARY)
    } else {
        base_style.foreground(TEXT_MUTED)
    }
}

fn documents_enum_style(_items: &dyn Children, i: usize) -> Style {
    if SELECTED == i {
        Style::new().foreground(ACCENT_PRIMARY)
    } else {
        Style::new().foreground(TEXT_MUTED)
    }
}

fn main() {
    let purple = Style::new().foreground(LIST_ITEM_SECONDARY).margin_right(1);

    let pink = Style::new().foreground(LIST_ITEM_PRIMARY).margin_right(1);

    let faint = Style::new().faint(true);

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
                        .item("Claire’s Boutique")
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
                .item(
                    // History box
                    &(Style::new()
                        .bold(true)
                        .foreground(TEXT_PRIMARY)
                        .background(ACCENT_SECONDARY)
                        .align_horizontal(CENTER)
                        .align_vertical(CENTER)
                        .padding(1, 3, 1, 3)
                        .margin(0, 1, 1, 1)
                        .width(40)
                        .render(
                            "Medieval quince preserves, which went by the French name cotignac, produced in a clear version and a fruit pulp version, began to lose their medieval seasoning of spices in the 16th century. In the 17th century, La Varenne provided recipes for both thick and clear cotignac.",
                        ))
                )
                .item(
                    // Small table
                    {
                        let label_style = Style::new().foreground(ACCENT_SECONDARY);
                        let mut t = Table::new()
                            .border(lipgloss::normal_border())
                            .border_style(label_style.margin_right(0))
                            .width(30)
                            .headers(vec!["ITEM", "QUANTITY"]) 
                            .rows(vec![
                                vec!["Apple", "6"],
                                vec!["Banana", "10"],
                                vec!["Orange", "2"],
                                vec!["Strawberry", "12"],
                            ])
                            .style_func_boxed(|row: i32, col: usize| {
                                let mut style = Style::new();
                                if col == 0 { style = style.align_horizontal(CENTER); }
                                else { style = style.align_horizontal(RIGHT).padding_right(2); }
                                if row == HEADER_ROW { return style.bold(true).align_horizontal(CENTER).padding_right(0); }
                                style.faint(true)
                            });
                        &t.render()
                    },
                )
                .item("Documents")
                .item_list(
                    List::new()
                        .enumerator(documents_enumerator)
                        .item_style_func(documents_item_style)
                        .enumerator_style_func(documents_enum_style)
                        .item(&format!("{}\n{}", "Foo Document", faint.render("1 day ago")))
                        .item(&format!("{}\n{}", "Bar Document", faint.render("2 days ago")))
                        .item(&format!("{}\n{}", "Baz Document", faint.render("10 minutes ago")))
                        .item(&format!("{}\n{}", "Qux Document", faint.render("1 month ago"))),
                )
                .item("EOF")
            )
            .item("go get github.com/charmbracelet/lipgloss/list\n")
            .item("See ya later")
        .item("xoxo, Charm_™");

    println!("{}", l);
}
