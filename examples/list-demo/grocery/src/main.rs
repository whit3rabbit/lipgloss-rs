use lipgloss::{
    color::{STATUS_SUCCESS, TEXT_MUTED, TEXT_PRIMARY},
    Style,
};
use lipgloss_list::List;
use lipgloss_tree::Children;

static PURCHASED: &[&str] = &[
    "Bananas",
    "Barley",
    "Cashews",
    "Coconut Milk",
    "Dill",
    "Eggs",
    "Fish Cake",
    "Leeks",
    "Papaya",
];

fn grocery_enumerator(items: &dyn Children, i: usize) -> String {
    if let Some(item) = items.at(i) {
        let value = item.value();
        for &p in PURCHASED {
            if value == p {
                return "✓".to_string();
            }
        }
    }
    "•".to_string()
}

fn enum_style_func(items: &dyn Children, i: usize) -> Style {
    let dim_enum_style = Style::new().foreground(TEXT_MUTED).margin_right(1);

    let highlighted_enum_style = Style::new().foreground(STATUS_SUCCESS).margin_right(1);

    if let Some(item) = items.at(i) {
        let value = item.value();
        for &p in PURCHASED {
            if value == p {
                return highlighted_enum_style;
            }
        }
    }
    dim_enum_style
}

fn item_style_func(items: &dyn Children, i: usize) -> Style {
    let item_style = Style::new().foreground(TEXT_PRIMARY);

    if let Some(item) = items.at(i) {
        let value = item.value();
        for &p in PURCHASED {
            if value == p {
                return item_style.strikethrough(true);
            }
        }
    }
    item_style
}

fn main() {
    let l = List::new()
        .items(vec![
            "Artichoke",
            "Baking Flour",
            "Bananas",
            "Barley",
            "Bean Sprouts",
            "Cashew Apple",
            "Cashews",
            "Coconut Milk",
            "Curry Paste",
            "Currywurst",
            "Dill",
            "Dragonfruit",
            "Dried Shrimp",
            "Eggs",
            "Fish Cake",
            "Furikake",
            "Jicama",
            "Kohlrabi",
            "Leeks",
            "Lentils",
            "Licorice Root",
        ])
        .enumerator(grocery_enumerator)
        .enumerator_style_func(enum_style_func)
        .item_style_func(item_style_func);

    println!("{}", l);
}
